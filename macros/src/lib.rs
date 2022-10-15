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
        pub struct Token {
            pub token_type: Tokens,
            pub pos: Option<Position>,
        }

        impl Tokens {
            fn find(token_str: &str, pos: Position) -> Result<Tokens, GenCFError> {
                match token_str {
                    $(
                        $token_str => Ok(Tokens::$token),
                    )*
                    _ => Err(GenCFError {
                        error_message: $error,
                        pos,
                    })
                }
            }
        }

        impl Tokens_trait<Tokens> for Tokens {
            fn new<'a>(file_stream: &mut FileStream<'a>) -> Result<Tokens, GenCFError> {
                let mut tokens = vec![$($token_str),*];
                let mut prev_tokens = tokens.clone();
                let mut strs = String::new();
                let mut index = 0;
                let start = file_stream.index.clone();

                loop {
                    let peep_char = file_stream.peep_char();
                    prev_tokens = tokens.clone();
                    tokens = tokens.iter_mut()
                        .filter(|token| {
                            token.chars().nth(index) == peep_char
                        })
                        .map(|token| { token.clone() })
                    .collect::<Vec<&str>>();
                    if tokens.len() == 1 {
                        file_stream.next_char();
                        break Tokens::find(tokens[0], Position {
                            start: start,
                            end: file_stream.index.clone(),
                        })
                    } else if tokens.is_empty() {
                        break if prev_tokens.is_empty() {
                            Err(GenCFError {
                                error_message: $error,
                                pos: Position {
                                    start: start,
                                    end: file_stream.index.clone(),
                                },
                            })
                        } else {
                            Tokens::find(prev_tokens.iter().find(|token| { token.to_string() == strs }).unwrap(), Position {
                                start: start,
                                end: file_stream.index.clone(),
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
    };
}

#[macro_export]
macro_rules! KeywordGenerator {
    ($name: ident, $error: expr, $( $keyword:expr => $keyword_type:ident ),*) => {
        #[derive(Debug, Clone, PartialEq)]
        pub enum $name {
            $(
                #[doc=$keyword]
                $keyword_type,
            )*
        }

        impl Keyword_trait<$name> for $name  {
            fn new(s: String, pos: Position) -> Result<$name, GenCFError> {
                match s.as_str() {
                    $(
                        $keyword => Ok($name::$keyword_type),
                    )*
                    _ => Err(GenCFError { error_message: $error, pos })
                }
            }
        }
    };
}

#[macro_export]
macro_rules! TokenRule {
    (@step $_idx:expr) => {
        $rule => true,
        _ => false
    };
    ($rule: pat) => {
        $rule
    };
    ($($start_rule: pat),*) => {
        match c {
            TokenRule!(
                @step 0,
                [$($start_rule),*],
                $rule
            )
        }
    };
    (@step $index: expr, $now_rule: pat, $($start_rule: pat),*) => {
        $now_rule if strs.len() == $index => true,
        TokenRule!(
            @step $index + 1,
            [$($start_rule),*],
            $rule
        )
    }
}

#[macro_export]
macro_rules! RuleGenerator {
    (_) => {
        _
    };
    ([$rule: pat]) => {
        $rule
    };
}

#[macro_export]
macro_rules! LexerGenerator {
    ($tokens_type: ty, { $($all_rule: pat, [$($rule: pat),*] => $token: ident),* }) => {
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

            pub fn next_token<T: Tokens_trait<$tokens_type>>(
                &mut self,
            ) -> Result<Token, GenCFError> {
                let mut strs = String::new();
                let start = self.file_stream.index.clone();
                let token = loop {
                    let end = self.file_stream.index.clone();
                    let pos = Position::new(start, end);
                    match self.file_stream.peep_char() {
                        None => {
                            break if strs.is_empty() {
                                Token {
                                    token_type: <$tokens_type>::EOF,
                                    pos: None,
                                }
                            } else {
                                Token {
                                    token_type: <$tokens_type>::new(&mut self.file_stream)?,
                                    pos: Some(pos),
                                }
                            }
                        }
                        Some(c) => {
                            match c {
                                $(
                                    $all_rule => {
                                        let rule_array: Vec<std::ops::Range<char>> = vec![$($rule),*];
                                        if !rule_array.is_empty() && rule_array[strs.len()].contains(&c) {
                                            strs.push(c);
                                        } else {
                                            break Token {
                                                token_type: <$tokens_type>::new(&mut self.file_stream)?,
                                                pos: Some(Position::new(start, self.file_stream.index.clone() - 1)),
                                            }
                                        }
                                    }
                                )*
                            }
                        }
                    }
                };

                Ok(token)
            }
        }
    };
}
