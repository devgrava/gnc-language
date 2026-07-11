use gnc::lexer::Lexer;
use gnc::token::token::Token;

fn main() {
    let source = String::from("a != b <= c >= d < e > f");

    let mut lexer = Lexer::new(source);

    loop {
        let token = lexer.next_token();

        println!("{:?}", token);

        if token == Token::EOF {
            break;
        }
    }
}
