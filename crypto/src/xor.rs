/// Performs a XOR operation between two buffers of the same size.
pub fn fixed_xor(buf_a: &[u8], buf_b: &[u8]) -> Result<Vec<u8>, String> {
    if buf_a.len() != buf_b.len() {
        return Err("Buffers must have equal length".to_string());
    } else if buf_a.is_empty() {
        return Ok(Vec::new());
    }

    let mut v: Vec<u8> = Vec::new();
    for (i, &a) in buf_a.iter().enumerate() {
        v.push(a ^ buf_b[i]);
    }
    Ok(v)
}

/// Applies a single character XOR operation to a buffer.
pub fn single_char_xor(buf: &[u8], c: &u8) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::new();
    for b in buf.iter() {
        v.push(b ^ c);
    }
    v
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
    use hex::{decode, encode};

    #[test]
    fn test_fixed_xor() {
        let buf_a = decode(b"1c0111001f010100061a024b53535009181c").unwrap();
        let buf_b = decode(b"686974207468652062756c6c277320657965").unwrap();
        let got = fixed_xor(&buf_a, &buf_b);
        let want = Ok(decode(b"746865206b696420646f6e277420706c6179").unwrap());
        assert_eq!(got, want);

        let buf_a = decode(b"abc123").unwrap();
        let buf_b = decode(b"c421").unwrap();
        let got = fixed_xor(&buf_a, &buf_b);
        let want = Err(String::from("Buffers must have equal length"));
        assert_eq!(got, want);

        let buf_a: &[u8] = &[];
        let buf_b: &[u8] = &[];
        let got = fixed_xor(buf_a, buf_b);
        let want = Ok(Vec::new());
        assert_eq!(got, want);
    }

    #[test]
    fn test_single_char_xor() {
        let buf = b"hello";
        let c: u8 = 64;
        let got = String::from_utf8(single_char_xor(buf, &c)).unwrap();
        let want = String::from("(%,,/");
        assert_eq!(got, want);
    }

    #[test]
    fn test_repeating_key_xor() {
        let src = b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let key = b"ICE";
        let got = &encode(&repeating_key_xor(src, key));
        let want = b"0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272\
            a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
        assert_eq!(got, want);
    }
}
