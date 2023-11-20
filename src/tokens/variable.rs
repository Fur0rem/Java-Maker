use crate::tokens::expr_type::ExprType;
use crate::tokens::modifier::Modifier;

use super::declaration::Declaration;

use std::{
	borrow::Cow,
	fmt::{self, Display, Formatter},
};

#[derive(Debug, Clone)]
pub struct Variable {
	modifier: Modifier,
	expr_type: ExprType,
	name: String,
	init: Option<String>,
}

impl Variable {
	pub fn new(modifier: Modifier, expr_type: ExprType, name: &str, init: Option<String>) -> Self {
		Self {
			modifier,
			expr_type,
			name: name.to_string(),
			init,
		}
	}

	pub fn init(&self) -> &Option<String> {
		return &self.init;
	}
}

impl Declaration for Variable {
	fn modifier(&self) -> Modifier {
		return self.modifier.clone();
	}

	fn name(&self) -> Option<Cow<str>> {
		return Some(Cow::Borrowed(&self.name));
	}

	fn expr_type(&self) -> Option<ExprType> {
		return Some(self.expr_type.clone());
	}

	fn body(&self) -> (Option<Cow<str>>, bool) {
		return (
			self.init.as_ref().map(|s| Cow::Owned(format!(" = {}", s))),
			false,
		);
	}

	fn end(&self) -> Option<Cow<str>> {
		return Some(Cow::Borrowed(";"));
	}
}

impl Display for Variable {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{} {};", self.expr_type, self.name)
	}
}
