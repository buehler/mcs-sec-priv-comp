pub mod lagrange_polynomial_okvs;

use crate::data::point::Point;

pub trait Encoder<const N: usize> {
    fn encode(data: &[Vec<Point>; N]) -> impl Store<N>;
}

pub trait Store<const N: usize> {
    fn decode(&self, key: impl Into<u64>) -> [u64; N];
}
