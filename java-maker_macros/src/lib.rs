use proc_macro::TokenStream;
use quote::quote;
// use syn;

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
