#![allow(clippy::needless_return)]
#![allow(dead_code)]

use crate::parser::Command;

pub fn create_equals(command: &Command) -> String {
	//on suppose que c'est vérifié avant qu'il faut créer un equals

	let mut function = String::from("@Override\npublic boolean equals(Object o) {\n");
	function.push_str("if (this == o) {\nreturn true;\n}\n");
	function.push_str("if (o == null || this.getClass() != o.getClass()) {\nreturn false;\n}\n");
	let var_name = &command.class_name.to_lowercase()[0..2];
	function.push_str(&format!(
		"{} {} = ({}) o;\n",
		&command.class_name, var_name, &command.class_name
	));
	//on compare les attributs
	function.push_str("return ");
	for att in &command.attributes {
		if !att.is_static() {
			function.push_str(&format!(
				"(this.{} == {}.{}) && ",
				&att.var_name, var_name, &att.var_name
			));
		}
	}
	function.truncate(function.len() - 4);
	function.push_str(";\n}\n");

	return function;
}
