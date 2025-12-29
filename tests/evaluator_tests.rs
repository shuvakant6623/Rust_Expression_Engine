use expression_engine::{
    context::context::Context,
    engine::evaluate_expression,
};

#[test] 
fn evaluate_expression_with_variables() {
    let mut ctx = Context::new();
    ctx.set("a", 10);
    ctx.set("b", 5);

    let result = evaluate_expression("(a+b) * 3", &ctx).unwrap();
    assert_eq!(result, 45);
}