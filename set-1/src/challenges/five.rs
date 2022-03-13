use crate::challenges::helpers::read_input;
use crypto::xor::repeating_key_xor;
use hex::encode;

/// Challenge 5 is the fifth Matasano challenge of Set 1.
pub fn challenge_five() {
    println!("\n# Challenge 5 #");
    loop {
        let hex = read_input("Please input message to encrypt:");
        let key = read_input("Please input multi-character xor encryption key:");

        let enc = repeating_key_xor(hex.as_bytes(), key.as_bytes());
        println!("{}", String::from_utf8(encode(&enc)).unwrap());
    }
}
