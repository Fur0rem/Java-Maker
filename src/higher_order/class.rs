use std::borrow::Cow;

use crate::tokens::{
	declaration::Declaration, modifier::Modifier, variable::Variable, visibility::Visibility,
};

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

	pub fn modifier(&self) -> &Modifier {
		&self.modifier
	}

	pub fn name(&self) -> &String {
		&self.name
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

	fn begin(&self) -> Option<Cow<str>> {
		return Some(Cow::Borrowed("{"));
	}

	fn end(&self) -> Option<Cow<str>> {
		return Some(Cow::Borrowed("}"));
	}
}
