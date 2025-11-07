use crate::utils::{Parameters, Setup};
use criterion::measurement::WallTime;
use criterion::{criterion_group, criterion_main, BenchmarkGroup, BenchmarkId, Criterion};

mod utils;

fn client_verification(
    rows: usize,
    cols: usize,
    decimals: u32,
    group: &mut BenchmarkGroup<WallTime>,
) {
    let n_client = 2;
    let setup = Setup::build(n_client, rows);
    let clients = setup.clients;

    // Computing signatures
    let mut signatures = Vec::new();
    let mut datasets = Vec::new();

    for client in &clients {
        let dataset_t = client.train(cols, rows, decimals).transpose().fr();
        signatures.push(client.prove(&dataset_t));
        datasets.push(dataset_t);
    }

    // Aggregation
    let aggregated_dataset = setup.aggregator.aggregate_data(&datasets);
    let aggregated_signatures = setup.aggregator.aggregate(&signatures);

    group.bench_function(
        BenchmarkId::new(
            "verifying_signatures",
            format!("decimals: {} - rows: {}", decimals, rows),
        ),
        |b| b.iter(|| clients[0].verify(&setup.pks, &aggregated_dataset, &aggregated_signatures)),
    );
}

pub fn bench_client_v(c: &mut Criterion) {
    let mut group = c.benchmark_group("client_v");
    group.sample_size(10);

    let parameters = Parameters::build();

    for decimals in parameters.decimals {
        for n_row in &parameters.rows {
            client_verification(*n_row, parameters.cols, decimals, &mut group)
        }
    }
    group.finish();
}

criterion_group!(benches, bench_client_v);
criterion_main!(benches);
