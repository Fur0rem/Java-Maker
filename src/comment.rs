use crate::attribute::Attribute;
use crate::getter_setter::getter::Getter;
use crate::getter_setter::setter::Setter;
use crate::parser::Command;

const PLACEHOLDER: &str = "TODO : PLACEHOLDER";

pub trait Commentable {
	fn comment(&self) -> String;
}

impl Commentable for Attribute {
	fn comment(&self) -> String {
		format!("/** {PLACEHOLDER} */\n")
	}
}

impl Commentable for Getter {
	fn comment(&self) -> String {
		format!(
			"/**\n* Accede {PLACEHOLDER} a {} de {}\n* @return\n*/\n",
			&self.attribute.var_name, &self.class_name
		)
	}
}

impl Commentable for Setter {
	fn comment(&self) -> String {
		format!(
			"/**\n* Modifie {} {PLACEHOLDER} de {}\n* @param {}\n*/\n",
			&self.attribute.var_name, &self.class_name, &self.attribute.var_name
		)
	}
}

impl Commentable for Command {
	fn comment(&self) -> String {
		format!("/**\n* Classe {} {PLACEHOLDER}\n*/\n", &self.class_name)
	}
}
