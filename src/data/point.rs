#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Point {
    pub(crate) x: u128,
    pub(crate) y: u128,
}

impl Point {
    pub fn new(x: impl Into<u128>, y: impl Into<u128>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

impl<K, V> From<(K, V)> for Point
where
    K: Into<u128>,
    V: Into<u128>,
{
    fn from((x, y): (K, V)) -> Self {
        Self::new(x, y)
    }
}

impl<K, V> From<&(K, V)> for Point
where
    K: Into<u128> + Copy,
    V: Into<u128> + Copy,
{
    fn from(pair: &(K, V)) -> Self {
        Self::new(pair.0, pair.1)
    }
}
