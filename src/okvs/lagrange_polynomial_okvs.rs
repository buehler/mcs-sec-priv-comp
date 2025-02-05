use crate::data::point::Point;

use super::OKVS;
use ark_ff::{Field, PrimeField, Zero};
use ark_poly::univariate::DensePolynomial;
use ark_poly::{DenseUVPolynomial, Polynomial};
use ark_test_curves::bls12_381::Fr;
use num_bigint::BigUint;
use std::collections::HashSet;

pub(super) struct LagrangePolynomialOKVS(DensePolynomial<Fr>);

impl LagrangePolynomialOKVS {
    pub(super) fn encode(data: &HashSet<Point>) -> Self {
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

        Self(poly)
    }
}

impl OKVS for LagrangePolynomialOKVS {
    fn decode(&self, key: impl Into<u128>) -> Point {
        let key = key.into();
        let p = Fr::from(key);
        let y = self.0.evaluate(&p);
        let y = BigUint::from(y.into_bigint()).to_u64_digits();
        let y = y[0];

        Point {
            x: key,
            y: y.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::point::Point;

    #[test]
    fn test_encode() {
        let data: HashSet<Point> = vec![
            Point::new(1u128, 2u128),
            Point::new(2u128, 16u128),
            Point::new(3u128, 6u128),
            Point::new(4u128, 28u128),
            Point::new(5u128, 10u128),
            Point::new(6u128, 555u128),
            Point::new(7u128, 7777u128),
            Point::new(8u128, 42u128),
        ]
        .into_iter()
        .collect();
        let okvs = LagrangePolynomialOKVS::encode(&data);

        // Verify that the OKVS is constructed correctly
        assert!(
            okvs.0.degree() >= data.len() - 1,
            "Polynomial degree should accommodate all key-value pairs."
        );
    }

    #[test]
    fn test_decode_valid_key() {
        let data: HashSet<Point> = vec![
            Point::new(1u128, 2u128),
            Point::new(2u128, 16u128),
            Point::new(3u128, 6u128),
            Point::new(4u128, 28u128),
            Point::new(5u128, 10u128),
            Point::new(6u128, 555u128),
            Point::new(7u128, 7777u128),
            Point::new(8u128, 42u128),
        ]
        .into_iter()
        .collect();
        let okvs = LagrangePolynomialOKVS::encode(&data);

        for point in &data {
            let decoded_point = okvs.decode(point.x);
            assert_eq!(
                decoded_point.y, point.y,
                "Decoded value should match the encoded value."
            );
        }
    }

    #[test]
    fn test_decode_invalid_key() {
        let data: HashSet<Point> = vec![
            Point::new(1u128, 2u128),
            Point::new(2u128, 16u128),
            Point::new(3u128, 6u128),
            Point::new(4u128, 28u128),
            Point::new(5u128, 10u128),
            Point::new(6u128, 555u128),
            Point::new(7u128, 7777u128),
            Point::new(8u128, 42u128),
        ]
        .into_iter()
        .collect();
        let okvs = LagrangePolynomialOKVS::encode(&data);

        let invalid_key = 42u128;
        let decoded_point = okvs.decode(invalid_key);
        assert_ne!(
            decoded_point.y, 0,
            "Decoded value for an invalid key should not be zero (random value expected)."
        );
    }
}
