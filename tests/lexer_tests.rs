use expression_engine::lexer::tokenizer::Tokenizer;
use expression_engine::core::token::Token;

#[test] 
fn tokenize_simple_expression() {
    let tokens = Tokenizer::tokenize("(a + b) * 3").unwrap();

    let expected = vec![
        Token::LParen,
        Token::Variable("a".into()),
        Token::Operator('+'),
        Token::Variable("b".into()),
        Token::RParen,
        Token::Operator('*'),
        Token::Number(3),
    ];

    assert_eq!(tokens, expected);
}