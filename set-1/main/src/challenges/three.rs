use crate::challenges::helpers::read_input;
use hex::decode;

const ALPHABET: &[u8] = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes();

/// Challenge 3 is the third Matasano challenge of Set 1.
pub fn challenge_three() {
    println!("\n# Challenge 3 #");
    loop {
        let hex = read_input("Please input single-character xor-encrypted hex string:/n");
        let hex = decode(hex.as_bytes()).unwrap();

        match force_decrypt(&hex, ALPHABET) {
            Ok((dec, c)) => println!("String was XORed with '{}':\n>>>> {}", c as char, dec),
            Err(e) => println!("{}", e),
        }
    }
}

// Attempts to decrypts a slice by XORing each byte against a single byte
// of the given set and then validating against a small set of valid English characters.
// If it finds a valid match it returns the valid string and the decrypting byte,
// otherwise it repeats the process with the next byte of the set.
fn force_decrypt(buf: &[u8], set: &[u8]) -> Result<(String, u8), String> {
    for b in set.iter() {
        let s = String::from_utf8(single_char_xor(buf, b)).unwrap();
        match is_valid_message(&s) {
            Ok(_) => return Ok((s, *b)),
            Err(c) => println!("XOR with char '{}' produces an invalid string", c as char),
        }
    }
    Err(String::from("Could not found decryption key"))
}

// Checks a string against invalid characters.
// TODO Improve validation to cater for the following:
// * multiple spaces, commas or quotes
// * word length and current average (missing spaces?)
// * mix case in single word (only allow capitalised or all caps)
fn is_valid_message(s: &str) -> Result<(), char> {
    let ls = s.to_lowercase();
    for c in ls.chars() {
        if !c.is_ascii_alphabetic() {
            match c {
                ' ' | ',' | '\'' | '!' | '?' | ':' | '.' => continue,
                _ => return Err(c),
            }
        }
    }
    Ok(())
}

// Applies a single character XOR operation to a buffer.
pub fn single_char_xor(buf: &[u8], c: &u8) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::new();
    for b in buf.iter() {
        v.push(b ^ c);
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_char_xor() {
        let buf: &[u8] = "hello".as_bytes();
        let c: u8 = 64;
        let got = String::from_utf8(single_char_xor(buf, &c)).unwrap();
        let want = String::from("(%,,/");
        assert_eq!(got, want);
    }

    #[test]
    fn test_is_valid_message() {
        let m = "Hello World!";
        let got = is_valid_message(m);
        let want = Ok(());
        assert_eq!(got, want);

        let m = "How's your day? Great, thank you.";
        let got = is_valid_message(m);
        let want = Ok(());
        assert_eq!(got, want);

        let m = "GUidi%bi_|ilf";
        let got = is_valid_message(m);
        let want = Err('%');
        assert_eq!(got, want);
    }

    // TODO Add test for force_decrypt
}
