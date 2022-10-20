extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item};

#[proc_macro_attribute]
pub fn core(_args: TokenStream, input: TokenStream) -> TokenStream {
    // parse the input
    let input = parse_macro_input!(input as Item);
    TokenStream::from(quote! {
        use std::path::Path;
        use std::fmt::Debug;
        use std::sync::Arc;
        use gencf::{GenCFError, Position, FileStream, TokensTrait, TokenTrait, KeywordTrait};

        pub struct Rule {}

        impl Rule {
            fn pass(c: char) -> bool {
                true
            }

            fn pop(c: char) -> bool {
                false
            }
        }

        #input
    })
}
