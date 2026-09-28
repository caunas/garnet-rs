mod ast;
mod lexing;
mod parsing;

use lexing::Lexer;
use parsing::Parser;

fn main() {
    let source = r#"
        int::x = 10;
        int::y = 20;

        x = 30;

        print(x);
    "#;

    let mut lexer = Lexer::new(source);

    let tokens = lexer.tokenize();

    println!("TOKENS:");
    println!("{:#?}", tokens);

    let mut parser = Parser::new(tokens);

    let ast = parser.parse();

    println!("\nAST:");
    println!("{:#?}", ast);
}