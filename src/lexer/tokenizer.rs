use crate::core::token::Token;
use crate::errors::error::ExpressionError;

pub struct Tokenizer;

impl Tokenizer {
    pub fn tokenize(input: &str) -> Result<Vec<Token>, ExpressionError> {
        if input.trim().is_empty() {
            return Err(ExpressionError::Lexer {
                message: "Empty expression".into(),
                position: 0,
            });
        }

        let mut tokens = Vec::new();
        let mut chars = input.chars().enumerate().peekable();

        while let Some((idx, ch)) = chars.peek().cloned() {
            match ch {
                '0'..='9' => {
                    let mut num = 0;
                    while let Some((_, c)) = chars.peek() {
                        if c.is_ascii_digit() {
                            num = num * 10 + (*c as i32 - '0' as i32);
                            chars.next();
                        } else { break; }
                    }
                    tokens.push(Token::Number(num));
                }

                'a'..='z' | 'A'..='Z' => {
                    let mut name = String::new();
                    while let Some((_, c)) = chars.peek() {
                        if c.is_alphanumeric() || *c == '_' {
                            name.push(*c);
                            chars.next();
                        } else { break; }
                    }
                    tokens.push(Token::Variable(name));
                }

                '+' | '-' | '*' | '/' | '^' => {
                    chars.next();
                    tokens.push(Token::Operator(ch));
                }

                '(' => { chars.next(); tokens.push(Token::LParen); }
                ')' => { chars.next(); tokens.push(Token::RParen); }

                ' ' | '\t' | '\n' => { chars.next(); }

                _ => {
                    return Err(ExpressionError::Lexer {
                        message: format!("Invalid character '{}'", ch),
                        position: idx,
                    });
                }
            }
        }

        Ok(tokens)
    }
}
