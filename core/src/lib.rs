extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn core(_args: TokenStream, input: TokenStream) -> TokenStream {
    // parse the input
    let input = parse_macro_input!(input as ItemFn);
    TokenStream::from(quote!{
        pub struct GenCFError {
            pub error_message: &'static str,
            pub pos: Position,
        }
        
        pub struct Position {
            pub start: usize,
            pub end: usize,
        }
        
        #input
    })
}