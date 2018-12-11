use blake2::{Blake2s, Digest};
use blake2b_simd::Params;
use pairing::bls12_381::Fr;

use crate::hasher::{Blake2sHasher, Hasher};

/// Key derivation function, based on pedersen hashing.
pub fn kdf(data: &[u8], m: usize) -> Fr {
    assert_eq!(
        data.len(),
        32 * (1 + m),
        "invalid input length: data.len(): {} m: {}",
        data.len(),
        m
    );

    let mut hasher = Params::new()
        .hash_length(32)
        .inner_hash_length(32)
        .to_state();

    let hashed = hasher.update(data).finalize();

    let mut res = <Blake2sHasher as Hasher>::Domain::default();
    res.0.copy_from_slice(&hashed.as_bytes()[..32]);
    res.trim_to_fr32();
    res.into()
}

#[cfg(test)]
mod tests {
    use super::kdf;
    use crate::fr32::bytes_into_fr;
    use pairing::bls12_381::Bls12;

    #[test]
    fn kdf_valid_block_len() {
        let m = 1;
        let size = 32 * (1 + m);

        let data = vec![1u8; size];
        let expected = bytes_into_fr::<Bls12>(
            &mut vec![
                220, 60, 76, 126, 119, 247, 67, 162, 98, 94, 119, 28, 247, 18, 71, 208, 167, 72,
                33, 85, 59, 56, 96, 13, 9, 67, 49, 109, 95, 246, 152, 63,
            ]
            .as_slice(),
        )
        .unwrap();

        let res = kdf(&data, m);
        assert_eq!(res, expected);
    }

    #[test]
    #[should_panic]
    fn kdf_invalid_block_len() {
        let data = vec![2u8; 1234];

        kdf(&data, 44);
    }
}
