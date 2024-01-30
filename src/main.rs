use ring::rand::SystemRandom;
use ring::signature::{KeyPair, Ed25519KeyPair};
use sha2::{Sha256, Digest};
use bitcoin_hashes::{ripemd160, Hash as BitcoinHash};
use bs58;

fn main() {
    let rng = SystemRandom::new();

    for _ in 0..100 {
        let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
        let key_pair = Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).unwrap();
        let public_key = key_pair.public_key().as_ref();

        // Step 1: SHA-256 hashing
        let sha256_hash = Sha256::digest(public_key);

        // Step 2: RIPEMD-160 hashing using bitcoin_hashes
        let ripemd160_hash = ripemd160::Hash::hash( &sha256_hash, );
        println!("Ripemd160 hash: {:?}", ripemd160_hash);

        // Step 3: Add version byte (0x00 for Bitcoin mainnet)
        let mut address = vec![0x00];
        address.extend_from_slice(&ripemd160_hash[..]);

        // Step 4 & 5: Double SHA-256 hashing for checksum
        let checksum = Sha256::digest(&Sha256::digest(&address));
        address.extend_from_slice(&checksum[..4]);

        // Step 6: Base58Check encoding
        let btc_address = bs58::encode(address).into_string();

        println!("Bitcoin address: {}", btc_address);
    }
}
