use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned};
use std::collections::HashMap;
use syn::{punctuated::Punctuated, token::Comma, Attribute, DeriveInput, Variant};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct TokenLex {
    ident: Ident,
    rule: TokenLexRule,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
enum TokenLexRule {
    Eq(String),
    None,
}

impl TokenLex {
    // fn get_tokens(&self) -> TokenStream {
    //     quote!()
    // }
}

impl TokenLexRule {
    fn nth(&self, index: usize) -> Option<char> {
        match &self {
            TokenLexRule::Eq(str) => str.chars().nth(index),
            _ => None,
        }
    }

    fn is_eq(&self) -> bool {
        match &self {
            TokenLexRule::Eq(..) => true,
            _ => false,
        }
    }
}

pub fn tokens(token_stream: DeriveInput) -> TokenStream {
    let ident = &token_stream.ident;

    match token_stream.data {
        syn::Data::Enum(ref r) => match enum_variants(&r.variants) {
            Ok(info) => gen_tokens_impl(info, ident),
            Err(e) => {
                let span = e.span();
                let error = e.to_compile_error();
                quote_spanned!(span => #error)
            }
        },
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
    Ok(variants
        .iter()
        .map(|variant| {
            Ok(variant_attributes(&variant.attrs)?
                .iter()
                .map(|rule| TokenLex {
                    ident: variant.ident.clone(),
                    rule: rule.clone(),
                })
                .collect::<Vec<TokenLex>>())
        })
        .collect::<syn::Result<Vec<_>>>()?
        .concat())
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
                _ => TokenLexRule::None
            })
        })
        .collect::<syn::Result<_>>()?;

    Ok(attrs)
}

fn gen_tokens_impl(info: Vec<TokenLex>, name: &Ident) -> TokenStream {
    let token_stream = gen_match(&info, 0);
    println!("{}", token_stream);
    quote! {
        impl TokensTrait for #name {
            fn new<'a>(file_stream: &mut FileStream<'a>, path: &'a Path) -> Result<Self, GenCFError<'a>> {
                let peep_char = file_stream.peep_char();
                let start = file_stream.index.clone();
                Ok(#token_stream)
            }

            fn eof_token() -> Self {
                Self::EOF
            }
        }
    }
}

/// Generation lexer match syntax
fn gen_match(info: &Vec<TokenLex>, index: usize) -> TokenStream {
    let mut eq_map: HashMap<char, Vec<TokenLex>> = HashMap::new();
    let mut match_char: Vec<TokenStream> = vec![];
    let mut now_ident: Option<Ident> = None;

    info.iter()
        .filter_map(|info| {
            if info.rule.is_eq() {
                Some(info.clone())
            } else {
                None
            }
        })
        .for_each(|rule| {
            let now_char = rule.rule.nth(index);
            if let Some(now_char) = now_char {
                eq_map.entry(now_char)
                    .and_modify(|lexs| lexs.push(rule.clone()))
                    .or_insert(vec![rule.clone()]);
            } else {
                now_ident = Some(rule.ident.clone());
            }
        });

    for (k, v) in eq_map.iter() {
        let token_stream = gen_match(v, index + 1);
        match_char.push(quote! {
            Some(#k) => {
                file_stream.next_char();
                let peep_char = file_stream.peep_char();
                
                #token_stream
            }
        })
    }

    let now_ident = if let Some(now_ident) = now_ident {
        quote! { Self::#now_ident }
    } else {
        quote! { return Err(GenCFError::new("", Position::new(start, file_stream.index.clone()), path)) }
    };

    if !eq_map.is_empty() {
        quote! {
            match peep_char {
                #(#match_char)*
                _ => #now_ident
            }
        }
    } else {
        quote! {
            #now_ident
        }
    }
}

