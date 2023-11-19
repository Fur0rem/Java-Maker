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
