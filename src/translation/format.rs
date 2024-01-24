use crate::parser::Command;
use crate::tokens::Declaration;
use crate::tokens::ExprType;
use convert_case::{Case, Casing};
use inline_colorization::{color_reset, color_yellow};

pub fn reformat_code(content: &mut String) {
	let lines: Vec<&str> = content.split('\n').collect();

	//for each line, count how many more opening brackets than closing brackets there are
	let mut lines_with_brackets: Vec<(String, isize)> = lines
		.iter()
		.map(|line| {
			let mut count = 0;
			for c in line.chars() {
				if c == '{' {
					count += 1;
				}
				else if c == '}' {
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

macro_rules! warn{
	($($arg:tt)*) => {
		println!(
			"{color_yellow}Warning:{color_reset} {}",
			format!($($arg)*)
		);
	};
}

pub fn warnings(command: &Command) {
	let class_name = &command.class_name;
	if class_name.chars().next().unwrap().is_lowercase() {
		warn!("class name {class_name} should start with an uppercase letter",);
	}

	for var in &command.attributes {
		let name = var.name().unwrap();
		if name.chars().next().unwrap().is_uppercase() {
			warn!("variable name {name} should start with a lowercase letter",);
		}
		if let Some(expr_type) = var.expr_type() {
			if expr_type == ExprType::boolean() {
				if !name.starts_with("is") {
					warn!("variable name {name} should start with \"is\" because it is a boolean",);
				}
			}
			else if name.starts_with("is") {
				warn!(
					"variable name {name} should not start with \"is\" because it is not a boolean",
				);
			}
		}
	}
}

pub fn fix(command: &mut Command) {
	command.class_name = command.class_name.to_case(Case::Pascal);

	for var in &mut command.attributes {
		if let Some(expr_type) = var.expr_type() {
			if expr_type == ExprType::boolean() && !var.name().unwrap().starts_with("is") {
				var.update_name(&format!("is{}", var.name().unwrap()));
			}
			else if var.name().unwrap().starts_with("is") {
				var.update_name(&var.name().unwrap()[2..]);
			}
		}
		var.update_name(&var.name().unwrap().to_string().to_case(Case::Camel));
	}
}
