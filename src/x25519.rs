use crate::arithmetic;

fn encode_ucoord(u_coord: [u8; 32]) {}

fn decode_ucoord(u_coord: [u8; 32]) -> u64 {
    // implicit assumption that we're doing 255 bits
    // see p7 of https://tools.ietf.org/html/rfc7748#section-5

    let mut u_list = u_coord;
    u_list[31] &= (1 << 255 % 8) - 1;

    decode_le(u_list)
}

fn decode_le(bytes: [u8; 32]) -> u64 {
    let mut sum = 0;
    for (i, b) in bytes.iter().enumerate() {
        sum += (*b << 8 * i) as u64;
    }

    sum
}

// decode 32 byte scalars for X25519
fn decode_scalar(k: [u8; 32]) -> u64 {
    0
}

// u is little endian
// must mask the msb in the final byte - if little endian, msb should be
// right hand most byte / bit
// if u is >= 2^255 - 19 but <= 2^255 - 1, then we -> modulo 2^255 - 19?
//
// k: is the scalar
// u: is the u-coordinate
// v: is the output coordinate
pub fn x25519(k: [u8; 32], u: &[u8; 32], v: &mut [u8; 32]) {}

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

        // call the x25519 function ...

        input_scalar =
            Vec::from_hex("4b66e9d4d1b4673c5ad22691957d6af5c11b6421e0ea01d42ca4169e7918ba0d");
        input_ucoord =
            Vec::from_hex("e5210f12786811d3f4b7959d0538ae2c31dbe7106fc03c3efc4cd549c715a493");
        output_ucoord =
            Vec::from_hex("95cbde9476e8907d7aade45cb4b873f88b595a68799fa152e6f8f7647aac7957")
    }

    #[test]
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

    #[test]
    fn test_ecdhe_x25519() {
        use hex::FromHex;
        let alice_prv_key =
            Vec::from_hex("77076d0a7318a57d3c16c17251b26645df4c2f87ebc0992ab177fba51db92c2a");
        let bob_prv_key =
            Vec::from_hex("5dab087e624a8a4b79e17f8b83800ee66f3bb1292618b6fd1c2f8b27ff88e0eb");

        let expected_bob_pub_key =
            Vec::from_hex("de9edb7d7b7dc1b4d35b61c2ece435373f8343c85b78674dadfc7e146f882b4f");
        let expected_alice_pub_key =
            Vec::from_hex("8520f0098930a754748b7ddcb43ef75a0dbf3a0d26381af4eba4a98eaa9b4e6a");
        let expected_shared_secret_key =
            Vec::from_hex("4a5d9d5ba4ce2de1728e3bf480350f25e07e21c947d19e3376f09b3c1e161742");

        // use x25519 etc to compute all of this
        // ...
    }
}
