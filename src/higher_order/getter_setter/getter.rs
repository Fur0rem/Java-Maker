#![allow(dead_code)]

use crate::tokens::declaration::Declaration;
use crate::tokens::expr_type::ExprType;
use crate::tokens::modifier::Modifier;
use crate::tokens::variable::Variable;
use crate::tokens::visibility::Visibility;

pub struct Getter<'a> {
	var: &'a Variable,
}

impl<'a> Getter<'a> {
	pub fn new(var: &'a Variable) -> Self {
		Self { var }
	}
}

impl Declaration for Getter<'_> {
	fn modifier(&self) -> Modifier {
		return Modifier::from_keywords(Visibility::Public, self.var.modifier().keywords().clone());
	}

	fn name(&self) -> Option<String> {
		return Some(format!(
			"get{}",
			self.var.name().unwrap()[0..1].to_uppercase() + &self.var.name().unwrap()[1..]
		));
	}

	fn parameters(&self) -> Option<Vec<(ExprType, String)>> {
		return Some(Vec::new());
	}

	fn expr_type(&self) -> Option<ExprType> {
		return self.var.expr_type();
	}

	fn body(&self) -> (Option<String>, bool) {
		let mut body = String::new();
		body.push_str("return ");
		body.push_str(&self.var.name().unwrap());
		body.push(';');
		return (Some(body), true);
	}

	fn begin(&self) -> Option<String> {
		return Some(String::from("{"));
	}

	fn end(&self) -> Option<String> {
		return Some(String::from("}"));
	}
}
