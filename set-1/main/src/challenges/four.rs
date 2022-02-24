use crate::challenges::helpers::read_input;
use crate::challenges::three::single_char_force_decrypt;
use hex::decode;
use std::fs;

/// Challenge 4 is the fourth Matasano challenge of Set 1.
pub fn challenge_four() {
    println!("\n# Challenge 4 #");
    loop {
        let filename = read_input("Please input filename:/n");
        match read_and_decrypt_lines(&filename) {
            Ok((dec, c)) => println!("String was XORed with '{}':\n>>>> {}", c as char, dec),
            Err(e) => println!("{}", e),
        }
    }
}

/// Reads a file and try to decrypt each line until it finds a valid message.
fn read_and_decrypt_lines(filename: &str) -> Result<(String, u8), String> {
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    for s in content.split_ascii_whitespace() {
        let buf = &decode(s.as_bytes()).unwrap();
        if let Ok(v) = single_char_force_decrypt(buf) {
            return Ok(v);
        }
    }
    Err(String::from("Could not found decryption key"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let filename: &str = "src/challenges/testdata/encrypted_strings.txt";
        let got = read_and_decrypt_lines(filename);
        let want = Ok((String::from("Now that the party is jumping\n"), 53));
        assert_eq!(got, want);
    }
}
