use crate::{gen_cf_error::GenCFError, position::Position, token::Token, FileStream, TokensTrait};
use std::path::Path;

pub struct Lexer<'a> {
    file_stream: FileStream<'a>,
    path: &'a Path
}

impl Lexer<'_> {
    pub fn new<'a>(path: &'a Path, file_content: &'a String) -> Lexer<'a> {
        Lexer {
            file_stream: FileStream::new(file_content.chars()),
            path,
        }
    }

    pub fn next_token<TS: TokensTrait>(&mut self) -> Result<Token<TS>, GenCFError> {
        let start = self.file_stream.index.clone();
        let token = loop {
            match self.file_stream.peep_char() {
                None => break Token::eof(),
                _ => {
                    let token_type = match TS::new(&mut self.file_stream, &self.path) {
                        Ok(token_type) => token_type,
                        Err(e) => return Err(e),
                    };

                    break Token::new(
                        token_type.try_into().unwrap(),
                        Position::new(start, self.file_stream.index.clone() - 1),
                    );
                }
            }
        };

        Ok(token)
    }
}
