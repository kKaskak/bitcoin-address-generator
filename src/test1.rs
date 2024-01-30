use ring::rand::SystemRandom;
use ring::signature::{KeyPair, Ed25519KeyPair};
use hex;

// stable working version

fn main() {

    let rng = SystemRandom::new();

    for _ in 0..100 {
        let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
        let key_pair = Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).unwrap();
        let public_key = key_pair.public_key();

        // Convert PKCS#8 bytes to hexadecimal format for printing
        let pkcs8_hex = hex::encode(pkcs8_bytes);

        // Output the PKCS#8 bytes in hexadecimal format and the public key
        println!("Private key (PKCS#8 bytes in hex): {}", pkcs8_hex);
        println!("Public key: {:?}", public_key);
    }
}