use std::path::Path;
use crate::{FileStream, TokensTrait, gen_cf_error::GenCFError, TokenTrait, position::Position};

pub struct Lexer<'a> {
    file_stream: FileStream<'a>,
    path: &'a Path,
}

impl Lexer<'_> {
    pub fn new<'a>(path: &'a Path, file_content: &'a String) -> Lexer<'a> {
        Lexer {
            file_stream: FileStream::new(file_content.chars()),
            path,
        }
    }

    pub fn next_token<T: TokenTrait, TS: TokensTrait>(
        &mut self,
    ) -> Result<T, GenCFError> where <T as TokenTrait>::TokenType: From<TS> {
        let mut strs = String::new();
        let start = self.file_stream.index.clone();
        let token = loop {
            match self.file_stream.peep_char() {
                None => {
                    break T::eof()
                }
                Some(c) => {
                    let token_type = match TS::new(&mut self.file_stream) {
                        Ok(token_type) => token_type,
                        Err(e) => {
                            // loop {
                            //     self.file_stream.next_char();
                            //     match c {
                            //         $($rule => { 
                            //             if gencf::RuleGenerator!($token) {
                            //                 continue;
                            //             };
                            //          })*
                            //         _ => return Err(e)
                            //     }
                            // }
                            return Err(e)
                        }
                    };

                    break T::new(
                        token_type.try_into().unwrap(),
                        Position::new(start, self.file_stream.index.clone() - 1),
                    )
                }
            }
        };

        Ok(token)
    }
}