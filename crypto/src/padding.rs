/// Pads a slice of bytes according to PKCS#7 scheme.
pub fn pkcs_hash7_padding(src: &[u8], block_size: usize) -> Vec<u8> {
    let mut v = src.to_vec();
    if let Some(mut p) = add_padding(src.len(), block_size) {
        v.append(&mut p)
    }
    v
}

fn add_padding(len: usize, cap: usize) -> Option<Vec<u8>> {
    let padding = cap - (len % cap);
    if padding > 0 {
        let byte = padding as u8;
        let mut v: Vec<u8> = Vec::new();
        for _ in 0..padding {
            v.push(byte);
        }
        return Some(v);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pkcs_hash7_padding() {
        let buf = b"YELLOW SUBMARINE";
        let got = pkcs_hash7_padding(buf, 20);
        let want = b"YELLOW SUBMARINE\x04\x04\x04\x04";
        assert_eq!(got, want);
    }
}
