use crate::core::token::Token;

pub struct Tokenizer;

impl Tokenizer {
    pub fn tokenize(input: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();

        while let Some(&ch) = chars.peek() {
            match ch {
                '0'..='9' => {
                    tokens.push(Self::read_number(&mut chars));
                }
                'a'..='z' | 'A'..='Z' => {
                    tokens.push(Self::read_variable(&mut chars));
                }
                '+' | '-' | '*' | '/' | '^' => {
                    chars.next();
                    tokens.push(Token::Operator(ch));
                }
                '(' => {
                    chars.next();
                    tokens.push(Token::LParen);
                }
                ')' => {
                    chars.next();
                    tokens.push(Token::RParen);
                }
                ' ' | '\t' | '\n' => {
                    chars.next(); // ignore whitespace
                }
                _ => {
                    panic!("Unexpected character: {}", ch);
                }
            }
        }

        tokens
    }

    fn read_number(chars: &mut std::iter::Peekable<std::str::Chars>) -> Token {
        let mut num = 0;

        while let Some(&ch) = chars.peek() {
            if ch.is_ascii_digit() {
                num = num * 10 + (ch as i32 - '0' as i32);
                chars.next();
            } else {
                break;
            }
        }

        Token::Number(num)
    }

    fn read_variable(chars: &mut std::iter::Peekable<std::str::Chars>) -> Token {
        let mut name = String::new();

        while let Some(&ch) = chars.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                name.push(ch);
                chars.next();
            } else {
                break;
            }
        }

        Token::Variable(name)
    }
}
