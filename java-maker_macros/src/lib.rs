use proc_macro::TokenStream;
use quote::quote;
// use syn;

use convert_case::{Case, Casing};

#[proc_macro]
pub fn function(_input: TokenStream) -> TokenStream {
	quote! {
		fn begin(&self) -> Option<Cow<str>> {
			Some(Cow::Borrowed("{"))
		}

		fn end(&self) -> Option<Cow<str>> {
			Some(Cow::Borrowed("}"))
		}

		fn document(&self) -> Cow<str> {
			let mut doc = format!("TODO : documentation for {}", self.name().unwrap());
			if let Some(parameters) = self.parameters() {
				if !parameters.is_empty() {
					doc.push('\n');
				}
				for (expr_type, name) in &parameters {
					doc.push_str(&format!("\n@param {}", name));
				}
			}
			if let Some(expr_type) = self.expr_type() {
				use crate::tokens::expr_type::ExprType;
				if expr_type != ExprType::void() {
					doc.push_str("\n\n@return ");
				}
			}
			return Cow::Owned(doc);
		}
	}
	.into()
}

/// options!("getters") => pub fn getters(&self) -> bool { self.options.contains("getters") }
/// TODO : do this for multiple options
/// TODO : Or ever detect them from the Option enum
#[proc_macro]
pub fn options(input: TokenStream) -> TokenStream {
	// convert to a string
	let options = input.to_string();
	// remove the quotes
	let option_name = options[1..options.len() - 1].to_string();
	let function_name = syn::parse_str::<syn::Ident>(&option_name).unwrap();
	let option_variant_name = option_name.to_case(Case::UpperCamel);
	let option_variant = syn::parse_str::<syn::Ident>(&option_variant_name).unwrap();

	quote! {
		pub fn #function_name(&self) -> bool {
			self.options.contains(&Option::#option_variant)
		}
	}
	.into()
}
