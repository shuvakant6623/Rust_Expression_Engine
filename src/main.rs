mod token;
mod stack;
mod precedence;

use token :: Token;
use stack :: Stack;

fn main() {
    let mut stack = Stack :: <Token> :: new(100);

    stack.push(Token::Number(3));
    stack.push(Token::Operator('+'));

    println!("{:?}", stack.peek());
}