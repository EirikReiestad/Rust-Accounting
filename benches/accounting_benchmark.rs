use rust_accounting;

use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion
};

fn write_to_workbook_benchmark(c: &mut Criterion) {

}

criterion_group!(benches, write_to_workbook_benchmark);
criterion_main!(benches);