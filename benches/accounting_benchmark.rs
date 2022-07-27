use rust_accounting::{accounting, excel, file, settings};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn same_path_benchmark(c: &mut Criterion) {
    c.bench_function("same path", |b| {
        b.iter(|| settings::lib::same_path("/test/", "/test/"))
    });
}

criterion_group!(benches, same_path_benchmark);
criterion_main!(benches);
