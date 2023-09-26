#![allow(clippy::needless_return)]
#![allow(dead_code)]

use crate::parser::Command;

use crate::constructor;
use crate::getter_setter::getter;
use crate::getter_setter::setter;

use crate::equals;
use crate::to_string;

use crate::comment::Commentable;

use std::collections::HashSet;
use std::io::prelude::*;

fn create_content(command: &Command) -> String {
	let mut content = String::new();

	//get all the needed imports
	let mut imports = HashSet::new();
	for att in &command.attributes {
		let needed_imports = att.needed_imports();
		for imp in needed_imports {
			imports.insert(imp);
		}
	}
	for imp in to_string::needed_imports() {
		imports.insert(imp);
	}

	//add the imports to the content
	for imp in imports {
		content.push_str("import ");
		content.push_str(&imp);
		content.push_str(";\n");
	}
	content.push('\n');

	//create class declaration
	let headline = command.comment();
	content.push_str(&headline);
	content.push_str("public class ");
	content.push_str(&command.class_name);
	content.push_str(" {\n\n");

	//create attributes
	for att in &command.attributes {
		let headline = att.comment();
		content.push_str(&headline);
		content.push_str(&att.to_string());
		content.push('\n');
	}
	//if there's a counter
	if command.options.counter {
		content.push_str("private static int compteur = 0;");
		content.push('\n');
	}
	content.push('\n');

	//create constructor
	content.push_str(&constructor::create_constructor(command));
	content.push('\n');

	//create getters
	if command.options.getters {
		for att in &command.attributes {
			let getter = getter::create_getter(att, &command.class_name);
			let headline = getter.comment();
			content.push_str(&headline);
			content.push_str(&getter.function);
			content.push('\n');
		}
	}
	if command.options.counter {
		content.push_str("public static int getCompteur() {\n");
		content.push_str("return compteur;\n");
		content.push_str("}\n");
	}
	content.push('\n');

	//create setters
	if command.options.setters {
		for att in &command.attributes {
			let setter = setter::create_setter(att, &command.class_name);
			let headline = setter.comment();
			content.push_str(&headline);
			content.push_str(&setter.function);
			content.push('\n');
		}
	}

	//create toString
	if command.options.to_string {
		content.push_str(&to_string::create_to_string(command));
		content.push('\n');
	}

	//create equals
	if command.options.equals {
		content.push_str(&equals::create_equals(command));
		content.push('\n');
	}

	content.push('}');
	return content;
}

fn reformat_code(content: &mut String) {
	//split at every \n
	let lines: Vec<&str> = content.split('\n').collect();

	//for each line, count how many more opening brackets than closing brackets there are
	let mut lines_with_brackets: Vec<(String, isize)> = lines
		.iter()
		.map(|line| {
			let mut count = 0;
			for c in line.chars() {
				if c == '{' {
					count += 1;
				} else if c == '}' {
					count -= 1;
				}
			}
			return (String::from(*line), count);
		})
		.collect();

	//for each line, add the number of opening brackets to the previous line
	for i in 1..lines_with_brackets.len() {
		lines_with_brackets[i].1 += lines_with_brackets[i - 1].1;
	}

	//for each line, add enough tabs to the beginning of the line
	for line in &mut lines_with_brackets {
		let mut tabs = String::new();
		let count = line.1 - line.0.contains('{') as isize;
		for _ in 0..count {
			tabs.push('\t');
		}
		line.0 = tabs + &line.0;
	}

	//recreate the content
	content.clear();
	for (line, _) in lines_with_brackets {
		*content += &line;
		*content += "\n";
	}
}

//Creates the fully formatted java file
pub fn create_class(command: Command) {
	//create the file
	let mut content = create_content(&command);
	reformat_code(&mut content);

	let mut file = std::fs::File::create(format!("{}.java", command.class_name))
		.expect("Unable to create file");

	//write the content to the file
	file.write_all(content.as_bytes())
		.expect("Unable to write to file");
	file.sync_all().expect("Unable to sync file");
	file.flush().expect("Unable to flush file");
}
