use std::collections::HashSet;

fn save_sub(bin: u64, delta: u64) -> u64 {
    if delta >= bin {
        return 0;
    }
    bin - delta
}

/// "H_1": This computes all bins for a given set of points.
/// The bins are calculated using the d_infinity metric.
/// Also, all bins are returned that are within the distance of each point.
/// So, for all points, calculate all bins such that point - delta and point + delta are included.
pub fn create_bins(points: &[u64], delta: u64) -> HashSet<u64> {
    let mut bins = HashSet::new();

    for &v in points {
        for val in save_sub(v, delta)..=(v + delta) {
            let bin = create_bin(val, delta);
            bins.insert(bin);
        }
    }

    bins
}

/// "H_2": This computes the bin for a given point.
/// This implementation uses the d_infinity metric. As such,
/// the bin is the floor of the y-coordinate divided by 2 * delta.
pub fn create_bin(val: u64, delta: u64) -> u64 {
    val / (2 * delta)
}

/// "H_1^(-1)": Kind of inversion of H_1. This computes a set of items from a bin.
/// invert_bin(bin, points, delta) returns a list of points that would be in the bin.
pub fn invert_bin(bin: u64, points: &[u64], delta: u64) -> Vec<u64> {
    let mut items = HashSet::new();

    for &v in points {
        let bin_v = create_bin(v, delta);
        if bin_v == bin {
            items.insert(v);
        }
    }

    items.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_create_bin() {
        let value = 8u64;
        let delta = 2;
        let bin = create_bin(value, delta);
        // 8 / (2*2) = 8/4 = 2
        assert_eq!(bin, 2);
    }

    #[test]
    fn test_create_bins() {
        let points = vec![8u64, 12u64];
        let delta = 2;
        let bins = create_bins(&points, delta);
        // For 8: (6..=10) gives bins: 6/4=1, 7/4=1, 8/4=2, 9/4=2, 10/4=2 -> {1,2}
        // For 12: (10..=14) gives bins: 10/4=2, 11/4=2, 12/4=3, 13/4=3, 14/4=3 -> {2,3}
        // Merged bins: {1,2,3}
        let expected_bins: HashSet<u64> = vec![1, 2, 3].into_iter().collect();
        assert_eq!(expected_bins, bins);
    }

    #[test]
    fn test_points_in_same_bin() {
        let value1 = 8u64;
        let value2 = 9u64;
        let delta = 2;
        let bin1 = create_bin(value1, delta);
        let bin2 = create_bin(value2, delta);
        assert_eq!(bin1, bin2);
    }
}
