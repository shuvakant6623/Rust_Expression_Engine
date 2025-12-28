mod core;
mod lexer;
mod parser;
mod context;
mod eval;

use std::io::{self, Write};

use lexer::tokenizer::Tokenizer;
use parser::infix_to_postfix::Parser;
use context::context::Context;
use eval::evaluator::Evaluator;

fn main() {
    println!("=== Rust Expression Engine ===");
    println!("Type expressions or assignments (e.g. a = 10)");
    println!("Type 'exit' to quit\n");

    let mut ctx = Context::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" {
            break;
        }

        if let Some((var, value)) = parse_assignment(input) {
            ctx.set(&var, value);
            println!("Set {} = {}", var, value);
            continue;
        }

        let tokens = Tokenizer::tokenize(input);
        let postfix = Parser::infix_to_postfix(tokens);
        let result = Evaluator::evaluate(postfix, &ctx);

        println!("Result: {}", result);
    }

    println!("Goodbye!");
}

fn parse_assignment(input: &str) -> Option<(String, i32)> {
    let parts: Vec<&str> = input.split('=').collect();
    if parts.len() != 2 {
        return None;
    }

    let var = parts[0].trim();
    let value = parts[1].trim().parse::<i32>().ok()?;

    Some((var.to_string(), value))
}
