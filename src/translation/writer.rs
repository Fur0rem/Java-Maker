use std::collections::HashSet;
use std::io::prelude::*;

use crate::higher_order::getter_setter::{getter::Getter, setter::Setter};
use crate::higher_order::{class::Class, constructor::Constructor};
use crate::parser::Command;
use crate::tokens::declaration::Declaration;
use crate::tokens::visibility::Visibility;
use crate::translation::format::reformat_code;

fn push_document(command: &Command, content: &mut String, document: &str) {
	if command.options.documentation {
		content.push_str("/**\n");
		for line in document.split('\n') {
			content.push_str(" * ");
			content.push_str(line);
			content.push('\n');
		}
		content.push_str(" */\n");
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

	if command.to_string() {
		declarations.push(Box::new(crate::higher_order::to_string::ToString::new(
			&class,
		)));
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

	push_document(command, &mut content, &class.document());
	content.push_str(&class.modifier().to_string());
	content.push_str(&class.name().unwrap());
	content.push(' ');
	content.push_str(&class.begin().unwrap());
	content.push('\n');

	for c in declarations {
		push_document(command, &mut content, &c.document());
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
