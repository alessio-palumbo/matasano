use crate::challenges::helpers::read_input;
use hex::encode;

/// Challenge 5 is the fifth Matasano challenge of Set 1.
pub fn challenge_five() {
    println!("\n# Challenge 5 #");
    loop {
        let hex = read_input("Please input message to encrypt:/n");
        let key = read_input("Please input multi-character xor encryption key:/n");

        match multi_char_encrypt(hex.as_bytes(), key.as_bytes()) {
            Ok(v) => println!("{}", v),
            Err(e) => println!("{}", e),
        }
    }
}

/// Encrypts a buffer with a multi-byte encryption key by sequentially XORing
/// each byte of the key with the next byte of the buffer.
pub fn multi_char_encrypt(buf: &[u8], key: &[u8]) -> Result<String, String> {
    let mut v: Vec<u8> = Vec::new();
    for (i, b) in buf.iter().enumerate() {
        v.push(b ^ key[i % key.len()]);
    }

    match String::from_utf8(encode(&v)) {
        Ok(v) => Ok(v),
        Err(_) => Err(String::from("Failed to encrypt")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let buf = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal"
            .as_bytes();
        let key = "ICE".as_bytes();
        let got = multi_char_encrypt(buf, key);
        let want = Ok(String::from(
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272\
            a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f",
        ));
        assert_eq!(got, want);
    }
}
