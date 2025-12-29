use expression_engine::{
    lexer::tokenizer::Tokenizer,
    parser::infix_to_postfix::Parser,
    core::token::Token,
    context::context::Context,
    engine::evaluate_expression,
};

#[test] 
fn infix_to_postfix_precedence() {
    let tokens = Tokenizer::tokenize("a + b * 3").unwrap();
    let postfix = Parser::infix_to_postfix(tokens).unwrap();

    let expected = vec![
        Token::Variable("a".into()),
        Token::Variable("b".into()),
        Token::Number(3),
        Token::Operator('*'),
        Token::Operator('+'),
    ];

    assert_eq!(postfix, expected);
}

#[test]
fn unmatched_paranthesis_returns_parser_error() {
    let ctx = Context::new();
    let err = evaluate_expression("(a + b", &ctx).unwrap_err();

    let msg = err.to_string();
    assert!(msg.contains("Unmatched"));
}

#[test]
fn implicit_multiplication_is_error() {
    let ctx = Context::new();
    let result = evaluate_expression("3(7*4)", &ctx);
    assert!(result.is_err());
}
