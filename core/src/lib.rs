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
        use gencf::FileStream;

        #[derive(Debug)]
        pub struct GenCFError {
            pub error_message: &'static str,
            pub pos: Position,
        }

        #[derive(Debug, Clone)]
        pub struct Position {
            pub start: usize,
            pub end: usize,
        }

        pub trait Keyword_trait<T> {
            fn find(s: &str, pos: Position) -> Option<T>;
        }

        pub trait Tokens_trait<T> {
            fn new<'a>(file_stream: &mut FileStream<'a>) -> Result<T, GenCFError>;
        }

        impl Position {
            pub fn new(start: usize, end: usize) -> Position {
                Position { start, end }
            }
        }

        #input
    })
}
