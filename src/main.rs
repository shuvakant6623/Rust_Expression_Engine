// Declare top-level modules

mod lexer;

// Bring tokenizer into scope
use lexer::tokenizer::Tokenizer;

fn main() {
    // Test input expression
    let input = "(a + b) * 3";

    // Run tokenizer (lexical analysis)
    let tokens = Tokenizer::tokenize(input);

    // Print tokens to verify correctness
    println!("Input Expression: {}", input);
    println!("Tokens:");
    for token in tokens {
        println!("{:?}", token);
    }
}
