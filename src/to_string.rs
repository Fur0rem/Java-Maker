#![allow(clippy::needless_return)]
#![allow(dead_code)]

use crate::parser::Command;


pub fn needed_imports() -> Vec<String> {
	return vec!["java.lang.StringBuilder".into(),"java.util.Arrays".into()];
}

pub fn create_to_string(command : &Command) -> String {

	let mut function = String::from("@Override\npublic String toString() {\n");
	function.push_str("StringBuilder sb = new StringBuilder();\n");
	function.push_str("sb.append(\"");
	function.push_str(&command.class_name);
	function.push_str(" : {\\n\");\n");

	for att in &command.attributes {

		let called_func = match &att.var_type {
			//nested arrays
			t if t.contains("[][]") => {
				"Arrays.deepToString("
			},
			//arrays
			t if t.contains("[]") || t.contains("<") => {
				"Arrays.toString("
			},
			//other types
			_ => {
				"String.valueOf("
			}
		};

		let qualifier = if att.is_static() {
				format!("{}.", &command.class_name)
			} else {
				"this.".into()
		};

		let end_of_func = if att.var_type.contains("[]") || att.var_type.contains("<") {
				".toArray())"
			} else {
				")"
		};

		let mut string_call = String::from(called_func);
		string_call.push_str(&qualifier);
		string_call.push_str(&att.var_name);
		string_call.push_str(end_of_func);
		
		function.push_str("sb.append(\"");
		function.push_str(&att.var_name);
		function.push_str(" = \");\n");
		function.push_str("sb.append(");
		function.push_str(&string_call);
		function.push_str(");\n");
		function.push_str("sb.append(\"\\n\");\n");
	}

	function.push_str("sb.append(\"}\");\n");

	function.push_str("return sb.toString();\n");
	function.push_str("}\n");
	return function;
}