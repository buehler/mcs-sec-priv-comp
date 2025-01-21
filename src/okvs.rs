use ark_ff::{Field, Zero};
use ark_poly::{univariate::DensePolynomial, DenseUVPolynomial, Polynomial};
use ark_test_curves::bls12_381::Fr;

pub struct OKVS(DensePolynomial<Fr>);

impl OKVS {
    /// ### Encode Function
    /// Encodes a set of key-value pairs into an Oblivious Key-Value Store (OKVS)
    /// using Lagrange interpolation over a finite field.
    ///
    /// The resulting polynomial is constructed such that it passes through all
    /// provided key-value points, allowing the values to be decoded from their
    /// corresponding keys.
    ///
    /// ### Arguments
    ///
    /// - `data`: A slice of key-value pairs, where:
    ///   - `K`: The type of the key, which must implement `Into<Fr>` and `Copy`.
    ///   - `V`: The type of the value, which must implement `Into<Fr>` and `Copy`.
    ///
    /// ### Returns
    ///
    /// - An `OKVS` instance containing a polynomial that represents the encoded data.
    ///
    /// ### Lagrange Interpolation Details
    ///
    /// The function constructs the polynomial \( P(x) \) of degree `n-1` (where `n`
    /// is the number of key-value pairs) using Lagrange basis polynomials:
    ///
    /// \[
    /// P(x) = \sum_{i=0}^{n-1} y_i \cdot L_i(x)
    /// \]
    ///
    /// Where:
    /// - \( L_i(x) = \prod_{j=0, j \neq i}^{n-1} \frac{x - x_j}{x_i - x_j} \)
    /// - \( y_i \) is the value corresponding to \( x_i \).
    ///
    /// ### Panics
    ///
    /// - If any denominator in the Lagrange interpolation process (i.e., \( x_i - x_j \)) is zero.
    /// - If the number of points exceeds the capacity of the polynomial representation.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use ark_ff::{Field, Zero};
    /// use ark_poly::{univariate::DensePolynomial, DenseUVPolynomial, Polynomial};
    /// use ark_test_curves::bls12_381::Fr;
    /// use fuzzy_psi::OKVS;
    ///
    /// let data = vec![
    ///     (1u64, 10u32),
    ///     (2u64, 20u32),
    ///     (3u64, 30u32),
    ///     (4u64, 40u32),
    /// ];
    ///
    /// // Encode the data into an OKVS
    /// let okvs = OKVS::encode(&data);
    ///
    /// // Decode a value using a key
    /// let key = 2u64;
    /// let decoded_value = okvs.decode(key);
    /// assert_eq!(decoded_value, Fr::from(20u32));
    /// ```
    ///
    /// ### Notes
    ///
    /// - The keys and values are internally converted into field elements of type `Fr`.
    /// - This function ensures that the encoded polynomial can accurately represent the provided data points.
    ///
    /// ### See Also
    ///
    /// - [`OKVS::decode`](#): Decodes a value from the polynomial using a given key.
    pub fn encode<K: Into<Fr> + Copy, V: Into<Fr> + Copy>(data: &[(K, V)]) -> Self {
        // Perform Lagrange interpolation
        let interpolating_poly = {
            let points = data
                .iter()
                .map(|(k, v)| ((*k).into(), (*v).into()))
                .collect::<Vec<(Fr, Fr)>>();
            let mut poly = DensePolynomial::zero();
            let n = points.len();

            for i in 0..n {
                let (x_i, y_i) = points[i];
                let mut l_i = DensePolynomial::from_coefficients_vec(vec![Fr::ONE]);

                for j in 0..n {
                    if i != j {
                        let (x_j, _) = points[j];
                        let denominator = x_i - x_j;
                        let denominator_inv = denominator.inverse().unwrap();
                        let term = DensePolynomial::from_coefficients_vec(vec![
                            -x_j * denominator_inv,
                            denominator_inv,
                        ]);
                        l_i = &l_i * &term;
                    }
                }

                l_i = &l_i * y_i;
                poly = &poly + &l_i;
            }

            poly
        };

        Self(interpolating_poly)
    }

