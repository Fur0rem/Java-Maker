use convert_case::{Case, Casing};
use std::borrow::Cow;

use crate::tokens::declaration::Declaration;
use crate::tokens::expr_type::ExprType;
use crate::tokens::modifier::Modifier;
use crate::tokens::variable::Variable;
use crate::tokens::visibility::Visibility;

extern crate java_maker_macros;
use java_maker_macros::function;

pub struct Setter<'a> {
	var: &'a Variable,
}

impl<'a> Setter<'a> {
	pub fn new(var: &'a Variable) -> Self {
		Self { var }
	}

	pub fn can_be_set(var: &Variable) -> bool {
		return !var.modifier().is_static() && !var.modifier().is_final();
	}
}

impl Declaration for Setter<'_> {
	fn modifier(&self) -> Modifier {
		return Modifier::from_keywords(Visibility::Public, self.var.modifier().keywords().clone());
	}

	fn name(&self) -> Option<Cow<str>> {
		return Some(Cow::Owned(format!(
			"set{}",
			self.var.name().unwrap().to_case(Case::Pascal)
		)));
	}

	fn parameters(&self) -> Option<Vec<(ExprType, Cow<str>)>> {
		return Some(vec![(
			self.var.expr_type().unwrap(),
			self.var.name().unwrap(),
		)]);
	}

	fn expr_type(&self) -> Option<ExprType> {
		return Some(ExprType::void());
	}

	fn body(&self) -> (Option<Cow<str>>, bool) {
		let mut body = String::new();
		body.push_str(&format!(
			"this.{} = {};\n",
			self.var.name().unwrap(),
			self.var.name().unwrap()
		));
		body.pop();
		return (Some(Cow::Owned(body)), true);
	}

	function!();
}
