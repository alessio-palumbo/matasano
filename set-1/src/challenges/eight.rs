use common::io::read_input;
use guess::aes::is_aes_in_ecb_mode;
use hex::decode;
use std::fs;

/// Challenge 8 is the eight Matasano challenge of Set 1.
pub fn challenge_eight() {
    println!("\n# Challenge 8 #");
    loop {
        let filename = read_input("Please input filename of an AES-128-ECB encrypted file:");
        let src = fs::read_to_string(filename).unwrap();
        let got = detect_aes_in_ecb_mode(&src).unwrap();
        println!("{}", String::from_utf8(got).unwrap());
    }
}

/// Reads a file containing a list of hexadecimal ciphertext and tries to
/// find which one has been encrypted with AES-128 ECB mode.
pub fn detect_aes_in_ecb_mode(src: &str) -> Result<Vec<u8>, String> {
    for s in src.split_ascii_whitespace() {
        let hex = decode(s.as_bytes()).unwrap();
        if is_aes_in_ecb_mode(&hex) {
            return Ok(hex.to_vec());
        }
    }
    Err(String::from("Could not detect ECB encrypted ciphertext"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex::encode;

    #[test]
    fn test_detect_aes_in_ecb_mode() {
        let src = fs::read_to_string("src/challenges/testdata/detect_aes_in_ecb_mode.txt").unwrap();
        let got = detect_aes_in_ecb_mode(&src).unwrap();
        let want = "d880619740a8a19b7840a8a31c810a3d08649af70dc06f4fd5d2d69c744cd283e2dd052f6b641dbf9d11b0348542bb5708649af70dc06f4fd5d2d69c744cd2839475c9dfdbc1d46597949d9c7e82bf5a08649af70dc06f4fd5d2d69c744cd28397a93eab8d6aecd566489154789a6b0308649af70dc06f4fd5d2d69c744cd283d403180c98c8f6db1f2a3f9c4040deb0ab51b29933f2c123c58386b06fba186a";
        assert_eq!(String::from_utf8(encode(&got)).unwrap(), want);
    }
}
