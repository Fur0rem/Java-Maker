#![allow(clippy::needless_return)]
#![allow(dead_code)]

use crate::parser::Command;

pub fn needed_imports() -> Vec<String> {
	return vec!["java.lang.StringBuilder".into(), "java.util.Arrays".into()];
}

pub fn create_to_string(command: &Command) -> String {
	let mut function = String::from("@Override\npublic String toString() {\n");
	function.push_str("StringBuilder sb = new StringBuilder();\n");
	function.push_str("sb.append(\"");
	function.push_str(&command.class_name);
	function.push_str(" : {\\n\");\n");

	for att in &command.attributes {
		let (called_func, qualifier, end_of_func) = match (&att.var_type, att.is_static()) {
			//nested arrays
			(t, _) if t.contains("[][]") => (
				"Arrays.deepToString(",
				format!("{}.", &command.class_name),
				".toArray())",
			),
			//arrays
			(t, _) if t.contains("[]") || t.contains('<') => (
				"Arrays.toString(",
				format!("{}.", &command.class_name),
				".toArray())",
			),
			//other types
			(_, true) => ("String.valueOf(", format!("{}.", &command.class_name), ")"),
			(_, false) => ("String.valueOf(", "this.".into(), ")"),
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
