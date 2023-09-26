#![allow(clippy::needless_return)]
#![allow(dead_code)]

use crate::attribute;
use crate::attribute::Attribute;


#[derive(Debug, Default)]
pub struct Options {
	pub getters : bool,
	pub setters : bool,
	pub counter : bool,
	pub to_string : bool,
	pub equals : bool,
	pub comments : bool,
}

#[derive(Debug)]
pub struct Command {
	pub class_name : String,
	pub attributes : Vec<Attribute>,
	pub options : Options,
}

//format de la commande : java_maker nomClasse { nomAttribut : typeAttribut @Visibility (extra1 | extra2), ... } --options

fn concatenate_command(command : &str) -> String {
	//removes multiple whitespaces in a row, newlines, tabs, and trims it
	let mut command = command.replace('\n', " ");
	
	//finds multiple whitespaces in a row and replaces them with a single whitespace
	let mut index = 0;
	while index < command.len() {
		if command[index..].starts_with("  ") {
			command = command[..index].to_string() + &command[index+1..];
		}
		else {
			index += 1;
		}
	}

	command = command.trim().to_string();

	return command;
}

fn split_command(command : &str) -> (String, String, String) {
	//split command into 3 parts : class name, attributes, options

	//find class name, split at first whitespace
	let index = command.find(' ').unwrap_or(command.len());
	let class_name = command[..index].trim().to_string();

	//find attributes, in between { and }
	let open_bracket = command.find('{').unwrap_or(command.len());
	let close_bracket = command.find('}').unwrap_or(command.len());
	let attributes = command[open_bracket+1..close_bracket].trim().to_string();

	//find options, after }
	let options = command[close_bracket+1..].trim().to_string();

	return (class_name, attributes, options);
}

fn parse_options(options : String) -> Options {
	
	//split the string at each "--"
	let options_vec : Vec<&str> = options.split("--").map(|s| s.trim()).collect();

	let mut options = Options::default();

	for o in &options_vec[1..] {
		match *o {
			"getters" => options.getters = true,
			"setters" => options.setters = true,
			"counter" => options.counter = true,
			"to_string" => options.to_string = true,
			"equals" => options.equals = true,
			"comments" => options.comments = true,
			_ => panic!("Unknown option : {}", o),
		}
	}

	return options;
}

pub fn parse_command(command : &str) -> Command {
	let command = concatenate_command(command);
	let (class_name, attributes, options) = split_command(&command);
	let attributes = attribute::parse_attributes(attributes);
	let options = parse_options(options);

	return Command {
		class_name,
		attributes,
		options,
	};
}