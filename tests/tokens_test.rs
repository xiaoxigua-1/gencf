use gencf::{GenCFErrorGenerator, TokensGenerator};

#[test]
#[gencf::core]
fn token_test() {
    GenCFErrorGenerator!(INVALID_SYNTAX => "invalid syntax");
    TokensGenerator!(GenCFErrorMessage::INVALID_SYNTAX, {
        "+" => Plus,
        "-" => Minus,
        "*" => Star,
        "/" => Slash
    });
}