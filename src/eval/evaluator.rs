use crate::{
    core::{stack::Stack, token::Token},
    context::context::Context,
    errors::error::ExpressionError,
};

pub struct Evaluator;

impl Evaluator {
    pub fn evaluate(postfix: &[Token], ctx: &Context)
        -> Result<i32, ExpressionError>
    {
        let mut stack = Stack::with_capacity(postfix.len());

        for token in postfix {
            match token {
                Token::Number(n) => stack.push(*n),

                Token::Variable(name) => {
                    let v = ctx.get(name)?;
                    stack.push(v);
                }

                Token::Operator(op) => {
                    let r = stack.pop().ok_or(
                        ExpressionError::Evaluation { message: "Missing operand".into() })?;
                    let l = stack.pop().ok_or(
                        ExpressionError::Evaluation { message: "Missing operand".into() })?;

                    let res = match op {
                        '+' => l + r,
                        '-' => l - r,
                        '*' => l * r,
                        '/' => {
                            if r == 0 {
                                return Err(ExpressionError::Evaluation {
                                    message: "Division by zero".into(),
                                });
                            }
                            l / r
                        }
                        '^' => {
                            l.checked_pow(r as u32).ok_or(
                                ExpressionError::Evaluation {
                                    message: "Integer overflow in exponentiation".into(),
                                }
                            )?
                        }
                        _ => unreachable!(),
                    };

                    stack.push(res);
                }

                _ => {}
            }
        }

        stack.pop().ok_or(ExpressionError::Evaluation {
            message: "Invalid expression".into(),
        })
    }
}
