use std::collections::HashSet;
use std::io::prelude::*;

use crate::higher_order::class::Class;
use crate::higher_order::constructor::Constructor;
use crate::higher_order::getter_setter::getter::Getter;
use crate::higher_order::getter_setter::setter::Setter;
use crate::tokens::declaration::Declaration;
use crate::tokens::visibility::Visibility;
use crate::translation::parser::Command;

fn push_comment(command: &Command, content: &mut String, comment: &str) {
	if command.options.documentation {
		content.push_str("/*");
		content.push_str(comment);
		content.push_str("*/\n");
	}
}

fn create_content(command: &Command) -> String {
	let mut content = String::new();

	let class = Class::new(
		Visibility::Public,
		&command.class_name,
		command.attributes.clone(),
	);

	let mut declarations: Vec<Box<dyn Declaration>> = Vec::new();
	for var in class.attributes() {
		declarations.push(Box::new(var.clone()));
	}
	declarations.push(Box::new(Constructor::new(&class)));

	if command.getters() {
		for var in class.attributes() {
			declarations.push(Box::new(Getter::new(var)));
		}
	}

	if command.setters() {
		for var in class.attributes() {
			if Setter::can_be_set(var) {
				declarations.push(Box::new(Setter::new(var)));
			}
		}
	}

	let mut imports: HashSet<String> = HashSet::new();
	for c in &declarations {
		for imp in c.needed_imports() {
			imports.insert(imp);
		}
	}
	//add the imports to the content
	for imp in imports {
		content.push_str("import ");
		content.push_str(&imp);
		content.push_str(";\n");
	}
	content.push('\n');

	push_comment(command, &mut content, &class.document());
	content.push_str(&class.modifier().to_string());
	content.push_str(&class.name().unwrap());
	content.push(' ');
	content.push_str(&class.begin().unwrap());
	content.push('\n');

	for c in declarations {
		push_comment(command, &mut content, &c.document());
		if let Some(decorator) = c.decorator() {
			content.push_str(format!("@{}\n", decorator).as_str());
		}
		content.push_str(&c.modifier().to_string());
		if let Some(expr_type) = c.expr_type() {
			content.push_str(&expr_type.to_string());
			content.push(' ');
		}

		content.push_str(&c.name().unwrap());

		if let Some(parameters) = c.parameters() {
			content.push('(');
			for (i, (expr_type, name)) in parameters.iter().enumerate() {
				if i != 0 {
					content.push_str(", ");
				}
				content.push_str(&expr_type.to_string());
				content.push(' ');
				content.push_str(name);
			}
			content.push(')');
		}

		if let Some(body) = c.begin() {
			content.push_str(format!(" {}\n", body).as_str());
		}

		if let (Some(body), new_line) = c.body() {
			content.push_str(&body);
			if new_line {
				content.push('\n');
			}
		}

		if let Some(end) = c.end() {
			content.push_str(&end);
			content.push('\n');
		}

		content.push('\n');
	}

	content.push_str(&class.end().unwrap());

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
pub fn create_class(command: Command) -> Result<(), std::io::Error> {
	//create the file
	let mut content = create_content(&command);
	reformat_code(&mut content);

	//create the file
	let file_path = command.path.join(format!("{}.java", command.class_name));
	let mut file = std::fs::File::create(file_path)?;

	//write the content to the file
	file.write_all(content.as_bytes())?;
	file.flush()?;
	return Ok(());
}
