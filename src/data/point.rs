use ark_test_curves::bls12_381::Fr;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Point{
    pub(crate) x: Fr,
    pub(crate) y: Fr,
}

impl Point {
    pub fn new(x: impl Into<Fr>, y: impl Into<Fr>) -> Self {
        Self { x: x.into(), y: y.into() }
    }
}
