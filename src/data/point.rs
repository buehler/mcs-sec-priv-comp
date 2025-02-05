#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Point {
    pub(crate) x: u64,
    pub(crate) y: u64,
}

impl Point {
    pub fn new(x: impl Into<u64>, y: impl Into<u64>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

impl<K, V> From<(K, V)> for Point
where
    K: Into<u64>,
    V: Into<u64>,
{
    fn from((x, y): (K, V)) -> Self {
        Self::new(x, y)
    }
}

impl<K, V> From<&(K, V)> for Point
where
    K: Into<u64> + Copy,
    V: Into<u64> + Copy,
{
    fn from(pair: &(K, V)) -> Self {
        Self::new(pair.0, pair.1)
    }
}
