use ark_test_curves::bls12_381::Fr;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Point {
    pub(crate) x: Fr,
    pub(crate) y: Fr,
}

impl Point {
    pub fn new(x: impl Into<Fr>, y: impl Into<Fr>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

impl<K, V> From<(K, V)> for Point
where
    K: Into<Fr>,
    V: Into<Fr>,
{
    fn from((x, y): (K, V)) -> Self {
        Self::new(x, y)
    }
}

impl<K, V> From<&(K, V)> for Point
where
    K: Into<Fr> + Copy,
    V: Into<Fr> + Copy,
{
    fn from(pair: &(K, V)) -> Self {
        Self::new(pair.0, pair.1)
    }
}
