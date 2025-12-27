use crate::core::precedence::precedence;
use crate::core::stack::Stack;
use crate::core::token::Token;

pub struct Parser;

impl Parser {
    pub fn infix_to_postfix(tokens: Vec<Token>) -> Vec<Token> {
        let mut output: Vec<Token> = Vec::new();
        let mut stack: Stack<Token> = Stack::new(100);

        for token in tokens {
            match token {
                Token::Number(_) | Token::Variable(_) => {
                    output.push(token);
                }

                Token::Operator(op) => {
                    while let Ok(top) = stack.peek() {
                        if let Token::Operator(top_op) = top {
                            if precedence(*top_op) >= precedence(op) {
                                output.push(stack.pop().unwrap());
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    stack.push(Token::Operator(op));
                }

                Token::LParen => {
                    stack.push(Token::LParen);
                }

                Token::RParen => {
                    while let Ok(top) = stack.pop() {
                        if top == Token::LParen {
                            break;
                        }
                        output.push(top);
                    }
                }
            }
        }

        while let Ok(op) = stack.pop() {
            output.push(op);
        }

        output
    }
}
