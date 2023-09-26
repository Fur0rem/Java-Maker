#![allow(dead_code)]
#![allow(clippy::needless_return)]

use std::str::FromStr;

use crate::visibility::Visibility;

#[derive(Debug, Clone)]
pub struct Attribute {
	pub var_name : String,
	pub var_type : String,
	pub visibility : Visibility,
	pub extra_info : Option<Vec<String>>,
}

impl Attribute {
	pub fn new(var_name : &str, var_type : &str, visibility : Visibility, extra_info : Option<Vec<String>>) -> Attribute {
		Attribute {
			var_name : var_name.to_string(),
			var_type : var_type.to_string(),
			visibility,
			extra_info,
		}
	}

	pub fn is_static(&self) -> bool {
		return self.extra_info.is_some() && self.extra_info.as_ref().unwrap().contains(&"static".into());
	}

	pub fn is_final(&self) -> bool {
		return self.extra_info.is_some() && self.extra_info.as_ref().unwrap().contains(&"final".into());
	}
}


impl ToString for Attribute {
	fn to_string(&self) -> String {
		// order : visibility type extra_info name
		let mut s = String::new();	

		s.push_str(&self.visibility.to_string());

		if let Some(extra_info) = &self.extra_info {
			for info in extra_info {
				s.push(' ');
				s.push_str(info);
			}
		}

		s.push(' ');
		s.push_str(&self.var_type);

		s.push(' ');
		s.push_str(&self.var_name);
		
		s.push(';');
		return s;
	}
}

impl Attribute {
	pub fn needed_imports(&self) -> Vec<String> {
		//check if the type is a collection
		//split at every '<'
		let mut type_vec : Vec<&str> = self.var_type.split('<').collect();
		//remove the last element
		type_vec.pop();
		//remove all the '<' and trim
		let type_vec = type_vec.iter().map(|s| s.trim().replace('<', "")).collect::<Vec<String>>();
		let mut imports : Vec<String> = Vec::new();
		for type_str in type_vec {	
			imports.push(format!("java.util.{}", type_str));
		}

		imports
	}
}

pub fn split_attribute(attribute : &str) -> Attribute {

	let colon = attribute.find(':').unwrap();
	let at = attribute.find('@');
	let open_parenthesis = attribute.find('(');
	let close_parenthesis = attribute.find(')');
	
	//find name and type
	let var_name = attribute[..colon].trim().to_string();

	let range = colon+1..match (at, open_parenthesis) {
		(Some(at), _) => at,
		(None, Some(open_parenthesis)) => open_parenthesis,
		(None, None) => attribute.len(),
	};
	
	let var_type = attribute[range].trim().to_string();

	//find visibility
	let visibility = if let Some(at) = at {
			let range = match open_parenthesis {
				Some(open_parenthesis) => at+1..open_parenthesis,
				None => at+1..close_parenthesis.unwrap_or(attribute.len()),
			};

			Visibility::from_str(attribute[range].trim()).unwrap()
		}
		else {
			Visibility::default()
		};

	//find extra_info
	let extra_info = if let Some(open_parenthesis) = open_parenthesis {
			let range = open_parenthesis+1..close_parenthesis.unwrap();
			let extra_info_str = &attribute[range];
			let extra_info_vec : Vec<&str> = extra_info_str.split('|').collect();
			Some(extra_info_vec.iter().map(|s| s.trim().to_string()).collect())
		}
		else {
			None
		};


	Attribute {
		var_name,
		var_type,
		visibility,
		extra_info,
	}

}

pub fn parse_attributes(attributes : String) -> Vec<Attribute> {
	let mut attributes = attributes.trim().to_string();
	attributes = attributes.replace('\n', " ");
	attributes = attributes.replace('\t', " ");
	attributes = attributes.replace("  ", " ");
	attributes = attributes.trim().to_string();

	let mut attributes_vec : Vec<Attribute> = Vec::new();

	let mut index = 0;
	while index < attributes.len() {
		let mut attribute = String::new();
		let mut open_brackets = 0;
		let mut close_brackets = 0;
		while index < attributes.len() {
			let c = attributes.chars().nth(index).unwrap();
			match c {
				'{' => open_brackets += 1,
				'}' => close_brackets += 1,
				',' if open_brackets == close_brackets => break,
				_ => ()
			}
			attribute.push(c);
			index += 1;
		}
		attributes_vec.push(split_attribute(&attribute));
		index += 1;
	}

	return attributes_vec;
}