use common::io::read_input;
use crypto::xor::fixed_xor;
use hex::{decode, encode};

/// Challenge 2 is the second Matasano challenge of Set 1.
pub fn challenge_two() {
    println!("\n# Challenge 2 #");
    loop {
        let a = read_input("Please input first hexadecimal string to be XORed:");
        let buf_a = decode(a.as_bytes()).unwrap();

        let b = read_input("Please input second hexadecimal string to be XORed:");
        let buf_b = decode(b.as_bytes()).unwrap();

        let xor = match fixed_xor(&buf_a, &buf_b) {
            Ok(v) => v,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };
        println!(
            "XORed input:\n>>>> {}",
            String::from_utf8(encode(&xor)).unwrap()
        );
    }
}
