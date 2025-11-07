use crate::primitives::mkhs::{Mkhs, Signature};
use ark_bls12_381::Fr;
use num_traits::Zero;
use rayon::prelude::*;

pub struct Aggregator {
    mkhs: Mkhs,
}

impl Aggregator {
    pub const fn new(mkhs: Mkhs) -> Self {
        Self { mkhs }
    }

    pub fn aggregate_data(&self, data: &[Vec<Vec<Fr>>]) -> Vec<Vec<Fr>> {
        if data.is_empty() {
            return vec![];
        }

        let rows = data[0].len();
        let cols = data[0][0].len();

        let mut result = vec![vec![Fr::zero(); cols]; rows];

        for matrix in data {
            for i in 0..rows {
                for j in 0..cols {
                    result[i][j] += matrix[i][j];
                }
            }
        }

        result
    }

    pub fn aggregate(&self, signatures: &[Vec<Signature>]) -> Vec<Signature> {
        let signatures_t: Vec<Vec<Signature>> = transpose_dataset(signatures);
        signatures_t
            .par_iter()
            .map(|col| self.mkhs.eval(col))
            .collect()
    }
}

fn transpose_dataset<T>(dataset: &[Vec<T>]) -> Vec<Vec<T>>
where
    T: Send + Sync + Clone,
{
    (0..dataset[0].len())
        .map(|col| dataset.par_iter().map(|row| row[col].clone()).collect())
        .collect()
}
