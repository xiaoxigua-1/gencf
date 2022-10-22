use crate::{Position, TokensTrait};

#[derive(Debug, Clone)]
pub struct Token<T> {
    pub token_type: T,
    pub pos: Option<Position>,
}

impl<T> Token<T> {
    pub fn new(token_type: T, pos: Position) -> Self {
        Token {
            token_type,
            pos: Some(pos),
        }
    }

    pub fn eof<TS: TokensTrait>() -> Self
    where
        T: From<TS>,
    {
        Token {
            token_type: TS::eof_token().try_into().unwrap(),
            pos: None,
        }
    }
}
