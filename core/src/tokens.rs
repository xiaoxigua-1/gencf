use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned};
use syn::{punctuated::Punctuated, token::Comma, Attribute, DeriveInput, Variant};

struct TokenLex {
    ident: Ident,
    rules: Vec<TokenLexRule>,
}

enum TokenLexRule {
    Eq(String),
}

pub fn tokens(token_stream: DeriveInput) -> TokenStream {
    let ident = &token_stream.ident;

    match token_stream.data {
        syn::Data::Enum(ref r) => {
            match enum_variants(&r.variants) {
                Ok(info) => quote! {
                    impl TokensTrait for #ident {
                        fn new<'a>(file_stream: &mut FileStream<'a>, path: &'a Path) -> Result<Self, GenCFError<'a>> {
                            Ok(Self::EOF)
                        }
    
                        fn eof_token() -> Self {
                            Self::EOF
                        }
                    }
                },
                Err(e) => {
                    let span = e.span();
                    let error = e.to_compile_error();
                    quote_spanned!(span => #error)
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

fn enum_variants(variants: &Punctuated<Variant, Comma>) -> syn::Result<Vec<TokenLex>> {
    variants
        .iter()
        .map(|variant| {
            Ok(TokenLex {
                ident: variant.ident.clone(),
                rules: variant_attributes(&variant.attrs)?,
            })
        })
        .collect::<_>()
}

fn variant_attributes(attrs: &Vec<Attribute>) -> syn::Result<Vec<TokenLexRule>> {
    let attrs = attrs
        .iter()
        .map(|attr| {
            Ok(match &attr.path {
                a if a.is_ident("eq") => {
                    let args = attr.parse_args::<syn::LitStr>()?;
                    TokenLexRule::Eq(args.value())
                }
                _ => TokenLexRule::Eq("".into()),
            })
        })
        .collect::<syn::Result<_>>()?;

    Ok(attrs)
}
