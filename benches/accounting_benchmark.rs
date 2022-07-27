use rust_accounting::{accounting, excel, file, settings};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn write_benchmark(c: &mut Criterion) -> Result<(), Box<dyn std::error::Error>> {
    c.bench_function("write", |b| {
        b.iter(|| {
            excel::test_setup::initialize();
            // could optimize more, meaning not creating mock transaction for each iteration
            let mock = excel::test_setup::create_mock_transactions(1000)?;
            excel::writing::write(
                &mock.path,
                mock.info,
                &mock.categories,
                &mock.date_delimiter,
                &mock.date_month_style,
                &mock.date_language,
                &mock.date_capitalize,
            )
        })
    });
    Ok(())
}

fn same_path_benchmark(c: &mut Criterion) {
    c.bench_function("same path", |b| {
        b.iter(|| settings::lib::same_path("/test/", "/test/"))
    });
}

criterion_group!{
    name = benches_excel;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().significance_level(0.1).sample_size(10);
    targets = write_benchmark
}
criterion_group!(benches_functions, same_path_benchmark, write_benchmark);
criterion_main!(benches_excel);
criterion_main!(benches_functions);