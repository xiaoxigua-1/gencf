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
    ($error: expr, { $( $token_str:expr => $token:ident ),* }) => {
        #[derive(Debug, Clone)]
        pub enum Tokens {
            EOF,
            $(
                #[doc=$token_str]
                $token,
            )*
        }

        impl Tokens {
            fn new(s: String, pos: Position) -> Result<Tokens, GenCFError> {
                match s.as_str() {
                    $(
                        $token_str => Ok(Tokens::$token),
                    )*
                    _ => Err(GenCFError { error_message: $error, pos })
                }
            }
        }

        pub struct Token {
            pub token_type: Tokens,
            pub pos: Position,
        }
    };
}