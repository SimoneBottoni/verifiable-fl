use crate::primitives::mkhs::{KeyPair, Mkhs, Signature, PK};
use crate::util::dataset::Dataset;
use ark_bls12_381::Fr;
use rayon::prelude::*;
use std::collections::HashMap;

pub struct Client {
    pub id: u64,
    pub mkhs: Mkhs,
    pub key_pair: KeyPair,
}

impl Client {
    pub fn build(id: u64, mkhs: Mkhs) -> Self {
        let key_pair = mkhs.generate_keys(id);
        Self { id, mkhs, key_pair }
    }

    pub fn train(&self, n_col: usize, n_row: usize, decimals: u32) -> Dataset {
        Dataset::build(n_col, n_row, decimals)
    }

    pub fn prove(&self, messages: &[Vec<Fr>]) -> Vec<Signature> {
        messages
            .par_iter()
            .map(|row| self.mkhs.sign(&self.key_pair.sk, row))
            .collect()
    }

    pub fn verify(
        &self,
        pks: &HashMap<u64, PK>,
        aggregated_data: &[Vec<Fr>],
        aggregated_signatures: &[Signature],
    ) -> anyhow::Result<()> {
        aggregated_signatures
            .par_iter()
            .zip(aggregated_data)
            .try_for_each(|(signature, data)| self.mkhs.verify(pks, data, signature))
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::mkhs::Mkhs;
    use crate::system::client::Client;
    use std::collections::HashMap;

    #[test]
    fn test_client() {
        let rows = 10;
        let cols = 1;

        let decimals = 4;

        let mkhs = Mkhs::setup(1, 10);

        let client = Client::build(1, mkhs);
        let client_pk = client.key_pair.pk.clone();

        let pks = HashMap::from([(1, client_pk)]);

        let dataset_t = client.train(cols, rows, decimals).transpose().fr();

        let signature = client.prove(&dataset_t);

        let check = client.verify(&pks, &dataset_t, &signature);
        assert!(check.is_ok());
    }
}
