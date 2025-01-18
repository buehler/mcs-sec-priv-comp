use std::collections::HashMap;

use ark_ff::{AdditiveGroup, UniformRand};
use ark_poly::{
    univariate::DensePolynomial, DenseUVPolynomial, EvaluationDomain, GeneralEvaluationDomain,
    Polynomial,
};
use ark_test_curves::bls12_381::Fr;
use rand::SeedableRng;

pub struct OKVS(DensePolynomial<Fr>);

impl OKVS {
    /// asdf
    pub fn encode(data: HashMap<u64, u64>) -> Self {
        // Get highest key to determine the size of the domain
        let highest_key = *data.keys().max().unwrap();
        let domain = GeneralEvaluationDomain::<Fr>::new(highest_key as usize).unwrap();

        // Initialize evaluations vector with zeros
        let mut evals = vec![Fr::ZERO; domain.size() + 1];

        // Fill in evaluations with data, for all keys in the domain
        // that are not found, fill in random values
        let mut rng = rand_chacha::ChaCha20Rng::from_entropy();
        for idx in 0..=domain.size() {
            let key = idx as u64;
            if let Some(&value) = data.get(&key) {
                evals[idx] = Fr::from(value);
            } else {
                evals[idx] = Fr::rand(&mut rng);
            }
        }

        // From a vector of evaluations, we can recover the polynomial with the IFFT
        // https://en.wikipedia.org/wiki/Fast_Fourier_transform
        let coeffs = domain.ifft(&evals);

        // Construct the polynomial from the coefficients
        let poly = DensePolynomial::from_coefficients_vec(coeffs);

        Self(poly)
    }

    pub fn decode(&self, key: &Fr) -> Fr {
        self.0.evaluate(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_okvs() {
        let data = [(1, 2), (2, 3), (32, 4)]
            .iter()
            .cloned()
            .collect::<HashMap<u64, u64>>();
        let okvs = OKVS::encode(data);
    }
}
