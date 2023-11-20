use std::borrow::Cow;

use crate::tokens::declaration::Declaration;

use crate::tokens::expr_type::ExprType;
use crate::tokens::modifier::Modifier;
use crate::tokens::visibility::Visibility;

use super::class::Class;

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

	fn begin(&self) -> Option<Cow<str>> {
		return Some(Cow::Borrowed("{"));
	}

	fn end(&self) -> Option<Cow<str>> {
		return Some(Cow::Borrowed("}"));
	}
}
