use crate::challenges::helpers::read_input;
use hex::encode;

/// Challenge 5 is the fifth Matasano challenge of Set 1.
pub fn challenge_five() {
    println!("\n# Challenge 5 #");
    loop {
        let hex = read_input("Please input message to encrypt:/n");
        let key = read_input("Please input multi-character xor encryption key:/n");

        let enc = repeating_key_xor(hex.as_bytes(), key.as_bytes());
        println!("{}", String::from_utf8(encode(&enc)).unwrap());
    }
}

/// Encrypts a buffer with a multi-byte encryption key by sequentially XORing
/// each byte of the key with the next byte of the buffer.
pub fn repeating_key_xor(buf: &[u8], key: &[u8]) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::new();
    for (i, b) in buf.iter().enumerate() {
        v.push(b ^ key[i % key.len()]);
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let src = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal"
            .as_bytes();
        let key = "ICE".as_bytes();
        let got = &encode(&repeating_key_xor(src, key));
        let want = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272\
            a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
            .as_bytes();
        assert_eq!(got, want);
    }
}
