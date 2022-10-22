extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Item, ItemEnum};

#[proc_macro_attribute]
pub fn core(_args: TokenStream, input: TokenStream) -> TokenStream {
    // parse the input
    let input = parse_macro_input!(input as Item);
    TokenStream::from(quote! {
        use std::path::Path;
        use std::fmt::Debug;
        use std::sync::Arc;
        use std::ops::Range;
        use gencf::{GenCFError, Position, FileStream, TokensTrait, OtherTokenTrait, TokenType, token_type_attribute, Token};

        #input
    })
}

#[proc_macro_derive(TokenType, attributes(basic, rule))]
pub fn token_type(_input: TokenStream) -> TokenStream {
    TokenStream::from(quote! {})
}

#[proc_macro_attribute]
pub fn token_type_attribute(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemEnum);
    let vis = &input.vis;
    let ident = &input.ident;
    let variants = &input
        .variants
        .iter()
        .map(|variant| {
            let ident = &variant.ident;
            println!("{}", ident);
            if variant
                .attrs
                .iter()
                .find(|attr| attr.path.is_ident("rule"))
                .is_some()
            {
                quote! {
                    #ident { content: String }
                }
            } else if variant
                .attrs
                .iter()
                .find(|attr| attr.path.is_ident("basic"))
                .is_some()
            {
                quote! {
                    #ident { r#type: #ident }
                }
            } else {
                quote! {
                    #variant
                }
            }
        })
        .collect::<Vec<TokenStream2>>();

    let output = quote! {
        #vis enum #ident {
            #(#variants),*
        }
    };
    println!("{}", output);
    TokenStream::from(output)
}
