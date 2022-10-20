#[macro_export]
macro_rules! GenCFErrorGenerator {
    ($( $error_name: ident => $error_message: expr ),*) => {
        mod GenCFErrorMessage {
            $(
                #[doc=$error_message]
                pub const $error_name: &str = $error_message;
            )*
        }
    }
}

#[macro_export]
macro_rules! TokensGenerator {
    ($error: expr, [$($other_type: ident),*], { $( $token_str:expr => $token:ident ),* }) => {
        #[derive(Debug, Clone, PartialEq)]
        pub enum Tokens {
            EOF,
            $(
                $other_type { r#type: $other_type },
            )*
            $(
                #[doc=$token_str]
                $token,
            )*
        }

        #[derive(Debug, Clone)]
        pub struct Token<T> {
            pub token_type: T,
            pub pos: Option<Position>,
        }

        impl Tokens {
            fn find<'a>(token_str: &str, pos: Position, path: &'a Path) -> Result<Tokens, GenCFError<'a>> {
                match token_str {
                    $(
                        $token_str => Ok(Tokens::$token),
                    )*
                    _ => {
                        $(
                            let find = <$other_type>::find(token_str, pos.clone());
                            if let Some(find) = find {
                                return Ok(Tokens::$other_type {
                                    r#type: find,
                                })
                            }
                        )*
                        Err(GenCFError {
                            error_message: $error,
                            pos,
                            path
                        })
                    }
                }
            }
        }

        impl TokensTrait for Tokens {
            fn new<'a>(file_stream: &mut FileStream<'a>, path: &'a Path) -> Result<Tokens, GenCFError<'a>> {
                let mut tokens = vec![$($token_str),*];
                let mut prev_tokens = tokens.clone();
                let mut strs = String::new();
                let mut index = 0;
                let start = file_stream.index.clone();
                $(
                    tokens.append(&mut $other_type::tokens());
                )*
                loop {
                    let peep_char = file_stream.peep_char();
                    prev_tokens = tokens.clone();
                    tokens = tokens.iter_mut()
                        .filter(|token| {
                            token.chars().nth(index) == peep_char
                        })
                        .map(|token| { token.clone() })
                    .collect::<Vec<&str>>();

                    if tokens.is_empty() {
                        let find = prev_tokens.iter().find(|token| { token.to_string() == strs });
                        break if let Some(find) = find {
                            Tokens::find(find, Position {
                                start: start,
                                end: file_stream.index.clone(),
                            }, path)
                        } else {
                            Err(GenCFError {
                                error_message: $error,
                                pos: Position::new(
                                    start,
                                    file_stream.index.clone(),
                                ),
                                path
                            })
                        }

                    } else {
                        if let Some(next_char) = file_stream.next_char() {
                            strs.push(next_char);
                        } else {
                            break Ok(Tokens::EOF)
                        }
                    }
                    index += 1;
                }
            }
        }

        impl TokenTrait for Token<Tokens> {
            type TokenType = Tokens;

            fn new(token_type: Self::TokenType, pos: Position) -> Self {
                Token { token_type: token_type, pos: Some(pos) }
            }

            fn eof() -> Self {
                Token { token_type: Tokens::EOF, pos: None }
            }
        }
    };
}

#[macro_export]
macro_rules! OtherTokenGenerator {
    ($name: ident, { $( $token:expr => $token_type:ident ),* }) => {
        #[derive(Debug, Clone, PartialEq)]
        pub enum $name {
            $(
                #[doc=$token]
                $token_type,
            )*
        }

        impl $name {
            fn tokens() -> Vec<&'static str> {
                vec![$($token),*]
            }
        }

        impl KeywordTrait<$name> for $name  {
            fn find(s: &str, pos: Position) -> Option<$name> {
                match s {
                    $(
                        $token => Some($name::$token_type),
                    )*
                    _ => None
                }
            }
        }
    };
}
