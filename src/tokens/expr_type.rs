#![allow(dead_code)]

use std::fmt::Display;
use std::str::FromStr;

use crate::JavaMakerError;

#[derive(Debug, Clone, PartialEq)]
pub struct ExprType {
	pub name: String,
}

impl ExprType {
	pub fn new(name: &str) -> Self {
		ExprType {
			name: name.to_string(),
		}
	}

	pub fn int() -> Self {
		return ExprType::new("int");
	}

	pub fn float() -> Self {
		return ExprType::new("float");
	}

	pub fn double() -> Self {
		return ExprType::new("double");
	}

	pub fn long() -> Self {
		return ExprType::new("long");
	}

	pub fn short() -> Self {
		return ExprType::new("short");
	}

	pub fn byte() -> Self {
		return ExprType::new("byte");
	}

	pub fn char() -> Self {
		return ExprType::new("char");
	}

	pub fn boolean() -> Self {
		return ExprType::new("boolean");
	}

	pub fn void() -> Self {
		return ExprType::new("void");
	}
}

impl Display for ExprType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.name)
	}
}

impl FromStr for ExprType {
	type Err = JavaMakerError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(ExprType::new(s))
	}
}
