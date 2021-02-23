use hex_literal::hex;
use sha2::{Digest, Sha256};

fn main() {
    let mut hasher = Sha256::new();
    hasher.update(b"pee pee");

    let result = hasher.finalize();
    println!("{:X}", result)
}
