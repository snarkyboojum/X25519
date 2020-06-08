/*

Curve25519 notes:

p - the prime number of the underlying field
GF(p) - finite field with p elements
A - element in GF(p), but not -2 or 2
d - a non-zero element of GF(p), not 1 (Edwards), and not -1 (twisted Edwards)
order - order of the prime-order subgroup
P - generator point over GF(p) of prime order
...

The prime 2^255 - 19 is ideal for performance.

See References[1] from p 7 for X25519 implementation notes.

*/

// would it be useful to have a method that is able to generate
// 32 random bytes?
fn generate_random_bytes() -> [u8; 32] {}

fn encode_ucoord() {}
fn decode_ucoord() {}
fn decode_le() {}

// decode 32 byte scalars for X25519
fn decode_scalar(k: [u8; 32]) -> u64 {
    0
}

//
fn x25519(k: [u8; 32], u: &[u8], v: &mut [u8]) {}

#[cfg(test)]
mod tests {
    // tests taken straight from p11, https://tools.ietf.org/html/rfc7748#ref-curve25519
    #[test]
    fn test_x25519() {
        use hex::FromHex;

        let mut input_scalar =
            Vec::from_hex("a546e36bf0527c9d3b16154b82465edd62144c0ac1fc5a18506a2244ba449ac4");
        let mut input_ucoord =
            Vec::from_hex("e6db6867583030db3594c1a424b15f7c726624ec26b3353b10a903a6d0ab1c4c");
        let mut output_ucoord =
            Vec::from_hex("c3da55379de9c6908e94ea4df28d084f32eccf03491c71f754b4075577a28552");

        input_scalar =
            Vec::from_hex("4b66e9d4d1b4673c5ad22691957d6af5c11b6421e0ea01d42ca4169e7918ba0d");
        input_ucoord =
            Vec::from_hex("e5210f12786811d3f4b7959d0538ae2c31dbe7106fc03c3efc4cd549c715a493");
        output_ucoord =
            Vec::from_hex("95cbde9476e8907d7aade45cb4b873f88b595a68799fa152e6f8f7647aac7957")
    }

    fn test_x25519_ktimes() {
        use hex::FromHex;

        let k = Vec::from_hex("0900000000000000000000000000000000000000000000000000000000000000");
        let u = Vec::from_hex("0900000000000000000000000000000000000000000000000000000000000000");

        for i in 1..=1_000_000 {
            if i == 1 {
                // after 1 iteration we should get
                // 422c8e7a6227d7bca1350b3e2bb7279f7897b87bb6854b783c60e80311ae3079
            } else if i == 1000 {
                // after 1,000 iterations we should get
                // 684cf59ba83309552800ef566f2f4d3c1c3887c49360e3875f2eb94d99532c51
            } else if i == 1_000_000 {
                // after 1,000,000 interations we should get
                // 7c3911e0ab2586fd864497297e575e6f3bc601c0883c30df5f4dd2d24f665424
            }
        }
    }
}
