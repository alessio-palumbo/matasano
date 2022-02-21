use crate::challenges::helpers::read_input;
use base64::encode_to_string;
use hex::decode;

/// Challenge 1 is the first Matasano challenge of Set 1.
pub fn challenge_one() {
    println!("\n# Challenge 1 #");
    loop {
        let hex =
            read_input("Please input a hexadecimal string which will be converted to base64:");

        let enc = match convert_hex_to_base64(&hex) {
            Ok(s) => s,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };
        println!("Hex string:\n {}\nconverts to base64:\n {}", hex, enc);
    }
}

/// Decodes a hexadecimal slice and converts it into a padded standard-encoding base64 String.
/// It returns an error if the input is not a valid hexadecimal string.
pub fn convert_hex_to_base64(s: &str) -> Result<String, String> {
    let dec = decode(s.as_bytes())?;
    Ok(encode_to_string(&dec))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_converts_hex_to_base64() {
        let src: &str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let got = convert_hex_to_base64(src);
        let want =
            Ok("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string());
        assert_eq!(got, want);

        let src: &str = "abcdef012";
        let got = convert_hex_to_base64(src);
        let want = Err("Invalid slice length: 9".to_string());
        assert_eq!(got, want);

        let src: &str = "abcfgh@#";
        let got = convert_hex_to_base64(src);
        let want = Err("Invalid hex value: 103".to_string());
        assert_eq!(got, want);
    }
}
