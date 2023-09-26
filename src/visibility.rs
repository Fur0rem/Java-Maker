use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Default, Clone)]
pub enum Visibility {
	#[default] Private,
	Protected,
	Package,
	Public,
}

impl ToString for Visibility {
	fn to_string(&self) -> String {
		match self {
			Visibility::Public => "public".to_string(),
			Visibility::Private => "private".to_string(),
			Visibility::Protected => "protected".to_string(),
			Visibility::Package => "".to_string(),
		}
	}
}

#[derive(Error,Debug)]
#[error("Unknown visibility, doesn't match public, private, protected or package")]
pub struct UnknownVisibilityError;

impl FromStr for Visibility {
	type Err = UnknownVisibilityError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"public" => Ok(Visibility::Public),
			"private" => Ok(Visibility::Private),
			"protected" => Ok(Visibility::Protected),
			"package" | "_" => Ok(Visibility::Package),
			_ => Err(UnknownVisibilityError)
		}
	}
}