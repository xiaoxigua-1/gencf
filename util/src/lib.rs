pub mod file_stream;
pub mod gen_cf_error;
pub mod lexer;
pub mod position;

pub use file_stream::FileStream;
pub use gen_cf_error::GenCFError;
pub use position::Position;


pub trait TokensTrait {
    fn new<'a>(file_stream: &mut FileStream<'a>) -> Result<Self, GenCFError> where Self: Sized;
}

pub trait TokenTrait {
    type TokenType;

    fn new(token_type: Self::TokenType, pos: Position) -> Self;

    fn eof() -> Self;
}