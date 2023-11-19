#![allow(dead_code)]
#![allow(clippy::needless_return)]

use crate::tokens::visibility::Visibility;

use std::fmt::Display;
//from str
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
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

impl Display for Keyword {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Keyword::Static => "static",
				Keyword::Final => "final",
				Keyword::Synchronized => "synchronized",
				Keyword::Abstract => "abstract",
				Keyword::Volatile => "volatile",
				Keyword::Transient => "transient",
				Keyword::Native => "native",
				Keyword::Strictfp => "strictfp",
				Keyword::Class => "class",
			}
		)
	}
}

impl FromStr for Keyword {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"static" => Ok(Keyword::Static),
			"final" => Ok(Keyword::Final),
			"synchronized" => Ok(Keyword::Synchronized),
			"abstract" => Ok(Keyword::Abstract),
			"volatile" => Ok(Keyword::Volatile),
			"transient" => Ok(Keyword::Transient),
			"native" => Ok(Keyword::Native),
			"strictfp" => Ok(Keyword::Strictfp),
			"class" => Ok(Keyword::Class),
			_ => Err(()),
		}
	}
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
