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
    (basic $other_type: ident) => {
        $other_type { r#type: $other_type }
    };
    (rule $other_type: ident) => {
        $other_type { content: String }
    };
    ($error: expr, [$($other_type: ident),*], { $( $token_str:expr => $token:ident ),* }) => {
        #[derive(Debug, Clone, PartialEq)]
        pub enum Tokens {
            EOF,
            Unknown(String),
            $(
                $other_type { r#type: $other_type },
            )*
            $(
                #[doc=$token_str]
                $token,
            )*
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
                            let pos = Position::new(start, file_stream.index.clone() - 1);
                            Ok(match file_stream.next_char() {
                                Some(next_char) => {
                                    strs.push(next_char);
                                    if strs.len() > 1 {
                                        return Err(GenCFError { error_message: $error, pos, path })
                                    } else {
                                        Tokens::Unknown(strs)
                                    }
                                },
                                _ => Tokens::EOF
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

            fn eof_token() -> Self {
                Tokens::EOF
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

        impl OtherTokenTrait<$name> for $name  {
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

#[macro_export]
macro_rules! RuleTokenGenerator {
    ($name: ident, $start_rule: pat) => {
        mod $name {
            pub fn start_rule(c: char) -> bool {
                match c {
                    $start_rule => true,
                    _ => false,
                }
            }
        }
    };
}