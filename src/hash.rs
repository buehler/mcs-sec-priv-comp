use ark_ff::PrimeField;
use ark_test_curves::bls12_381::Fr;
use num_bigint::BigUint;
use std::collections::HashSet;

use crate::data::point::Point;

pub(crate) fn create_bins(points: &[Point], delta: impl Into<Fr>) -> HashSet<Point> {
    let mut bins = HashSet::new();

    let delta = delta.into();
    for v in points {
        let bin = create_bin(v, delta);
        bins.insert(bin);
    }

    bins
}

pub(crate) fn create_bin(point: &Point, delta: impl Into<Fr>) -> Point {
    let two: Fr = Fr::from(2);
    let delta = BigUint::from((two * delta.into()).into_bigint());

    let y = BigUint::from(point.y.into_bigint());
    let bin_y = y / delta.clone();

    Point::new(point.x, Fr::from(bin_y))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::point::Point;
    use ark_test_curves::bls12_381::Fr;

    #[test]
    fn test_create_bin() {
        let point = Point::new(Fr::from(1), Fr::from(8));
        let delta = Fr::from(2);
        let bin = create_bin(&point, delta);
        assert_eq!(bin, Point::new(Fr::from(1), Fr::from(2)));
    }

    #[test]
    fn test_create_bins() {
        let points = vec![
            Point::new(Fr::from(1), Fr::from(8)),
            Point::new(Fr::from(1), Fr::from(12)),
        ];
        let delta = Fr::from(2);
        let bins = create_bins(&points, delta);
        let expected_bins: HashSet<Point> = vec![
            Point::new(Fr::from(1), Fr::from(2)),
            Point::new(Fr::from(1), Fr::from(3)),
        ]
        .into_iter()
        .collect();
        assert_eq!(expected_bins, bins);
    }

    #[test]
    fn test_points_in_same_bin() {
        let point1 = Point::new(Fr::from(1), Fr::from(8));
        let point2 = Point::new(Fr::from(1), Fr::from(8) + Fr::from(1));
        let delta = Fr::from(2);
        let bin1 = create_bin(&point1, delta);
        let bin2 = create_bin(&point2, delta);
        assert_eq!(bin1, bin2);
    }
}
