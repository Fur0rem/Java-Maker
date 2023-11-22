use proc_macro::TokenStream;
use quote::quote;
// use syn;

#[proc_macro]
pub fn curly_braces_codeblock(_input: TokenStream) -> TokenStream {
	quote! {
		fn begin(&self) -> Option<Cow<str>> {
			return Some(Cow::Borrowed("{"));
		}

		fn end(&self) -> Option<Cow<str>> {
			return Some(Cow::Borrowed("}"));
		}
	}
	.into()
}
