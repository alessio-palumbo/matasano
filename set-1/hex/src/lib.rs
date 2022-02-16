pub fn from_hex(hex_bytes: &[u8]) -> Result<Vec<u8>, String> {
    if hex_bytes.len() % 2 != 0 {
        return Err(format!("Invalid slice length: {}", hex_bytes.len()));
    }

    let mut bytes: Vec<u8> = Vec::new();
    let mut idx = 0;

    for _ in hex_bytes.iter().step_by(2) {
        let high_bits = bits_from_hex(hex_bytes[idx])?;
        let low_bits = bits_from_hex(hex_bytes[idx + 1])?;

        bytes.push(high_bits << 4 | low_bits);
        idx += 2;
    }

    Ok(bytes)
}

fn bits_from_hex(c: u8) -> Result<u8, String> {
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
        let hex2: &[u8] = "0123".as_bytes();
        let hex3: &[u8] = "0Aab".as_bytes();

        assert_eq!(from_hex(hex1), Err(format!("Invalid slice length: 3")));
        assert_eq!(from_hex(hex2), Ok(vec![1, 35]));
        assert_eq!(from_hex(hex3), Ok(vec![10, 171]));
    }
}
