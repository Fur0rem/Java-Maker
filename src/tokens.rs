use std::borrow::Cow;
use std::fmt::{self, Formatter};
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, Default, Clone, Copy, PartialEq, EnumString, EnumIter, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Visibility {
	#[default]
	Private,
	Protected,
	Package,
	Public,
}

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

use std::fmt::Display;
use std::str::FromStr;

use crate::JavaMakerError;

#[derive(Debug, Clone, PartialEq)]
pub struct ExprType {
	pub name: String,
}

macro_rules! primitive_type {
	($name:ident) => {
		pub fn $name() -> ExprType {
			return ExprType::new(stringify!($name));
		}
	};
}

impl ExprType {
	pub fn new(name: &str) -> Self {
		ExprType {
			name: name.to_string(),
		}
	}

	primitive_type!(double);
	primitive_type!(float);
	primitive_type!(long);
	primitive_type!(int);
	primitive_type!(short);
	primitive_type!(byte);
	primitive_type!(boolean);
	primitive_type!(void);
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

#[derive(Debug, Clone, PartialEq, EnumString, EnumIter, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Keyword {
	Static,
	Final,
	Synchronized,
	Abstract,
	Volatile,
	Transient,
	Native,
	Strictfp,
	Class,
}

#[derive(Debug, Clone)]
pub struct Modifier {
	pub visibility: Visibility,
	pub extra_info: Vec<Keyword>,
}

impl Modifier {
	pub fn new(vis: Visibility, extra_info: Vec<&str>) -> Self {
		let mut extra_info = extra_info
			.into_iter()
			.filter(|s| !s.is_empty())
			.map(|s| Keyword::from_str(s).unwrap())
			.collect::<Vec<Keyword>>();
		extra_info.dedup();

		Modifier {
			visibility: vis,
			extra_info,
		}
	}

	pub fn from_keywords(vis: Visibility, extra_info: Vec<Keyword>) -> Self {
		Modifier {
			visibility: vis,
			extra_info,
		}
	}

	pub fn keywords(&self) -> &Vec<Keyword> {
		return &self.extra_info;
	}

	pub fn has_keyword(&self, keyword: Keyword) -> bool {
		return self.extra_info.contains(&keyword);
	}

	pub fn is_static(&self) -> bool {
		return self.has_keyword(Keyword::Static);
	}

	pub fn is_final(&self) -> bool {
		return self.has_keyword(Keyword::Final);
	}
}

/// Order : visibility extra_info
/// Example : public static final ...
/// Example : private ...
impl ToString for Modifier {
	fn to_string(&self) -> String {
		let mut modifier = String::new();
		modifier.push_str(&self.visibility.to_string());
		modifier.push(' ');

		for info in &self.extra_info {
			modifier.push_str(&info.to_string());
			modifier.push(' ');
		}

		return modifier;
	}
}

impl Modifier {
	pub fn from_strs(vis: &str, extra_info: Vec<&str>) -> Modifier {
		let vis = Visibility::from_str(vis).unwrap();
		return Modifier::new(vis, extra_info);
	}
}

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

	pub fn update_name(&mut self, name: &str) {
		self.name = name.to_string();
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
