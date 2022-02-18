use std::io;

fn main() {
    let mut input = String::new();
    println!("Please input a hexadecimal string which will be converted to base64:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let input = input.trim();
    let result = match convert_hex_to_base64(input) {
        Ok(s) => s,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    println!("Hex string:\n {}\nconverts to base64:\n {}", input, result);
}

fn convert_hex_to_base64(s: &str) -> Result<String, String> {
    let dec = hex::decode(s.as_bytes())?;
    Ok(base64::encode_to_string(&dec))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_converts() {
        let src: &str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let got = convert_hex_to_base64(src);
        let want =
            Ok("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string());
        assert_eq!(got, want);
    }

    #[test]
    fn it_fails_on_invalid_input_length() {
        let src: &str = "abcdef012";
        let got = convert_hex_to_base64(src);
        let want = Err("Invalid slice length: 9".to_string());
        assert_eq!(got, want);
    }

    #[test]
    fn it_fails_on_invalid_hex() {
        let src: &str = "abcfgh@#";
        let got = convert_hex_to_base64(src);
        let want = Err("Invalid hex value: 103".to_string());
        assert_eq!(got, want);
    }
}
