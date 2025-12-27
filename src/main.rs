mod core;
mod lexer;
mod parser;

use lexer::tokenizer::Tokenizer;
use parser::infix_to_postfix::Parser;

fn main() {
    let input = "(a + b) * 3";

    let tokens = Tokenizer::tokenize(input);
    let postfix = Parser::infix_to_postfix(tokens);

    println!("Postfix Expression:");
    for token in postfix {
        println!("{:?}", token);
    }
}
