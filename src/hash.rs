use std::collections::HashSet;

use crate::data::point::Point;

pub(crate) fn create_bins(points: &[Point], delta: u64) -> HashSet<Point> {
    let mut bins = HashSet::new();

    for v in points {
        let bin = create_bin(v, delta);
        bins.insert(bin);
    }

    bins
}

/// "H_2": This computes the bin for a given point.
/// This implementation uses the d_infinity metric. As such,
/// the bin is the floor of the y-coordinate divided by 2 * delta.
pub(crate) fn create_bin(point: &Point, delta: u64) -> Point {
    let bin_y = point.y / (2 * delta);
    Point::new(point.x, bin_y)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::point::Point;

    #[test]
    fn test_create_bin() {
        let point = Point::new(1u64, 8u64);
        let delta = 2;
        let bin = create_bin(&point, delta);
        assert_eq!(bin, Point::new(1u64, 2u64));
    }

    #[test]
    fn test_create_bins() {
        let points = vec![Point::new(1u64, 8u64), Point::new(1u64, 12u64)];
        let delta = 2;
        let bins = create_bins(&points, delta);
        let expected_bins: HashSet<Point> = vec![Point::new(1u64, 2u64), Point::new(1u64, 3u64)]
            .into_iter()
            .collect();
        assert_eq!(expected_bins, bins);
    }

    #[test]
    fn test_points_in_same_bin() {
        let point1 = Point::new(1u64, 8u64);
        let point2 = Point::new(1u64, 9u64);
        let delta = 2;
        let bin1 = create_bin(&point1, delta);
        let bin2 = create_bin(&point2, delta);
        assert_eq!(bin1, bin2);
    }
}
