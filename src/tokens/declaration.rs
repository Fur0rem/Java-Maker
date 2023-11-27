use std::borrow::Cow;

use crate::tokens::expr_type::ExprType;
use crate::tokens::modifier::Modifier;

use super::visibility::Visibility;

pub trait Declaration {
	fn decorator(&self) -> Option<Cow<str>> {
		None
	}

	fn modifier(&self) -> Modifier {
		Modifier::new(Visibility::default(), Vec::new())
	}

	fn name(&self) -> Option<Cow<str>> {
		None
	}

	fn parameters(&self) -> Option<Vec<(ExprType, Cow<str>)>> {
		None
	}

	fn expr_type(&self) -> Option<ExprType> {
		None
	}

	fn body(&self) -> (Option<Cow<str>>, bool) {
		(None, false)
	}

	fn begin(&self) -> Option<Cow<str>> {
		None
	}

	fn end(&self) -> Option<Cow<str>> {
		None
	}

	fn document(&self) -> Cow<str> {
		Cow::Owned(format!("TODO : documentation for {}", self.name().unwrap()))
	}

	fn needed_imports(&self) -> Vec<String> {
		Vec::new()
	}
}
