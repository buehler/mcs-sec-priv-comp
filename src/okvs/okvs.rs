use ark_test_curves::bls12_381::Fr;

use crate::data::point::Point;

pub trait OKVS {
    fn decode(&self, key: impl Into<Fr>) -> Point;
}
