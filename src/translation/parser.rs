use crate::translation::format::{fix, warnings};
use std::{
	path::{Path, PathBuf},
	str::FromStr,
};
use strum::IntoEnumIterator;

use crate::{
	tokens::{expr_type::ExprType, modifier::Modifier, variable::Variable, visibility::Visibility},
	JavaMakerError,
};

#[derive(Debug, Default)]
pub struct Options {
	pub getters: bool,
	pub setters: bool,
	pub counter: bool,
	pub to_string: bool,
	pub equals: bool,
	pub documentation: bool,
	pub warnings: bool,
	pub fix: bool,
}

#[derive(Debug)]
pub struct Command {
	pub class_name: String,
	pub attributes: Vec<Variable>,
	pub options: Options,
	pub path: PathBuf,
}

//format de la commande : java_maker nomClasse { nomAttribut : typeAttribut @Visibility (extra1 | extra2), ... } --options

/// Removes all whitespaces from a string
fn concatenate_command(command: &str) -> String {
	let mut command = command.to_string();
	command.retain(|c| !c.is_whitespace());
	return command;
}

fn split_command(command: &str) -> (String, String, String) {
	//split command into 3 parts : class name, attributes, options

	//find class name
	let index = command
		.find(|c| c == '{' || c == '-')
		.unwrap_or(command.len());
	let class_name = command[..index].to_string();

	//find attributes, in between { and }
	let open_bracket = command.find('{').unwrap_or(command.len());
	let close_bracket = command.find('}').unwrap_or(command.len());
	let attributes = command[open_bracket + 1..close_bracket].to_string();

	//find options, after }
	let options = command[close_bracket + 1..].to_string();

	return (class_name, attributes, options);
}

fn parse_options(options: String) -> Result<Options, JavaMakerError> {
	//split the string at each "--"
	let options_vec: Vec<&str> = options.split("--").map(|s| s.trim()).collect();

	let mut options = Options::default();

	for o in &options_vec[1..] {
		match *o {
			"getters" => options.getters = true,
			"setters" => options.setters = true,
			"counter" => options.counter = true,
			"to_string" => options.to_string = true,
			"equals" => options.equals = true,
			"docs" => options.documentation = true,
			"warnings" => options.warnings = true,
			"fix" => options.fix = true,
			_ => return Err(JavaMakerError::UnknownOption(o.to_string())),
		}
	}

	return Ok(options);
}

fn parse_attributes(attributes: &str) -> Result<Vec<Variable>, JavaMakerError> {
	let attributes: Vec<&str> = attributes.split(';').map(|s| s.trim()).collect();
	let mut parsed_attributes = Vec::new();

	for att in attributes {
		// Find the name : first non-alphanumeric character or '_'
		let name = att[..att
			.find(|c: char| !c.is_alphanumeric() && c != '_')
			.unwrap()]
			.trim()
			.to_string();

		// Find the type : first non-alphanumeric character or '_' or '<' or '>' or '[' or ']' or ',' after the :
		const TYPE_CHARS: [char; 6] = ['<', '>', '[', ']', ',', '_'];
		let double_point_index = att.find(':');
		let att_type = match double_point_index {
			Some(begin_index) => {
				// find first non-alphanumeric character or '_' or '<' or '>' or '[' or ']' or ',' after the ':'
				let mut index = begin_index + 1;
				let mut end_index = None;
				while let Some(c) = att[index..].chars().next() {
					if !TYPE_CHARS.contains(&c) && !c.is_alphanumeric() {
						end_index = Some(index);
						break;
					}
					index += 1;
				}
				ExprType::from_str(&att[begin_index + 1..end_index.unwrap_or(att.len())])?
			}
			None => ExprType::void(),
		};

		//find the visibility
		let at_symbol_index = att.find('@');
		let visibility = if let Some(index) = at_symbol_index {
			let mut end_index = None;
			for possible_visibility in Visibility::iter() {
				if att[index + 1..].starts_with(possible_visibility.to_string().as_str()) {
					end_index = Some(index + possible_visibility.to_string().len());
					break;
				}
			}
			if end_index.is_none() {
				return Err(JavaMakerError::UnknownVisibility(att[index..].to_string()));
			}
			Visibility::from_str(&att[index + 1..=end_index.unwrap()]).unwrap()
		} else {
			Visibility::default()
		};

		let modifier = match att.find('(') {
			Some(index) => {
				//find the other
				let other_index = att[index + 1..].find(')').unwrap() + index + 1;
				let modifier = &att[index + 1..other_index];
				// split the string at each "|"
				let modifiers_vec = modifier.split('|').collect();
				Modifier::from_strs(&visibility.to_string(), modifiers_vec)
			}
			None => Modifier::from_strs(&visibility.to_string(), vec![]),
		};

		let init = att
			.find('=')
			.map(|index| att[index + 1..].trim().to_string());

		let var = Variable::new(modifier, att_type, &name.clone(), init);
		parsed_attributes.push(var);
	}

	return Ok(parsed_attributes);
}

pub fn parse_command(command: &str, path: &Path) -> Result<Command, JavaMakerError> {
	let command = concatenate_command(command);
	let (class_name, attributes, options) = split_command(&command);
	let attributes = parse_attributes(&attributes)?;
	let options = parse_options(options)?;

	let mut command = Command {
		class_name,
		attributes,
		options,
		path: path.to_path_buf(),
	};
	if command.warnings() {
		warnings(&command);
	}
	if command.fix() {
		fix(&mut command);
	}

	return Ok(command);
}

#[allow(dead_code)]
impl Command {
	pub fn getters(&self) -> bool {
		self.options.getters
	}

	pub fn setters(&self) -> bool {
		self.options.setters
	}

	pub fn counter(&self) -> bool {
		self.options.counter
	}

	pub fn to_string(&self) -> bool {
		self.options.to_string
	}

	pub fn equals(&self) -> bool {
		self.options.equals
	}

	pub fn documentation(&self) -> bool {
		self.options.documentation
	}

	pub fn warnings(&self) -> bool {
		self.options.warnings
	}

	pub fn fix(&self) -> bool {
		self.options.fix
	}
}
