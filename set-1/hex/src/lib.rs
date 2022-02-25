const HEX_TABLE: &[u8] = "0123456789abcdef".as_bytes();

/// Encodes a slice of bytes into an hex vector.
pub fn encode(s: &[u8]) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::new();
    for b in s.iter() {
        v.push(HEX_TABLE[(b >> 4) as usize]);
        v.push(HEX_TABLE[(b & 0x0f) as usize]);
    }
    v
}

/// Decodes an even size hex-encoded slice of bytes and either returns
/// a vector of bytes or an error message.
pub fn decode(b: &[u8]) -> Result<Vec<u8>, String> {
    if b.len() % 2 != 0 {
        return Err(format!("Invalid slice length: {}", b.len()));
    }

    let mut v: Vec<u8> = Vec::new();
    for i in (0..b.len()).step_by(2) {
        v.push(from_hex(b[i])? << 4 | from_hex(b[i + 1])?);
    }
    Ok(v)
}

/// Decodes a hex char into the represented 4 bits value.
pub fn from_hex(c: u8) -> Result<u8, String> {
    match c {
        b'0'..=b'9' => Ok(c - b'0'),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        b'A'..=b'F' => Ok(c - b'A' + 10),
        _ => Err(format!("Invalid hex value: {}", c)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        let hex1: &[u8] = "012".as_bytes();
        let hex2: &[u8] = "Abg0".as_bytes();
        let hex3: &[u8] = "0123".as_bytes();
        let hex4: &[u8] = "0Aab".as_bytes();

        assert_eq!(decode(hex1), Err("Invalid slice length: 3".to_string()));
        assert_eq!(decode(hex2), Err("Invalid hex value: 103".to_string()));
        assert_eq!(decode(hex3), Ok(vec![1, 35]));
        assert_eq!(decode(hex4), Ok(vec![10, 171]));
    }
}