    /// ### Decode Function
    /// Decodes the value corresponding to a given key from the Oblivious Key-Value Store (OKVS).
    ///
    /// The function evaluates the polynomial constructed during the encoding process
    /// at the given key to retrieve the corresponding value.
    ///
    /// ### Arguments
    ///
    /// - `key`: The key for which the value needs to be decoded. The key must:
    ///   - Implement `Into<Fr>` to allow conversion into the field element type.
    ///   - Implement `Copy` to ensure immutability during processing.
    ///
    /// ### Returns
    ///
    /// - The decoded value as an `Fr` field element. If the key was not part of the
    ///   original data, the returned value might not correspond to any meaningful data.
    ///
    /// ### Evaluation Details
    ///
    /// The function evaluates the polynomial \( P(x) \) at the given key:
    ///
    /// \[
    /// P(\text{key}) = \sum_{i=0}^{n-1} y_i \cdot L_i(\text{key})
    /// \]
    ///
    /// Where:
    /// - \( y_i \) are the original values corresponding to keys \( x_i \).
    /// - \( L_i(\text{key}) \) are the Lagrange basis polynomials evaluated at the given key.
    ///
    /// ### Panics
    ///
    /// - If the field element conversion from the key fails.
    /// - If the polynomial evaluation encounters any unexpected behavior.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use ark_ff::{Field, Zero};
    /// use ark_poly::{univariate::DensePolynomial, DenseUVPolynomial, Polynomial};
    /// use ark_test_curves::bls12_381::Fr;
    /// use fuzzy_psi::OKVS;
    ///
    /// let data = vec![
    ///     (1u64, 10u32),
    ///     (2u64, 20u32),
    ///     (3u64, 30u32),
    ///     (4u64, 40u32),
    /// ];
    ///
    /// // Encode the data into an OKVS
    /// let okvs = OKVS::encode(&data);
    ///
    /// // Decode a value using a key
    /// let key = 3u64;
    /// let decoded_value = okvs.decode(key);
    /// assert_eq!(decoded_value, Fr::from(30u32));
    ///
    /// // Decode a key that wasn’t in the original data
    /// let random_key = 42u64;
    /// let random_value = okvs.decode(random_key);
    /// println!("Decoded value for key 42: {}", random_value);
    /// ```
    ///
    /// ### Notes
    ///
    /// - The decoding process relies on the correctness of the polynomial constructed
    ///   during encoding. Any errors in encoding will propagate during decoding.
    /// - Decoding a key that was not part of the original data will return a value
    ///   that does not correspond to the initial dataset.
    pub fn decode<K: Into<Fr> + Copy>(&self, key: K) -> Fr {
        self.0.evaluate(&key.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ff::AdditiveGroup;
    use ark_test_curves::bls12_381::Fr;

    #[test]
    fn test_encode() {
        let data = vec![
            (1u64, 2u32),
            (2u64, 16u32),
            (3u64, 6u32),
            (4u64, 28u32),
            (5u64, 10u32),
            (6u64, 555u32),
            (7u64, 7777u32),
            (8u64, 42u32),
        ];
        let okvs = OKVS::encode(&data);

        // Verify that the OKVS is constructed correctly
        assert!(
            okvs.0.degree() >= data.len() - 1,
            "Polynomial degree should accommodate all key-value pairs."
        );
    }

    #[test]
    fn test_decode_valid_key() {
        let data = vec![
            (1u64, 2u32),
            (2u64, 16u32),
            (3u64, 6u32),
            (4u64, 28u32),
            (5u64, 10u32),
            (16u64, 555u32),
            (7u64, 7777u32),
            (8u64, 42u32),
        ];
        let okvs = OKVS::encode(&data);

        for (key, value) in data {
            let decoded_value = okvs.decode(key);
            assert_eq!(
                decoded_value,
                Fr::from(value),
                "Decoded value should match the encoded value."
            );
        }
    }

    #[test]
    fn test_decode_invalid_key() {
        let data = vec![
            (1u64, 2u32),
            (2u64, 16u32),
            (3u64, 6u32),
            (4u64, 28u32),
            (5u64, 10u32),
            (16u64, 555u32),
            (7u64, 7777u32),
            (8u64, 42u32),
        ];
        let okvs = OKVS::encode(&data);

        let decoded_value = okvs.decode(42);
        assert_ne!(
            decoded_value,
            Fr::ZERO,
            "Decoded value for an invalid key should not be zero (random value expected)."
        );
    }
}
