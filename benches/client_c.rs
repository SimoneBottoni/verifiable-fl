use crate::utils::Parameters;
use criterion::measurement::WallTime;
use criterion::{criterion_group, criterion_main, BenchmarkGroup, BenchmarkId, Criterion};
use lz4_flex::compress_prepend_size;
use std::fs::File;
use std::io::Write;

mod utils;

fn client_computation(
    rows: usize,
    cols: usize,
    decimals: u32,
    group: &mut BenchmarkGroup<WallTime>,
) {
    let n_client = 1;
    let setup = utils::Setup::build(n_client, rows);
    let client = setup.clients.first().unwrap();

    let dataset_t = client.train(cols, rows, decimals).transpose().fr();

    // Computing signatures
    group.bench_function(
        BenchmarkId::new(
            "computing_signatures",
            format!("decimals: {} - rows: {}", decimals, rows),
        ),
        |b| {
            b.iter(|| {
                client.prove(&dataset_t);
            })
        },
    );
    let signature = client.prove(&dataset_t);

    let serialized = bincode::serialize(&signature).expect("Failed to serialize");
    let compressed = compress_prepend_size(&serialized);

    let mut file = File::create(format!("proof_decimals_{}_rows_{}.out", decimals, rows)).unwrap();
    file.write_all(&compressed).expect("Failed to write");
}

pub fn bench_client_c(c: &mut Criterion) {
    let mut group = c.benchmark_group("client_c");
    group.sample_size(10);

    let parameters = Parameters::build();

    for decimals in parameters.decimals {
        for n_row in &parameters.rows {
            client_computation(*n_row, parameters.cols, decimals, &mut group)
        }
    }
    group.finish();
}

criterion_group!(benches, bench_client_c);
criterion_main!(benches);
