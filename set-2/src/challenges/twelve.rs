use crate::challenges::eleven::RandomEncryptor;
use common::io::read_input;
use guess::aes::is_aes_in_ecb_mode;
use std::collections::HashMap;

/// Challenge 12 is the twelfth Matasano challenge of Set 2.
pub fn challenge_twelve() {
    println!("\n# Challenge 12 #");
    loop {
        let plaintext = read_input("Plese input plaintext");
        let salt = read_input("Please input base64 encrypted salt");
        println!(
            "Encrypted text is:\n{}",
            byte_at_a_time_ecb_decrypt(plaintext.as_bytes(), salt.as_bytes())
        );
    }
}

#[allow(dead_code)]
fn byte_at_a_time_ecb_decrypt(src: &[u8], salt: &[u8]) -> String {
    let cipher = RandomEncryptor::new();
    let block_size = find_block_size(&cipher, salt);
    if block_size != 16 {
        panic!("Invalid block size");
    }

    let ciphertext = cipher.random_append_ecb_encrypt(src, salt);
    if !is_aes_in_ecb_mode(&ciphertext) {
        panic!("Encryption should be ECB");
    }

    let short_block = vec![b'A'; block_size - 1];
    let mut values: HashMap<Vec<u8>, u8> = HashMap::new();

    for c in valid_english_chars().into_iter() {
        let salt = vec![c; 1];
        let ciphertext = cipher.random_append_ecb_encrypt(&short_block, &salt);
        values.insert(ciphertext, c);
    }

    let mut res: Vec<u8> = Vec::new();
    for i in 0..salt.len() {
        let ciphertext = cipher.random_append_ecb_encrypt(&short_block, &salt[i..i + 1]);
        if let Some((_, v)) = values.get_key_value(&ciphertext[..16]) {
            res.push(*v);
        } else {
            panic!("Could not match value for {:?}", &salt[i..i + 1]);
        }
    }

    String::from_utf8(res).unwrap()
}

/// Returns a vector with the accepted character for an English text,
/// including common punctuation.
fn valid_english_chars() -> Vec<u8> {
    let upper = b'A'..=b'Z';
    let lower = b'a'..=b'z';
    let numbers = b'0'..=b'9';
    let punctuation = [
        b',', b' ', b',', b'\'', b'"', b'-', b'\n', b'!', b'?', b':', b'.',
    ];
    upper
        .chain(lower)
        .chain(numbers)
        .chain(punctuation)
        .collect()
}

/// Detect the ciphers' block size by feeding an extra byte
/// to the cipher until the returned ciphertext's size changes.
/// It then returns the two sizes' diff, which is the block size.
fn find_block_size(cipher: &RandomEncryptor, salt: &[u8]) -> usize {
    let mut bb = vec![b'A'; 1];
    let initial_len = cipher.random_append_ecb_encrypt(&bb, salt).len();

    let mut block_size = 0;
    while block_size < 1 {
        bb.push(b'A');
        block_size = cipher.random_append_ecb_encrypt(&bb, salt).len() - initial_len;
    }
    block_size
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::decode;

    #[test]
    fn test_byte_at_a_time_ecb_decrypt() {
        let src = decode(
            "d880619740a8a19b7840a8a31c810a3d08649af70dc06f4fd5d2d69c744c\
        d283e2dd052f6b641dbf9d11b0348542bb5708649af70dc06f4fd5d2d69c7\
        44cd2839475c9dfdbc1d46597949d9c7e82bf5a08649af70dc06f4fd5d2d6\
        9c744cd28397a93eab8d6aecd566489154789a6b0308649af70dc06f4fd5d\
        2d69c744cd283d403180c98c8f6db1f2a3f9c4040deb0ab51b29933f2c123\
        c58386b06fba186a"
                .as_bytes(),
        )
        .unwrap();
        let salt = decode(
            "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkg\
            aGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBq\
            dXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK"
                .as_bytes(),
        )
        .unwrap();
        let got = byte_at_a_time_ecb_decrypt(&src, &salt);
        let want = String::from(
            "Rollin' in my 5.0\nWith my rag-top down so my hair can blow\n\
            The girlies on standby waving just to say hi\n\
            Did you stop? No, I just drove by\n",
        );
        assert_eq!(got, want);
    }
}
