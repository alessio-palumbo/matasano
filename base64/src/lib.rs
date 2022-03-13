/// Represents the base64 standard encoding.
const STD_ENCODING: &[u8] =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes();
/// Represents the standard padding byte.
const PADDING: u8 = b'=';

/// Encodes a slice of bytes into a padded base64 string according to the standard encoding.
pub fn encode_to_string(src: &[u8]) -> String {
    let mut dst: Vec<u8> = vec![0; encode_size(src.len())];
    encode(&mut dst, src);
    String::from_utf8(dst).unwrap()
}

/// Returns the size required to store the encoded bytes for a given length.
pub fn encode_size(s: usize) -> usize {
    (s + 2) / 3 * 4
}

/// Encodes a slice of bytes (src) according to standard base64 encoding and writes it to dst.
/// If src is not a multiple of 3, it applies padding to remaining bytes.
/// It panics if dst overflows. Use `encode_size` to determine the correct size.
pub fn encode(dst: &mut [u8], src: &[u8]) {
    if src.is_empty() {
        return;
    }

    // Limit loop to maximum number of bytes that are multiple of 3.
    // The remainder, if any, will be fill out later on.
    let n = (src.len() / 3) * 3;

    for i in (0..n).step_by(3) {
        let v = (src[i] as u32) << 16 | (src[i + 1] as u32) << 8 | src[i + 2] as u32;

        let di = src_to_dst_index(i);
        dst[di] = STD_ENCODING[(v >> 18 & 0x3F) as usize];
        dst[di + 1] = STD_ENCODING[(v >> 12 & 0x3F) as usize];
        dst[di + 2] = STD_ENCODING[(v >> 6 & 0x3F) as usize];
        dst[di + 3] = STD_ENCODING[(v & 0x3F) as usize];
    }

    let remainder = src.len() % 3;
    if remainder == 0 {
        return;
    }

    let mut v = (src[n] as u32) << 16;
    if remainder == 2 {
        v |= (src[n + 1] as u32) << 8;
    }

    let di = src_to_dst_index(n);
    dst[di] = STD_ENCODING[(v >> 18 & 0x3F) as usize];
    dst[di + 1] = STD_ENCODING[(v >> 12 & 0x3F) as usize];

    dst[di + 2] = if remainder == 2 {
        STD_ENCODING[(v >> 6 & 0x3F) as usize]
    } else {
        PADDING
    };
    dst[di + 3] = PADDING;
}

fn src_to_dst_index(i: usize) -> usize {
    (i / 3) * 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_encodes() {
        let src: &[u8] = "abc123!?$*&()'-=@~".as_bytes();
        let mut dst: Vec<u8> = vec![0; encode_size(src.len())];
        encode(&mut dst, src);
        let want: &[u8] = "YWJjMTIzIT8kKiYoKSctPUB+".as_bytes();
        assert_eq!(dst, want);
    }

    #[test]
    fn it_encodes_to_string() {
        let src: &[u8] = "abc123!?$*&()'-=@~".as_bytes();
        let dst = encode_to_string(src);
        let want = String::from("YWJjMTIzIT8kKiYoKSctPUB+");
        assert_eq!(dst, want);

        let src: &[u8] = "Lorem ipsum dolor sit amet, consectetur adipiscing elit".as_bytes();
        let dst = encode_to_string(src);
        let want = String::from(
            "TG9yZW0gaXBzdW0gZG9sb3Igc2l0IGFtZXQsIGNvbnNlY3RldHVyIGFkaXBpc2NpbmcgZWxpdA==",
        );
        assert_eq!(dst, want);
    }
}
