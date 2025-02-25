use crate::data::point::Point;

use crate::okvs::{Encoder, Store};
use ark_ff::{Field, PrimeField, Zero};
use ark_poly::univariate::DensePolynomial;
use ark_poly::{DenseUVPolynomial, Polynomial};
use ark_test_curves::bls12_381::Fr;
use num_bigint::BigUint;
use rand_chacha::rand_core::{RngCore, SeedableRng};

pub struct LagrangePolynomialOKVSBuilder<const N: usize> {

}

pub struct LagrangePolynomialOKVS<const N: usize>(pub(crate) [DensePolynomial<Fr>; N]);

impl<const N: usize> Encoder<N> for LagrangePolynomialOKVS<N> {
    fn encode(data: &[Vec<Point>; N]) -> impl Store<N> {
        let mut polynomials = core::array::from_fn(|_| DensePolynomial::zero().clone());
        for dimension in 0..N {
            let dimension_data = &data[dimension];
            let poly = Self::interpolate(dimension_data);
            polynomials[dimension] = poly;
        }

        Self(polynomials)
    }
}

impl<const N: usize> Store<N> for LagrangePolynomialOKVS<N> {
    fn decode(&self, key: impl Into<u64>) -> [u64; N] {
        let mut rnd = rand_chacha::ChaCha20Rng::from_os_rng();
        let mut result = [0; N];
        let key = key.into();
        for dimension in 0..N {
            let poly = &self.0[dimension];
            let p = Fr::from(key);
            let y = poly.evaluate(&p);
            let y = BigUint::from(y.into_bigint()).to_u64_digits();
            let y = if y.is_empty() { rnd.next_u64() } else { y[0] };
            result[dimension] = y;
        }

        result
    }
}

impl<const N: usize> LagrangePolynomialOKVS<N> {
    fn interpolate(data: &[Point]) -> DensePolynomial<Fr> {
        let mut poly = DensePolynomial::zero();
        let points = data
            .iter()
            .map(|p| (Fr::from(p.x), Fr::from(p.y)))
            .collect::<Vec<(Fr, Fr)>>();
        let n = points.len();

        for i in 0..n {
            let (x_i, y_i) = points[i];
            let mut l_i = DensePolynomial::from_coefficients_vec(vec![Fr::ONE]);

            for j in 0..n {
                if i != j {
                    let (x_j, _) = points[j];
                    let denominator = x_i - x_j;
                    let denominator_inv = denominator.inverse().unwrap();
                    let term = DensePolynomial::from_coefficients_vec(vec![
                        -x_j * denominator_inv,
                        denominator_inv,
                    ]);
                    l_i = &l_i * &term;
                }
            }

            l_i = &l_i * y_i;
            poly = &poly + &l_i;
        }

        poly
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::point::Point;

    #[test]
    fn test_encode() {
        let data = [vec![
            Point::new(1u64, 2u64),
            Point::new(2u64, 16u64),
            Point::new(3u64, 6u64),
            Point::new(4u64, 28u64),
            Point::new(5u64, 10u64),
            Point::new(6u64, 555u64),
            Point::new(7u64, 7777u64),
            Point::new(8u64, 42u64),
        ]];
        let okvs = LagrangePolynomialOKVS::encode(&data);

        // Verify that the OKVS is constructed correctly
        assert!(
            okvs.0[0].degree() >= data.len() - 1,
            "Polynomial degree should accommodate all key-value pairs."
        );
    }

    #[test]
    fn test_decode_valid_key() {
        let data = [vec![
            Point::new(1u64, 2u64),
            Point::new(2u64, 16u64),
            Point::new(3u64, 6u64),
            Point::new(4u64, 28u64),
            Point::new(5u64, 10u64),
            Point::new(6u64, 555u64),
            Point::new(7u64, 7777u64),
            Point::new(8u64, 42u64),
        ]];
        let okvs = LagrangePolynomialOKVS::encode(&data);

        for point in &data[0] {
            let decoded_point = okvs.decode(point.x);
            assert_eq!(
                decoded_point[0], point.y,
                "Decoded value should match the encoded value."
            );
        }
    }

    #[test]
    fn test_decode_invalid_key() {
        let data = [vec![
            Point::new(1u64, 2u64),
            Point::new(2u64, 16u64),
            Point::new(3u64, 6u64),
            Point::new(4u64, 28u64),
            Point::new(5u64, 10u64),
            Point::new(6u64, 555u64),
            Point::new(7u64, 7777u64),
            Point::new(8u64, 42u64),
        ]];
        let okvs = LagrangePolynomialOKVS::encode(&data);

        let invalid_key = 42u64;
        let decoded_point = okvs.decode(invalid_key);
        assert_ne!(
            decoded_point[0], 0,
            "Decoded value for an invalid key should not be zero (random value expected)."
        );
    }
}
