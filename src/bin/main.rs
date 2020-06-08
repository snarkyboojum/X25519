fn main() {
    println!("Welcome to an X25519 implementation written in Rust!");

    // this is how you use the library
    // see p14, https://tools.ietf.org/html/rfc7748#section-6

    /*

        The X25519 function can be used in an Elliptic Curve Diffie-Hellman
       (ECDH) protocol as follows:

       Alice generates 32 random bytes in a[0] to a[31] and transmits K_A =
       X25519(a, 9) to Bob, where 9 is the u-coordinate of the base point
       and is encoded as a byte with value 9, followed by 31 zero bytes.

       Bob similarly generates 32 random bytes in b[0] to b[31], computes
       K_B = X25519(b, 9), and transmits it to Alice.

       Using their generated values and the received input, Alice computes
       X25519(a, K_B) and Bob computes X25519(b, K_A).

       Both now share K = X25519(a, X25519(b, 9)) = X25519(b, X25519(a, 9))
       as a shared secret.  Both MAY check, without leaking extra
       information about the value of K, whether K is the all-zero value and
       abort if so (see below).  Alice and Bob can then use a key-derivation
       function that includes K, K_A, and K_B to derive a symmetric key.

       The check for the all-zero value results from the fact that the
       X25519 function produces that value if it operates on an input
       corresponding to a point with small order, where the order divides
       the cofactor of the curve (see Section 7).  The check may be
       performed by ORing all the bytes together and checking whether the
       result is zero, as this eliminates standard side-channels in software
       implementations.

       Test vector:

       Alice's private key, a:
         77076d0a7318a57d3c16c17251b26645df4c2f87ebc0992ab177fba51db92c2a
       Alice's public key, X25519(a, 9):
         8520f0098930a754748b7ddcb43ef75a0dbf3a0d26381af4eba4a98eaa9b4e6a
       Bob's private key, b:
         5dab087e624a8a4b79e17f8b83800ee66f3bb1292618b6fd1c2f8b27ff88e0eb
       Bob's public key, X25519(b, 9):
         de9edb7d7b7dc1b4d35b61c2ece435373f8343c85b78674dadfc7e146f882b4f
       Their shared secret, K:
         4a5d9d5ba4ce2de1728e3bf480350f25e07e21c947d19e3376f09b3c1e161742

    */
}
