pub mod file_stream;
pub mod gen_cf_error;
pub mod lexer;
pub mod position;
pub mod token;

pub use file_stream::FileStream;
pub use gen_cf_error::GenCFError;
pub use position::Position;
use std::path::Path;
pub use token::Token;

pub trait TokensTrait {
    fn new<'a>(file_stream: &mut FileStream<'a>, path: &'a Path) -> Result<Self, GenCFError<'a>>
    where
        Self: Sized;

    fn eof_token() -> Self;
}

pub trait OtherTokenTrait<T> {
    fn find(s: &str, pos: Position) -> Option<T>;
}

pub trait RuleTokenTrait {
    fn new(content: String) -> Self;

    fn start_rule(c: char) -> bool;

    fn end_rule(c: char) -> bool;

    fn is_eof() -> &'static str;
}