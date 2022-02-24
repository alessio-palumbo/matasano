use crate::challenges::helpers::read_input;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_xor() {
        let buf_a: &[u8] = &decode("1c0111001f010100061a024b53535009181c".as_bytes()).unwrap();
        let buf_b: &[u8] = &decode("686974207468652062756c6c277320657965".as_bytes()).unwrap();
        let got = fixed_xor(buf_a, buf_b);
        let want = Ok(decode("746865206b696420646f6e277420706c6179".as_bytes()).unwrap());
        assert_eq!(got, want);

        let buf_a: &[u8] = &decode("abc123".as_bytes()).unwrap();
        let buf_b: &[u8] = &decode("c421".as_bytes()).unwrap();
        let got = fixed_xor(buf_a, buf_b);
        let want = Err("Buffers must have equal length".to_string());
        assert_eq!(got, want);

        let buf_a: &[u8] = &[];
        let buf_b: &[u8] = &[];
        let got = fixed_xor(buf_a, buf_b);
        let want = Ok(Vec::new());
        assert_eq!(got, want);
    }
}
