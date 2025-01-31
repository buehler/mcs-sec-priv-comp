use crate::data::point::Point;

use super::OKVS;
use ark_ff::{Field, Zero};
use ark_poly::univariate::DensePolynomial;
use ark_poly::{DenseUVPolynomial, Polynomial};
use ark_test_curves::bls12_381::Fr;
use std::collections::HashSet;

pub(super) struct LagrangePolynomialOKVS(DensePolynomial<Fr>);

impl LagrangePolynomialOKVS {
    pub(super) fn encode(data: &HashSet<Point>) -> Self {
        let mut poly = DensePolynomial::zero();
        let points = data.iter().map(|p| (p.x, p.y)).collect::<Vec<(Fr, Fr)>>();
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
    fn decode(&self, key: impl Into<Fr>) -> Point {
        let key = key.into();
        Point {
            x: key,
            y: self.0.evaluate(&key),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::point::Point;
    use ark_ff::AdditiveGroup;
    use ark_test_curves::bls12_381::Fr;

    #[test]
    fn test_encode() {
        let data: HashSet<Point> = vec![
            Point::new(Fr::from(1), Fr::from(2)),
            Point::new(Fr::from(2), Fr::from(16)),
            Point::new(Fr::from(3), Fr::from(6)),
            Point::new(Fr::from(4), Fr::from(28)),
            Point::new(Fr::from(5), Fr::from(10)),
            Point::new(Fr::from(6), Fr::from(555)),
            Point::new(Fr::from(7), Fr::from(7777)),
            Point::new(Fr::from(8), Fr::from(42)),
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
            Point::new(Fr::from(1), Fr::from(2)),
            Point::new(Fr::from(2), Fr::from(16)),
            Point::new(Fr::from(3), Fr::from(6)),
            Point::new(Fr::from(4), Fr::from(28)),
            Point::new(Fr::from(5), Fr::from(10)),
            Point::new(Fr::from(16), Fr::from(555)),
            Point::new(Fr::from(7), Fr::from(7777)),
            Point::new(Fr::from(8), Fr::from(42)),
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
            Point::new(Fr::from(1), Fr::from(2)),
            Point::new(Fr::from(2), Fr::from(16)),
            Point::new(Fr::from(3), Fr::from(6)),
            Point::new(Fr::from(4), Fr::from(28)),
            Point::new(Fr::from(5), Fr::from(10)),
            Point::new(Fr::from(16), Fr::from(555)),
            Point::new(Fr::from(7), Fr::from(7777)),
            Point::new(Fr::from(8), Fr::from(42)),
        ]
        .into_iter()
        .collect();
        let okvs = LagrangePolynomialOKVS::encode(&data);

        let invalid_key = Fr::from(42);
        let decoded_point = okvs.decode(invalid_key);
        assert_ne!(
            decoded_point.y,
            Fr::ZERO,
            "Decoded value for an invalid key should not be zero (random value expected)."
        );
    }
}
