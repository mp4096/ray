use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ray::material::Material;

fn criterion_benchmark(c: &mut Criterion) {
    let input = 5u64;
    c.bench_with_input(BenchmarkId::new("function_name", input), &input, |b, i| {
        b.iter(|| {
            // Code to benchmark using input `i` goes here
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
