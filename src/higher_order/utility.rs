use crate::tokens::{Declaration, ExprType, Modifier, Visibility};

use super::class::Class;
use java_maker_macros::function;
use std::borrow::Cow;

pub struct ToString<'a> {
	class: &'a Class,
}

impl<'a> ToString<'a> {
	pub fn new(class: &'a Class) -> Self {
		Self { class }
	}
}

impl<'a> Declaration for ToString<'a> {
	fn modifier(&self) -> Modifier {
		return Modifier::new(Visibility::Public, Vec::new());
	}

	fn name(&self) -> Option<Cow<str>> {
		return Some(Cow::Borrowed("toString"));
	}

	fn parameters(&self) -> Option<Vec<(ExprType, Cow<str>)>> {
		return Some(Vec::new());
	}

	fn expr_type(&self) -> Option<ExprType> {
		return Some(ExprType::new("String"));
	}

	fn body(&self) -> (Option<Cow<str>>, bool) {
		let mut function = String::new();
		function.push_str("StringBuilder sb = new StringBuilder();\n");
		function.push_str("sb.append(\"");
		function.push_str(&self.class.name().unwrap());
		function.push_str("(\");\n");
		let nb_attributes = self.class.attributes().len();
		for (i, var) in self.class.attributes().iter().enumerate() {
			function.push_str(&format!("sb.append(\"{}=\");\n", &var.name().unwrap()));
			function.push_str(&format!("sb.append(this.{});\n", var.name().unwrap()));
			// if not the last
			// I used enumerate because else I would need to derive PartialEq for Variable
			// and every other field inside the struct
			if i != nb_attributes - 1 {
				function.push_str("sb.append(\", \");\n");
			}
		}
		function.push_str("sb.append(\")\");\n");
		function.push_str("return sb.toString();");
		return (Some(Cow::Owned(function)), true);
	}

	function!();

	fn decorator(&self) -> Option<Cow<str>> {
		return Some(Cow::Borrowed("Override"));
	}
}
