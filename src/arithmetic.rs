/*

Implementation of all elliptic curve primitives for curve 25519,
including arithmetic for performing operations on curves and so on.

Notes:

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

// p = 2^255 - 19;

// we're going to have to get good at arithmetic on 256 bit numbers:
// see, https://www.chosenplaintext.ca/articles/radix-2-51-trick.html
// and, https://github.com/dalek-cryptography/curve25519-dalek/blob/master/src/backend/serial/u64/scalar.rs

// use 5 x 64 bit unsigned ints to represent 256 bit integers,
// where each limb is 51 or 52 bits, and the highest bits in each are for carries
struct BigInt {
    l: [u64; 5],
}

// a BigInt can store up to 32 bytes, 256 bits
impl BigInt {
    // build a BigInt from an array of 32 bytes
    fn from(bytes: [u8; 32]) {}

    // shortcut for generating little endian bytes
    fn bytes(self, bytes: &mut [u8; 32]) {}

    // turn a BigInt into an array of little endian bytes
    fn le_bytes(self, bytes: &mut [u8; 32]) {}

    // turn a BigInt into an array of big endian bytes
    fn be_bytes(self, bytes: &mut [u8; 32]) {}
}

use std::ops;

impl ops::Add for BigInt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self { l: [0u64; 5] }
    }
}

impl ops::Sub for BigInt {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self { l: [0u64; 5] }
    }
}

impl ops::Mul for BigInt {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self { l: [0u64; 5] }
    }
}
