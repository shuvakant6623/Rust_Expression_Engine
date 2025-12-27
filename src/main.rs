mod core;
mod lexer;
mod parser;
mod context;
mod eval;

use lexer::tokenizer::Tokenizer;
use parser::infix_to_postfix::Parser;
use context::context::Context;
use eval::evaluator::Evaluator;
fn main() {
    let input = "(a + b) * 3";

    let tokens = Tokenizer::tokenize(input);
    let postfix = Parser::infix_to_postfix(tokens);

    let mut ctx = Context::new();
    ctx.set("a", 10);
    ctx.set("b", 5);

    let result = Evaluator::evaluate(postfix, &ctx);

    println!("Result: {}", result);
}
