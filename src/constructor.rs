#![allow(clippy::needless_return)]

use crate::parser::Command;
use crate::visibility::Visibility;

pub fn create_constructor(command: &Command) -> String {
	let mut constructor = String::new();

	//visibility of the constructor
	constructor.push_str(Visibility::Public.to_string().as_str());
	constructor.push(' ');

	//name of the constructor
	constructor.push_str(&command.class_name);
	constructor.push('(');

	//parameters of the constructor
	for att in command.attributes.iter().filter(|a| !a.is_static()) {
		constructor.push_str(&att.var_type);
		constructor.push(' ');
		constructor.push_str(&att.var_name);
		constructor.push_str(", ");
	}

	//closing of the parameters
	constructor.truncate(constructor.len() - 2);
	constructor.push_str(") ");

	//opening of the constructor
	constructor.push_str("{\n");

	//body of the constructor
	for att in command.attributes.iter().filter(|a| !a.is_static()) {
		constructor.push_str("this.");
		constructor.push_str(&att.var_name);
		constructor.push_str(" = ");
		constructor.push_str(&att.var_name);
		constructor.push_str(";\n");
	}

	//if there's a counter
	if command.options.counter {
		constructor.push_str(&command.class_name);
		constructor.push_str(".compteur++;\n");
	}

	//closing of the constructor
	constructor.push_str("}\n");

	return constructor;
}
