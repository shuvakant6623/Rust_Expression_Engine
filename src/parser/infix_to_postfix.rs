use crate::core::{stack::Stack, token::Token, precedence::precedence};
use crate::errors::error::ExpressionError;

pub struct Parser;

impl Parser {
    pub fn infix_to_postfix(tokens: Vec<Token>)
        -> Result<Vec<Token>, ExpressionError>
    {
        let mut output = Vec::new();
        let mut stack = Stack::with_capacity(tokens.len());

        for token in tokens {
            match token {
                Token::Number(_) | Token::Variable(_) => {
                    output.push(token);
                }

                Token::Operator(op) => {
                    while let Some(Token::Operator(top)) = stack.peek() {
                        if precedence(*top) >= precedence(op) {
                            output.push(stack.pop().unwrap());
                        } else { break; }
                    }
                    stack.push(Token::Operator(op));
                }

                Token::LParen => stack.push(Token::LParen),

                Token::RParen => {
                    let mut matched = false;
                    while let Some(t) = stack.pop() {
                        if t == Token::LParen {
                            matched = true;
                            break;
                        }
                        output.push(t);
                    }
                    if !matched {
                        return Err(ExpressionError::Parser {
                            message: "Unmatched ')'".into(),
                        });
                    }
                }
            }
        }

        while let Some(t) = stack.pop() {
            if t == Token::LParen {
                return Err(ExpressionError::Parser {
                    message: "Unmatched '('".into(),
                });
            }
            output.push(t);
        }

        Ok(output)
    }
}
