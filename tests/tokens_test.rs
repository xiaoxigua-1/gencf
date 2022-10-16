use gencf::OtherTokenGenerator;
#[gencf::core]
use gencf::{GenCFErrorGenerator, LexerGenerator, TokensGenerator};

GenCFErrorGenerator!(INVALID_SYNTAX => "invalid syntax");
OtherTokenGenerator!(Keywords, "if" => If);
TokensGenerator!(
    GenCFErrorMessage::INVALID_SYNTAX,
    [Keywords],
    {
        "+" => Plus,
        "-" => Minus,
        "*" => Star,
        "/" => Slash,
        "++" => PlusPlus
    }
);
LexerGenerator!(
    Tokens,
    {
        'a'..='z' => Keywords,
        _ => Tokens
    }
);

#[test]
fn token_test() {
    let content = String::from("+-*/++if ac");
    let mut lexer = Lexer::new(&Path::new(""), &content);
    while let Ok(token) = lexer.next_token::<Tokens>() {
        println!("{:?}", token);
        if token.token_type == Tokens::EOF {
            break;
        }
    }
}
