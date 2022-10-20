pub mod file_stream;
pub mod gen_cf_error;
pub mod lexer;
pub mod position;

pub use file_stream::FileStream;
pub use gen_cf_error::GenCFError;
pub use position::Position;
use std::path::Path;

pub trait TokensTrait {
    fn new<'a>(file_stream: &mut FileStream<'a>, path: &'a Path) -> Result<Self, GenCFError<'a>>
    where
        Self: Sized;
}

pub trait TokenTrait {
    type TokenType;

    fn new(token_type: Self::TokenType, pos: Position) -> Self;

    fn eof() -> Self;
}

pub trait KeywordTrait<T> {
    fn find(s: &str, pos: Position) -> Option<T>;
}
