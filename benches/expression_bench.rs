use criterion::{criterion_group, criterion_main, Criterion};
use expression_engine::{
    engine::evaluate_expression,
    context::context::Context,
};

fn bench_expression(c: &mut Criterion) {
    let mut ctx = Context::new();
    ctx.set("a", 10);
    ctx.set("b", 5);

    c.bench_function("evaluate (a+b)*3", |b| {
        b.iter(|| {
            evaluate_expression("(a + b) * 3", &ctx).unwrap()
        })
    });
}

criterion_group!(benches, bench_expression);
criterion_main!(benches);
