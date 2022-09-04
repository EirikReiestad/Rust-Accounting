use rust_accounting::{excel};

use criterion::{criterion_group, criterion_main, Criterion};

const MOCK_TRANSACTIONS: u32= 100;

fn create_mock_transactions_benchmark(c: &mut Criterion) -> Result<(), Box<dyn std::error::Error>> {
    let _  = excel::test_setup::create_mock_transactions(MOCK_TRANSACTIONS).map_err(|e| format!("{:?}", e))?;
    Ok(())
}

fn write_benchmark(c: &mut Criterion) -> Result<(), Box<dyn std::error::Error>> {
    // the problem with this is that it is testing the test_setup initialize in the benchmark... as well as creating mock transaction
    c.bench_function("write", |b| {
        b.iter(|| {
            excel::test_setup::initialize();
            // could optimize more, meaning not creating mock transaction for each iteration
            let mock = excel::test_setup::create_mock_transactions(MOCK_TRANSACTIONS)?;
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

// fn remove_duplicates_benchmark(c: &mut Criterion) -> Result<(), Box<dyn std::error::Error>> {

// }

criterion_group!{
    name = benches_creating_mock_transactions;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().significance_level(0.1).sample_size(10);
    targets = create_mock_transactions_benchmark
}
criterion_group!{
    name = benches_excel;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().significance_level(0.1).sample_size(10);
    targets = write_benchmark
}

criterion_main!(benches_creating_mock_transactions);