use crate::tokens::expr_type::ExprType;
use crate::tokens::modifier::Modifier;

use super::visibility::Visibility;

pub trait Declaration {
	fn decorator(&self) -> Option<String> {
		None
	}

	fn modifier(&self) -> Modifier {
		Modifier::new(Visibility::default(), Vec::new())
	}

	fn name(&self) -> Option<String> {
		None
	}

	fn parameters(&self) -> Option<Vec<(ExprType, String)>> {
		None
	}

	fn expr_type(&self) -> Option<ExprType> {
		None
	}

	fn body(&self) -> (Option<String>, bool) {
		(None, false)
	}

	fn begin(&self) -> Option<String> {
		None
	}

	fn end(&self) -> Option<String> {
		None
	}

	fn document(&self) -> String {
		String::from("TODO : documentation")
	}

	fn needed_imports(&self) -> Vec<String> {
		Vec::new()
	}
}
