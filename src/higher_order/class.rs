use std::borrow::Cow;

use crate::tokens::{Declaration, ExprType, Modifier, Variable, Visibility};

extern crate java_maker_macros;
use java_maker_macros::function;

pub struct Class {
	pub modifier: Modifier,
	pub name: String,
	pub attributes: Vec<Variable>,
}

impl Class {
	pub fn new(visibility: Visibility, name: &str, attributes: Vec<Variable>) -> Self {
		let modifier = Modifier::new(visibility, vec!["class"]);
		Self {
			modifier,
			name: name.to_string(),
			attributes,
		}
	}

	pub fn attributes(&self) -> &Vec<Variable> {
		&self.attributes
	}
}

impl Declaration for Class {
	fn modifier(&self) -> Modifier {
		return self.modifier.clone();
	}

	fn name(&self) -> Option<Cow<str>> {
		return Some(Cow::Borrowed(&self.name));
	}

	function!();
}

/// Represents the constructor of a class
pub struct Constructor<'a> {
	class: &'a Class,
}

impl<'a> Constructor<'a> {
	pub fn new(class: &'a Class) -> Self {
		Self { class }
	}
}

impl Declaration for Constructor<'_> {
	fn modifier(&self) -> Modifier {
		return Modifier::new(Visibility::Public, Vec::new());
	}

	fn name(&self) -> Option<Cow<str>> {
		return Some(Cow::Borrowed(&self.class.name));
	}

	fn parameters(&self) -> Option<Vec<(ExprType, Cow<str>)>> {
		let mut parameters = Vec::new();

		for var in self.class.attributes() {
			if var.init().is_some() || var.modifier().is_static() || var.init().is_some() {
				continue;
			}
			parameters.push((var.expr_type().unwrap(), var.name().unwrap()));
		}

		return Some(parameters);
	}

	fn body(&self) -> (Option<Cow<str>>, bool) {
		let mut body = String::new();
		body.push_str("super();\n");

		for var in self.class.attributes() {
			if var.init().is_some() || var.modifier().is_static() || var.init().is_some() {
				continue;
			}

			body.push_str(&format!(
				"this.{} = {};\n",
				var.name().unwrap(),
				var.name().unwrap()
			));
		}
		body.pop();
		return (Some(Cow::Owned(body)), true);
	}

	function!();
}
