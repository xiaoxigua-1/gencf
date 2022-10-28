#[gencf::core]
use gencf::lexer::Lexer;
use gencf::{GenCFErrorGenerator, Tokens};

GenCFErrorGenerator!(INVALID_SYNTAX => "invalid syntax");

#[derive(Tokens)]
enum Tokens {
    #[eq("abc")]
    #[eq(123)]
    EOF
}

#[test]
fn token_test() {
    let content = String::from("-*/++++ifabc");
    let mut lexer = Lexer::new(&Path::new(""), &content);
    // loop {
    //     match lexer.next_token::<Tokens>() {
    //         Ok(token) => {
    //             if token.token_type == Tokens::EOF {
    //                 break;
    //             } else {
    //                 println!("{:?}", token);
    //             }
    //         }
    //         Err(err) => {
    //             println!("{:?}", err);
    //             break;
    //         }
    //     }
    // }
}
