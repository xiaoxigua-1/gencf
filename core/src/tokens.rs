use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::DeriveInput;

pub fn tokens(token_stream: DeriveInput) -> TokenStream {
    let ident = &token_stream.ident;
    
    match token_stream.data {
        syn::Data::Enum(ref r) => {
            quote! {
                impl TokensTrait for #ident {
                    fn new<'a>(file_stream: &mut FileStream<'a>, path: &'a Path) -> Result<Self, GenCFError<'a>> {
                        Ok(Self::EOF)
                    }

                    fn eof_token() -> Self {
                        Self::EOF
                    }
                }
            }
        }
        _ => {
            let error_message = format!("expected {} is enum", ident.to_string());
            let span = token_stream.ident.span();
            let error = syn::Error::new(span, error_message).to_compile_error();
            quote_spanned! {
                span => #error
            }
        }
    }
}