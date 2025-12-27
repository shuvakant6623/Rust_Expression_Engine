use crate::context::context::Context;
use crate::core::stack::Stack;
use crate::core::token::Token;

pub struct Evaluator;

impl Evaluator {
    pub fn evaluate(postfix: Vec<Token>, ctx: &Context) -> i32 {
        let mut stack: Stack<i32> = Stack::new(200);

        for token in postfix {
            match token {
                Token::Number(n) => {
                    stack.push(n);
                }

                Token::Variable(name) => {
                    let value = ctx.get(&name);
                    stack.push(value);
                }

                Token::Operator(op) => {
                    let right = stack.pop().expect("Missing operand");
                    let left = stack.pop().expect("Missing operand");

                    let result = match op {
                        '+' => left + right,
                        '-' => left - right,
                        '*' => left * right,
                        '/' => left / right,
                        '^' => left.pow(right as u32),
                        _ => panic!("Unknown operator"),
                    };

                    stack.push(result);
                }

                _ => panic!("Invalid token in evaluation"),
            }
        }

        stack.pop().expect("Invalid expression")
    }
}
