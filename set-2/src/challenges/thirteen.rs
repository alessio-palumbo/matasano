use crate::challenges::eleven::RandomEncryptor;
use crypto::{aes::BLOCK_SIZE, padding::pkcs_hash7_padding};
use std::collections::HashMap;

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct Profile {
    email: String,
    uid: i32,
    role: String,
}

#[allow(dead_code)]
impl Profile {
    /// Initialises a profile with sanitised email and default values.
    pub fn profile_for(email: &str) -> Profile {
        Profile {
            email: sanitise_email(email),
            uid: 10,
            role: "user".to_owned(),
        }
    }

    /// Encodes the profile in k=v& format.
    pub fn encode(&self) -> String {
        format!("email={}&uid={}&role={}", self.email, self.uid, self.role)
    }
}

/// Creates and encrypts an encoded Profile for the given input.
#[allow(dead_code)]
fn encrypt_profile_for(cipher: &RandomEncryptor, input: &str) -> Vec<u8> {
    let profile = Profile::profile_for(input);
    cipher.ecb_encrypt(profile.encode().as_bytes())
}

/// Parses the given string of key-value pairs into a map.
#[allow(dead_code)]
fn parse_key_value(s: &str) -> HashMap<&str, &str> {
    s.split('&')
        .map(|p| {
            let kv: Vec<&str> = p.split('=').collect();
            if kv.len() != 2 {
                panic!("Not a valid key-value pair");
            }
            (kv[0], kv[1])
        })
        .collect()
}

/// Removes any reserved characters from an email.
#[allow(dead_code)]
fn sanitise_email(email: &str) -> String {
    email
        .chars()
        .filter_map(|c| match c {
            '&' | '=' => None,
            v => Some(v),
        })
        .collect()
}

#[allow(dead_code)]
fn cut_and_paste() -> Vec<u8> {
    let b1 = "email=";
    let b2 = "&uid=10&role=";

    let cipher = RandomEncryptor::new();

    // Craft a slice of bytes long enough to push "user" role
    // in the third block once encoded.
    // Encrypt encoded block and cut the first 2 blocks
    // we will append the "admin" role to.
    let input_size = 2 * BLOCK_SIZE - (b1.len() + b2.len());
    let input = string_repeat(input_size, 'A');
    let enc = encrypt_profile_for(&cipher, &input);

    let mut enc_admin: Vec<u8> = Vec::new();
    enc_admin.extend_from_slice(&enc[0..32]);

    // Craft a slice of bytes to pad the "email" block and append
    // the new "admin" role with pkcs7.
    // Encrypt and cut the second block containing the padded admin role.
    let input_size = BLOCK_SIZE - b1.len();
    let mut input = string_repeat(input_size, 'A');
    let admin_block = String::from_utf8(pkcs_hash7_padding(b"admin", BLOCK_SIZE)).unwrap();
    input.push_str(&admin_block);
    let enc = encrypt_profile_for(&cipher, &input);

    enc_admin.extend_from_slice(&enc[16..32]);

    cipher.ecb_decrypt(&enc_admin)
}

fn string_repeat(n: usize, c: char) -> String {
    (0..n).map(|_| c).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cut_and_paste() {
        let got = cut_and_paste();
        let want = "email=AAAAAAAAAAAAA&uid=10&role=admin\u{b}\u{b}\u{b}\u{b}\u{b}\u{b}\u{b}\u{b}\u{b}\u{b}\u{b}";
        assert_eq!(String::from_utf8(got).unwrap(), want);
    }

    #[test]
    fn test_encrypt_profile_for() {
        let cipher = RandomEncryptor::new();
        let enc = encrypt_profile_for(&cipher, "john@gmail.com");
        let got = cipher.ecb_decrypt(&enc);
        let want = "email=john@gmail.com&uid=10&role=user\u{b}\u{b}\u{b}\u{b}\u{b}\u{b}\u{b}\u{b}\u{b}\u{b}\u{b}";
        assert_eq!(String::from_utf8(got).unwrap(), want);
    }

    #[test]
    fn test_parse_key_value() {
        let s = "foo=bar&baz=qux&zap=zazzle";
        let want = HashMap::from([("foo", "bar"), ("baz", "qux"), ("zap", "zazzle")]);
        let got = parse_key_value(s);
        assert_eq!(got, want);
    }

    #[test]
    fn test_encode_profile() {
        let want = String::from("email=john@gmail.com&uid=10&role=user");
        let got = Profile::profile_for("john@gmail.com").encode();
        assert_eq!(got, want);
    }

    #[test]
    fn test_sanitise_email() {
        let got = sanitise_email("someuser&@email.=com");
        let want = String::from("someuser@email.com");
        assert_eq!(got, want);
    }
}
