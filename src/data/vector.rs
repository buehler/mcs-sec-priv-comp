use ark_test_curves::bls12_381::Fr;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Vector(pub(crate) Vec<Fr>);

macro_rules! impl_from_num {
    ($($t:ty),*) => {
        $(
            impl From<$t> for Vector {
                fn from(value: $t) -> Self {
                    Vector(vec![Fr::from(value)])
                }
            }

            impl From<Vec<$t>> for Vector {
                fn from(data: Vec<$t>) -> Self {
                    Vector(data.into_iter().map(Fr::from).collect())
                }
            }

            impl From<&[$t]> for Vector {
                fn from(data: &[$t]) -> Self {
                    Vector(data.iter().map(|&x| Fr::from(x)).collect())
                }
            }

            impl<const N: usize> From<&[$t; N]> for Vector {
                fn from(data: &[$t; N]) -> Self {
                    Vector(data.iter().map(|&x| Fr::from(x)).collect())
                }
            }
        )*
    }
}

impl_from_num!(u8, u16, u32, u64, u128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let v1 = Vector::from(1u8);
        let v2 = Vector::from(vec![1u8, 2u8, 3u8]);
        let v3 = Vector::from(vec![1u8, 2u8, 3u8].as_slice());
        let v4 = Vector::from(&[1u8, 2u8, 3u8]);

        let r = Vector(vec![Fr::from(1u8), Fr::from(2u8), Fr::from(3u8)]);
        assert_eq!(v1, Vector(vec![Fr::from(1u8)]));
        assert_eq!(v2, r);
        assert_eq!(v3, r);
        assert_eq!(v4, r);
    }
}
