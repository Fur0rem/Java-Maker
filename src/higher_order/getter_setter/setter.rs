use crate::tokens::declaration::Declaration;
use crate::tokens::expr_type::ExprType;
use crate::tokens::modifier::Modifier;
use crate::tokens::variable::Variable;
use crate::tokens::visibility::Visibility;

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

	fn name(&self) -> Option<String> {
		return Some(format!(
			"set{}",
			self.var.name().unwrap()[0..1].to_uppercase() + &self.var.name().unwrap()[1..]
		));
	}

	fn parameters(&self) -> Option<Vec<(ExprType, String)>> {
		return Some(vec![(
			self.var.expr_type().unwrap(),
			self.var.name().unwrap(),
		)]);
	}

	fn expr_type(&self) -> Option<ExprType> {
		return Some(ExprType::void());
	}

	fn body(&self) -> (Option<String>, bool) {
		return (
			Some(format!(
				"this.{} = {};",
				self.var.name().unwrap(),
				self.var.name().unwrap()
			)),
			true,
		);
	}

	fn begin(&self) -> Option<String> {
		return Some(String::from("{"));
	}

	fn end(&self) -> Option<String> {
		return Some(String::from("}"));
	}
}
