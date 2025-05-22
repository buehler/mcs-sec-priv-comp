//! The code of this module is copied from https://github.com/felicityin/rb-okvs/tree/main.
//! Because the original authors have deprecated feature flags and old code, we cannot directly
//! use the dependency. Thus, the code is copied and modified to fit our needs.

use super::error::Result;
use sp_core::U256;

/// For small encoding sizes (i.e., high rate), one should try to fix small
/// choices of ϵ such as 0.03-0.05 In contrast, if one wishes for an
/// instantiation of RB-OKVS with fast encoding/decoding times, then one can
/// pick larger values of ϵ such as 0.07-0.1.
const EPSILON: f64 = 0.1;
const _LAMBDA: usize = 20;
const BAND_WIDTH: usize = 128; // ((LAMBDA as f64 + 15.21) / 0.2691) as usize = 130

/// RB-OKVS, Oblivious Key-Value Stores
pub struct RbOkvs {
    columns: usize,
    band_width: usize,
}

impl RbOkvs {
    pub fn new(kv_count: usize) -> RbOkvs {
        let columns = ((1.0 + EPSILON) * kv_count as f64) as usize;

        Self {
            columns,
            band_width: if BAND_WIDTH < columns {
                BAND_WIDTH
            } else {
                core::cmp::max(8, columns * 80 / 100)
            },
        }
    }
}

impl Okvs for RbOkvs {
    fn encode<K: OkvsK, V: OkvsV>(&self, input: Vec<Pair<K, V>>) -> Result<Encoding<V>> {
        let (matrix, start_pos, y) = self.create_sorted_matrix(input)?;
        utils::simple_gauss::<V>(y, matrix, start_pos, self.columns)
    }

    fn decode<V: OkvsV>(&self, encoding: &Encoding<V>, key: &impl OkvsK) -> V {
        let start = key.hash_to_index(self.columns - self.band_width);
        let band = key.hash_to_band(self.band_width);
        utils::inner_product(&band, &encoding[start..])
    }
}

impl RbOkvs {
    fn create_sorted_matrix<K: OkvsK, V: OkvsV>(
        &self,
        input: Vec<Pair<K, V>>,
    ) -> Result<(Vec<U256>, Vec<usize>, Vec<V>)> {
        let n = input.len();
        let mut start_pos: Vec<(usize, usize)> = vec![(0, 0); n];

        input.iter().enumerate().for_each(|(i, (k, _))| {
            start_pos[i] = (i, k.hash_to_index(self.columns - self.band_width))
        });

        utils::radix_sort(&mut start_pos, self.columns - self.band_width - 1);

        let mut matrix: Vec<U256> = vec![U256::default(); n];
        let mut start_ids: Vec<usize> = vec![0; n];
        let mut y: Vec<V> = vec![V::default(); n];

        // Generate binary matrix
        start_pos
            .into_iter()
            .enumerate()
            .for_each(|(k, (i, start))| {
                matrix[k] = input[i].0.hash_to_band(self.band_width);
                y[k] = input[i].1.to_owned();
                start_ids[k] = start;
            });

        Ok((matrix, start_ids, y))
    }
}

pub type Encoding<T> = Vec<T>;
pub type Pair<K, V> = (K, V);

pub trait Okvs {
    fn encode<K: OkvsK, V: OkvsV>(&self, input: Vec<Pair<K, V>>) -> Result<Encoding<V>>;
    fn decode<V: OkvsV>(&self, encoding: &Encoding<V>, key: &impl OkvsK) -> V;
}

pub trait OkvsK {
    fn hash_to_index(&self, range: usize) -> usize;
    fn hash_to_band(&self, band_width: usize) -> U256;
    fn to_bytes(&self) -> Vec<u8>;
}

pub trait OkvsV: Clone {
    fn default() -> Self;
    fn is_zero(&self) -> bool;
    fn xor(&self, other: &Self) -> Self;
    fn in_place_xor(&mut self, other: &Self);
}

#[derive(Clone)]
pub struct OkvsKey<const N: usize = 8>(pub [u8; N]);

impl<const N: usize> OkvsK for OkvsKey<N> {
    /// hash1(key) -> [0, range)
    fn hash_to_index(&self, range: usize) -> usize {
        let v = utils::blake2b::<8>(&self.to_bytes());
        usize::from_le_bytes(v) % range
    }

