/// Decodes a hex-encoded slice of bytes and either returns
/// a vector of bytes or an error message.
pub fn decode(b: &[u8]) -> Result<Vec<u8>, String> {
    if b.len() % 2 != 0 {
        return Err(format!("Invalid slice length: {}", b.len()));
    }

    let mut v: Vec<u8> = Vec::new();

    for i in (0..b.len()).step_by(2) {
        let hb = from_hex(b[i])?;
        let lb = from_hex(b[i + 1])?;
        v.push(hb << 4 | lb);
    }

    Ok(v)
}

fn from_hex(c: u8) -> Result<u8, String> {
    if (b'0'..=b'9').contains(&c) {
        return Ok(c - b'0');
    } else if (b'a'..=b'f').contains(&c) {
        return Ok(c - b'a' + 10);
    } else if (b'A'..=b'F').contains(&c) {
        return Ok(c - b'A' + 10);
    }
    return Err(format!("Invalid hex value: {}", c));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_hex_to_bytes() {
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
