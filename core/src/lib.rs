extern crate proc_macro;

mod tokens;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item, DeriveInput};

#[proc_macro_attribute]
pub fn core(_args: TokenStream, input: TokenStream) -> TokenStream {
    // parse the input
    let input = parse_macro_input!(input as Item);
    TokenStream::from(quote! {
        use std::path::Path;
        use std::fmt::Debug;
        use std::sync::Arc;
        use std::ops::Range;
        use gencf::{GenCFError, Position, FileStream, TokensTrait, OtherTokenTrait, Token};

        #input
    })
}

#[proc_macro_derive(Tokens, attributes(eq))]
pub fn tokens_derive(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(input);

    TokenStream::from(tokens::tokens(derive_input))
}