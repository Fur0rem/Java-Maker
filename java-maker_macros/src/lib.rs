use proc_macro::{Delimiter, Group, TokenStream, TokenTree};
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
				for (expr_type, name) in parameters {
					doc.push_str(&format!("\n@param {}", name));
				}
			}
			if let Some(expr_type) = self.expr_type() {
				doc.push_str("\n@return ");
			}
			return Cow::Owned(doc);
		}
	}
	.into()
}

/// This macro is used to generate a pseudo documentation for the generated code.
/// It expects three functions in its input:
/// name
/// parameters
/// expr_type
#[proc_macro]
pub fn document_function(input: TokenStream) -> TokenStream {
	let mut tokens = vec![];

	for token in input.into_iter() {
		tokens.push(token.clone());
		println!("{:?}", token);
		//tokens.push(Punct::new(',', proc_macro::Spacing::Alone).into());
		//tokens.push(token.clone());
	}

	vec![TokenTree::Group(Group::new(
		Delimiter::None,
		tokens.into_iter().collect(),
	))]
	.into_iter()
	.collect()
}
