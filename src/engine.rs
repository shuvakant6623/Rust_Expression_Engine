use crate::{
    lexer::tokenizer::Tokenizer,
    parser::infix_to_postfix::Parser,
    eval::evaluator::Evaluator,
    context::context::Context,
    errors::error::ExpressionError,
};

pub fn evaluate_expression(
    input: &str,
    ctx: &Context,
) -> Result<i32, ExpressionError> {
    let tokens = Tokenizer::tokenize(input)?;
    let postfix = Parser::infix_to_postfix(tokens)?;
    Evaluator::evaluate(&postfix, ctx)
}
