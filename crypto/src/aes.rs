use aes::cipher::{generic_array::GenericArray, BlockDecrypt, KeyInit};
use aes::Aes128;

pub const BLOCK_SIZE: usize = 16;

pub fn decrypt_ecb(src: &[u8], key: &[u8]) -> Vec<u8> {
    if key.len() != BLOCK_SIZE {
        panic!("invalid key length");
    }

    let key: [u8; BLOCK_SIZE] = key.try_into().unwrap();
    let key = GenericArray::from(key);
    let cipher = Aes128::new(&key);

    let n_blocks = src.len() / 16;
    let more = src.len() % 16 != 0;

    let mut blocks =
        vec![GenericArray::from([0u8; 16]); if more { n_blocks + 1 } else { n_blocks }];

    let n = n_blocks * 16;
    for (i, b) in (0..n).step_by(16).enumerate() {
        let block: [u8; 16] = src[b..b + BLOCK_SIZE].try_into().unwrap();
        blocks[i] = GenericArray::from(block);
    }
    if more {
        let block: [u8; 16] = src[n..].try_into().unwrap();
        blocks[n / 16] = GenericArray::from(block);
    }
    cipher.decrypt_blocks(&mut blocks);
    blocks.into_iter().flatten().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex::decode;

    #[test]
    fn test_decrypt_ecb() {
        let src =
            decode(b"646C424369B514BF5ECADB962FE8BD3F7ABCEC648CFA7034AA68CF7034AF1CF1").unwrap();
        let key = b"YELLOW SUBMARINE";
        let got = decrypt_ecb(&src, key);
        let want = String::from("Hello World, this is a test!\u{4}\u{4}\u{4}\u{4}");
        assert_eq!(String::from_utf8(got).unwrap(), want);
    }
}
