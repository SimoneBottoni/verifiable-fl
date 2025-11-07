use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator};
use rayon::prelude::*;
use std::collections::HashMap;
use verifiable_fl::primitives::mkhs::{Mkhs, PK};
use verifiable_fl::system::aggregator::Aggregator;
use verifiable_fl::system::client::Client;

pub struct Parameters {
    pub cols: usize,
    pub decimals: Vec<u32>,
    pub rows: Vec<usize>,
    pub clients: Vec<usize>,
}

impl Parameters {
    pub fn build() -> Self {
        let decimals = vec![4];
        let rows = vec![500000, 750000, 1000000, 5000000, 10000000];
        let clients = vec![2, 5, 10, 20, 50, 100, 500, 1000];

        let cols = 10;
        let rows = rows.par_iter().map(|v| v / cols).collect();

        Self {
            cols,
            decimals,
            rows,
            clients,
        }
    }
}

pub struct Setup {
    pub clients: Vec<Client>,
    pub pks: HashMap<u64, PK>,
    pub aggregator: Aggregator,
}

impl Setup {
    pub fn build(n_client: usize, n_row: usize) -> Self {
        let mkhs = Mkhs::setup(n_client, n_row);

        let clients: Vec<Client> = (1..=n_client)
            .into_par_iter()
            .map(|id| Client::build(id as u64, mkhs.clone()))
            .collect();

        let aggregator = Aggregator::new(mkhs.clone());

        let pks: HashMap<u64, PK> = HashMap::from_par_iter(
            clients
                .par_iter()
                .map(|client| (client.id, client.key_pair.pk.clone())),
        );

        Self {
            clients,
            pks,
            aggregator,
        }
    }
}
