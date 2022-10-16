#[gencf::core]
use gencf::{GenCFErrorGenerator, LexerGenerator, OtherTokenGenerator, TokensGenerator};

GenCFErrorGenerator!(INVALID_SYNTAX => "invalid syntax");
OtherTokenGenerator!(Keywords, {
    "if" => If
});
TokensGenerator!(
    GenCFErrorMessage::INVALID_SYNTAX,
    [Keywords],
    {
        "-" => Minus,
        "*" => Star,
        "/" => Slash,
        "++" => PlusPlus
    }
);
LexerGenerator!(
    Tokens,
    {
        ' ' | '\n' | '\t' | '\r' | '\u{000C}' | '\u{000B}' => pass
    }
);

#[test]
fn token_test() {
    let content = String::from("-*/++++if ac");
    let mut lexer = Lexer::new(&Path::new(""), &content);
    loop {
        match lexer.next_token::<Tokens>() {
            Ok(token) => {
                if token.token_type == Tokens::EOF {
                    break;
                } else {
                    println!("{:?}", token);
                }
            }
            Err(err) => {
                println!("{:?}", err);
                break;
            }
        }
    }
}