    /// hash2(key) -> {0, 1}^band_width
    fn hash_to_band(&self, band_width: usize) -> U256 {
        let mut v = utils::hash(&self.0, band_width / 8);
        v[0] |= 1;
        U256::from_little_endian(&v)
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.0.into()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OkvsValue<const N: usize = 8>(pub [u8; N]);

impl<const N: usize> OkvsV for OkvsValue<N> {
    fn default() -> Self {
        Self([0u8; N])
    }

    fn is_zero(&self) -> bool {
        for v in &self.0 {
            if *v != 0 {
                return false;
            }
        }
        true
    }

    fn xor(&self, other: &Self) -> Self {
        let mut result = [0u8; N];
        for (i, item) in self.0.iter().enumerate() {
            result[i] = item ^ other.0[i];
        }
        Self(result)
    }

    fn in_place_xor(&mut self, other: &Self) {
        for i in 0..self.0.len() {
            self.0[i] ^= other.0[i];
        }
    }
}

mod utils {
    use blake2::{Blake2b512, Digest};
    use sp_core::U256;

    use super::super::error::{Error, Result};

    /// Martin Dietzfelbinger and Stefan Walzer. Efficient Gauss Elimination for
    /// Near-Quadratic Matrices with One Short Random Block per Row, with
    /// Applications. In 27th Annual European Symposium on Algorithms (ESA 2019).
    /// Schloss Dagstuhl-Leibniz-Zentrum fuer Informatik, 2019.
    pub fn simple_gauss<V: super::OkvsV>(
        mut y: Vec<V>,
        mut bands: Vec<U256>,
        start_pos: Vec<usize>,
        cols: usize,
    ) -> Result<Vec<V>> {
        let rows = bands.len();
        assert_eq!(rows, start_pos.len());
        assert_eq!(rows, y.len());
        let mut pivot: Vec<usize> = vec![0; rows];

        for i in 0..rows {
            let y_i = y[i].clone();

            let first_one = bands[i].trailing_zeros() as usize;
            if first_one == 256 {
                return Err(Error::ZeroRow(i));
            }

            pivot[i] = first_one + start_pos[i];

            for k in (i + 1)..rows {
                if start_pos[k] > pivot[i] {
                    break;
                }
                if bit(&bands[k], pivot[i] - start_pos[k]) {
                    bands[k] = xor(bands[i], bands[k], first_one, pivot[i] - start_pos[k]);
                    y[k].in_place_xor(&y_i);
                }
            }
        }

        // back subsitution
        let mut x = vec![V::default(); cols]; // solution to Ax = y
        for i in (0..rows).rev() {
            x[pivot[i]] = inner_product::<V>(&bands[i], &x[start_pos[i]..]).xor(&y[i]);
        }
        Ok(x)
    }

    const MASK: [u64; 64] = [
        0x1,
        0x2,
        0x4,
        0x8,
        0x10,
        0x20,
        0x40,
        0x80,
        0x100,
        0x200,
        0x400,
        0x800,
        0x1000,
        0x2000,
        0x4000,
        0x8000,
        0x10000,
        0x20000,
        0x40000,
        0x80000,
        0x100000,
        0x200000,
        0x400000,
        0x800000,
        0x1000000,
        0x2000000,
        0x4000000,
        0x8000000,
        0x10000000,
        0x20000000,
        0x40000000,
        0x80000000,
        0x100000000,
        0x200000000,
        0x400000000,
        0x800000000,
        0x1000000000,
        0x2000000000,
        0x4000000000,
        0x8000000000,
        0x10000000000,
        0x20000000000,
        0x40000000000,
        0x80000000000,
        0x100000000000,
        0x200000000000,
        0x400000000000,
        0x800000000000,
        0x1000000000000,
        0x2000000000000,
        0x4000000000000,
        0x8000000000000,
        0x10000000000000,
        0x20000000000000,
        0x40000000000000,
        0x80000000000000,
        0x100000000000000,
        0x200000000000000,
        0x400000000000000,
        0x800000000000000,
        0x1000000000000000,
        0x2000000000000000,
        0x4000000000000000,
        0x8000000000000000,
    ];

    fn bit(u: &U256, index: usize) -> bool {
        if index < 64 {
            u.0[0] & MASK[index] != 0
        } else if index < 128 {
            u.0[1] & MASK[index - 64] != 0
        } else if index < 192 {
            u.0[2] & MASK[index - 128] != 0
        } else {
            u.0[3] & MASK[index - 192] != 0
        }
    }

    fn xor(a: U256, b: U256, start_a: usize, start_b: usize) -> U256 {
        match start_a.cmp(&start_b) {
            std::cmp::Ordering::Equal => b ^ a,
            std::cmp::Ordering::Less => {
                let diff = start_b - start_a;
                ((b >> diff) ^ a) << diff
            }
            std::cmp::Ordering::Greater => {
                let diff = start_a - start_b;
                ((b << diff) ^ a) >> diff
            }
        }
    }

    pub fn inner_product<V: super::OkvsV>(m: &U256, x: &[V]) -> V {
        let mut result = V::default();
        let bits = m.bits();

        if bits <= 64 {
            for i in 0..bits {
                if m.0[0] & MASK[i] != 0 {
                    result.in_place_xor(&x[i]);
                }
            }
            return result;
        }

        for i in 0..64 {
            if m.0[0] & MASK[i] != 0 {
                result.in_place_xor(&x[i]);
            }
        }

        let x64 = &x[64..];

        if bits <= 128 {
            for i in 0..bits - 64 {
                if m.0[1] & MASK[i] != 0 {
                    result.in_place_xor(&x64[i]);
                }
            }
            return result;
        }

        for i in 0..64 {
            if m.0[1] & MASK[i] != 0 {
                result.in_place_xor(&x64[i]);
            }
        }

        let x128 = &x[128..];

        if bits <= 192 {
            for i in 0..bits - 128 {
                if m.0[2] & MASK[i] != 0 {
                    result.in_place_xor(&x128[i]);
                }
            }
            return result;
        }

        for i in 0..64 {
            if m.0[2] & MASK[i] != 0 {
                result.in_place_xor(&x128[i]);
            }
        }

        let x192 = &x[192..];

        for i in 0..bits - 192 {
            if m.0[3] & MASK[i] != 0 {
                result.in_place_xor(&x192[i]);
            }
        }
        result
    }

    pub fn blake2b<const N: usize>(data: &[u8]) -> [u8; N] {
        use blake2::digest::{Update, VariableOutput};
        use blake2::Blake2bVar;
        assert!(N <= 64);

        let mut hasher = Blake2bVar::new(N).unwrap();
        hasher.update(data);
        let mut buf = [0u8; N];
        hasher.finalize_variable(&mut buf).unwrap();
        buf
    }

    pub fn hash<T: AsRef<[u8]>>(data: &T, to_bytes_size: usize) -> Vec<u8> {
        let mut hasher = Blake2b512::new();

        if to_bytes_size <= 64 {
            hasher.update(data);
            let res = hasher.finalize();
            return res[0..to_bytes_size].into();
        }

        let mut result = vec![];
        let mut last_length = to_bytes_size;
        let loop_count = (to_bytes_size + 63) / 64;

        for i in 0..loop_count {
            if i == loop_count - 1 {
                result.extend(hash(data, last_length));
            } else {
                result.extend(hash(data, 64));
                last_length -= 64;
            }
        }

        result
    }

    /// Sort by arr[i].1
    pub fn radix_sort(arr: &mut Vec<(usize, usize)>, max: usize) {
        let mut exp = 1;
        loop {
            if max / exp == 0 {
                break;
            }
            *arr = count_sort(arr, exp);
            exp *= 10;
        }
    }

    fn count_sort(arr: &Vec<(usize, usize)>, exp: usize) -> Vec<(usize, usize)> {
        let mut count = [0usize; 10];

        arr.iter().for_each(|(_, b)| count[(b / exp) % 10] += 1);

        for i in 1..10 {
            count[i] += count[i - 1];
        }

        let mut output = vec![(0usize, 0usize); arr.len()];

        arr.iter().rev().for_each(|(a, b)| {
            output[count[(b / exp) % 10] - 1] = (*a, *b);
            count[(b / exp) % 10] -= 1;
        });

        output
    }
}
