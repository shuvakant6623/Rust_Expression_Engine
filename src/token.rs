
#[derive(Debug,Clone,PartialEq)]
pub enum Token {
    Number(i32),
    Operator(char),
    Variable(String),
    LParen,
    RParen,
}