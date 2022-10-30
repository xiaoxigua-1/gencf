#[gencf::core]
use gencf::lexer::Lexer;
use gencf::{GenCFErrorGenerator, Tokens};

GenCFErrorGenerator!(INVALID_SYNTAX => "invalid syntax");

#[derive(Tokens, PartialEq, Debug)]
enum Tokens {
    #[eq("+")]
    Plus,
    #[eq("++")]
    PlusPlus,
    EOF
}

#[test]
fn token_test() {
    let content = String::from("+++++");
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
