use common::io::read_input;
use common::ok_or_continue;
use crypto::xor::single_char_xor;
use hex::decode;

/// Challenge 3 is the third Matasano challenge of Set 1.
pub fn challenge_three() {
    println!("\n# Challenge 3 #");
    loop {
        let hex = read_input("Please input single-character xor-encrypted hex string:");
        let hex = decode(hex.as_bytes()).unwrap();

        match single_char_force_decrypt(&hex) {
            Ok((dec, c)) => println!("String was XORed with '{}':\n>>>> {}", c as char, dec),
            Err(e) => println!("{}", e),
        }
    }
}

/// Attempts to decrypts a slice by XORing each byte against a single byte
/// value, then validates against a small set of valid English characters.
/// If it finds a valid match it returns the valid string and the decrypting byte,
/// otherwise it repeats the process with the next byte of the set.
pub fn single_char_force_decrypt(buf: &[u8]) -> Result<(String, u8), String> {
    for b in 0..=255 {
        let s = ok_or_continue!(String::from_utf8(single_char_xor(buf, &b)));

        ok_or_continue!(is_valid_message(&s));
        return Ok((s, b));
    }
    Err(String::from("Could not found decryption key"))
}

/// Checks a string against invalid characters.
/// TODO Improve validation to cater for the following:
/// * multiple spaces, commas or quotes
/// * word length and current average (missing spaces?)
/// * mix case in single word (only allow capitalised or all caps)
fn is_valid_message(s: &str) -> Result<(), char> {
    for c in s.chars() {
        if !c.is_ascii_alphanumeric() {
            match c {
                ' ' | ',' | '\'' | '"' | '-' | '\n' | '!' | '?' | ':' | '.' => continue,
                _ => {
                    return Err(c);
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_force_decryt() {
        let buf: &[u8] = &decode(
            "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".as_bytes(),
        )
        .unwrap();
        let got = single_char_force_decrypt(buf);
        let want = Ok((String::from("Cooking MC's like a pound of bacon"), 88));
        assert_eq!(got, want);
    }
}
