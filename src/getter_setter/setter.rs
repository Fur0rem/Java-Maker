#![allow(clippy::needless_return)]
#![allow(dead_code)]

use crate::attribute::Attribute;
use crate::visibility::Visibility;

pub struct Setter {
	pub function : String,
	pub attribute : Attribute,
	pub class_name : String,
}

pub fn create_setter(att : &Attribute, class_name : &String) -> Setter {

	let mut function = String::new();

	//on vérifie si l'attribut est final
	if att.is_final() {
 		return Setter {
 			function,
 			attribute : att.clone(),
 			class_name : class_name.clone(),
 		};
 	}

	//visibility of the function
	function.push_str(&Visibility::Public.to_string());
	function.push(' ');

	//on vérifie si l'attribut est static
	let indentifier = if att.is_static() {
			function.push_str("static ");
			"this"
		} else {
			class_name
		};

	//return type of the function
	function.push_str("void");
	function.push(' ');

	//name of the function
	function.push_str("set");
	function.push_str(&att.var_name[..1].to_uppercase());
	function.push_str(&att.var_name[1..]);

	//parameters of the function
	function.push('(');
	function.push_str(&att.var_type);
	function.push(' ');
	function.push_str(&att.var_name);
	function.push(')');
	function.push(' ');

	//opening of the function
	function.push('{');
	function.push('\n');

	//body of the function
	function.push_str(indentifier);
	function.push('.');
	function.push_str(&att.var_name);
	function.push_str(" = ");
	function.push_str(&att.var_name);
	function.push(';');
	function.push('\n');

	//closing of the function
	function.push('}');
	function.push('\n');

	return Setter {
		function,
		attribute : att.clone(),
		class_name : class_name.clone(),
	};
}