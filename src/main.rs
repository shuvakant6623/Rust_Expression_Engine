use std::io::{self, Write};

use expression_engine::{
    engine::evaluate_expression,
    context::context::Context,
};

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
