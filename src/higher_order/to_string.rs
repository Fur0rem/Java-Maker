#![allow(clippy::needless_return)]
#![allow(dead_code)]

use crate::tokens::{
	expr_type::ExprType,
	modifier::Modifier,
	traits::{Commentable, Declaration},
	visibility::Visibility,
};

use super::class::Class;

pub struct ToString<'a> {
	class: &'a Class,
}

impl<'a> Commentable for ToString<'a> {
	fn comment(&self) -> String {
		return String::new();
	}
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

	fn name(&self) -> Option<String> {
		return Some("toString".to_string());
	}

	fn parameters(&self) -> Option<Vec<(ExprType, String)>> {
		return Some(Vec::new());
	}

	fn expr_type(&self) -> Option<ExprType> {
		return Some(ExprType::new("String"));
	}

	fn body(&self) -> (Option<String>, bool) {
		let mut function = String::new();
		function.push_str("StringBuilder sb = new StringBuilder();\n");
		function.push_str("sb.append(\"");
		function.push_str(self.class.name());
		function.push_str("(\");\n");
		for var in self.class.attributes() {
			function.push_str(&format!("sb.append(\"{}=\");\n", &var.name()));
			function.push_str(&format!("sb.append({});\n", &var.name()));
			function.push_str("sb.append(\",\");\n");
		}
		function.push_str("sb.append(\")\");\n");
		function.push_str("return sb.toString();");
		return (Some(function), true);
	}

	fn begin(&self) -> Option<String> {
		return Some(String::from("{"));
	}

	fn end(&self) -> Option<String> {
		return Some(String::from("}"));
	}

	fn decorator(&self) -> Option<String> {
		return Some(String::from("Override"));
	}
}
