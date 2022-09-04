use rust_accounting::excel;

use criterion::{criterion_group, criterion_main, Criterion};

const MOCK_TRANSACTIONS: u32 = 100;

fn create_mock_transactions_benchmark(c: &mut Criterion) -> Result<(), Box<dyn std::error::Error>> {
    c.bench_function("create_mock_transactions", |b| {
        b.iter(|| {
            let _ = excel::test_setup::create_mock_transactions(MOCK_TRANSACTIONS, None)
                .map_err(|e| format!("{:?}", e));
        })
    });
    Ok(())
}

fn write_benchmark(c: &mut Criterion) -> Result<(), Box<dyn std::error::Error>> {
    // the problem with this is that it is testing the test_setup initialize in the benchmark... as well as creating mock transaction
    let mock = excel::test_setup::create_mock_transactions(MOCK_TRANSACTIONS, None)?;
    c.bench_function("write", |b| {
        b.iter(|| {
            excel::test_setup::initialize();
            // could optimize more, meaning not creating mock transaction for each iteration
            excel::writing::write(
                &mock.path,
                &mock.info,
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

fn remove_duplicates_benchmark(c: &mut Criterion) -> Result<(), Box<dyn std::error::Error>> {
    let mock1 =
        excel::test_setup::create_mock_transactions(MOCK_TRANSACTIONS, Some(String::from("1")))?;
    let mock2 =
        excel::test_setup::create_mock_transactions(MOCK_TRANSACTIONS, Some(String::from("2")))?;
    c.bench_function("remove_duplicates", |b| {
        b.iter(|| {
            // kinda heavy with the clone operator so might not be the fastest possible
            let _ = excel::lib::remove_duplicates(
                excel::workbook::WorkbookInfo::clone(&mock1.info)
                    .map_err(|e| e)
                    .unwrap(),
                excel::workbook::WorkbookInfo::clone(&mock2.info)
                    .map_err(|e| e)
                    .unwrap(),
            );
        })
    });
    Ok(())
}

criterion_group! {
    name = benches_creating_mock_transactions;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().significance_level(0.1).sample_size(10);
    targets = create_mock_transactions_benchmark
}

criterion_group! {
    name = benches_excel;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().significance_level(0.1).sample_size(10);
    targets = write_benchmark
}

criterion_group!{
    name = benches_remove_duplicates;
    config = Criterion::default().significance_level(0.1).sample_size(10);
    targets = remove_duplicates_benchmark
}

criterion_main!(benches_remove_duplicates);
