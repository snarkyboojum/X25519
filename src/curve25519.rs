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

fn add() {}

fn sub() {}

fn mul() {}
