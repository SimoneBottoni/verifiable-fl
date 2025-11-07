use crate::utils::{Parameters, Setup};
use criterion::measurement::WallTime;
use criterion::{criterion_group, criterion_main, BenchmarkGroup, BenchmarkId, Criterion};
use rayon::prelude::*;
use verifiable_fl::primitives::mkhs::Signature;

mod utils;

fn aggregator(
    rows: usize,
    cols: usize,
    n_clients: usize,
    decimals: u32,
    group: &mut BenchmarkGroup<WallTime>,
) {
    // Setup
    let setup = Setup::build(n_clients, rows);

    // Computing signatures
    let signatures: Vec<Vec<Signature>> = setup
        .clients
        .par_iter()
        .map(|client| {
            let dataset_t = client.train(cols, rows, decimals).transpose().fr();
            client.prove(&dataset_t)
        })
        .collect();

    // Aggregating signatures
    group.bench_function(
        BenchmarkId::new(
            "aggregating_signatures",
            format!("clients: {} - rows: {}", n_clients, rows),
        ),
        |b| {
            b.iter(|| {
                setup.aggregator.aggregate(&signatures);
            })
        },
    );
}

pub fn bench_aggregator(c: &mut Criterion) {
    let mut group = c.benchmark_group("aggregator");
    group.sample_size(10);

    let parameters = Parameters::build();

    for n_clients in parameters.clients {
        aggregator(
            10,
            parameters.cols,
            n_clients,
            parameters.decimals[0],
            &mut group,
        )
    }
    group.finish();
}

criterion_group!(benches, bench_aggregator);
criterion_main!(benches);
