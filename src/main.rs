mod core;
mod lexer;
mod parser;
mod context;
mod eval;
mod errors;

use std::io::{self, Write};

use context::context::Context;
use lexer::tokenizer::Tokenizer;
use parser::infix_to_postfix::Parser;
use eval::evaluator::Evaluator;

fn evaluate_expression(
    input: &str,
    ctx: &Context,
) -> Result<i32, errors::error::ExpressionError> {
    let tokens = Tokenizer::tokenize(input)?;
    let postfix = Parser::infix_to_postfix(tokens)?;
    Evaluator::evaluate(&postfix, ctx)
}

fn main() {
    let ctx = Context::new();

    loop {
        print!("> ");
        io::stdout().flush().ok();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Input error");
            continue;
        }

        let input = input.trim();
        if input == "exit" {
            break;
        }

        match evaluate_expression(input, &ctx) {
            Ok(result) => println!("Result: {}", result),
            Err(err) => println!("{}", err),
        }
    }
}
