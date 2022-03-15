use aes::cipher::{generic_array::GenericArray, typenum::U16, BlockDecrypt, BlockEncrypt, KeyInit};
use aes::Aes128;

pub const BLOCK_SIZE: usize = 16;

#[derive(Debug)]
pub struct Aes128Cipher {
    cipher: Aes128,
}

impl Aes128Cipher {
    pub fn new(key: &[u8]) -> Aes128Cipher {
        let key: [u8; BLOCK_SIZE] = key.try_into().unwrap();
        let key = GenericArray::from(key);
        Aes128Cipher {
            cipher: Aes128::new(&key),
        }
    }

    pub fn split_to_blocks(&self, src: &[u8]) -> Vec<GenericArray<u8, U16>> {
        let mut blocks: Vec<GenericArray<u8, U16>> = Vec::with_capacity(
            src.len() / BLOCK_SIZE + (if src.len() % BLOCK_SIZE > 0 { 1 } else { 0 }),
        );

        for (i, _) in src.iter().enumerate().step_by(BLOCK_SIZE) {
            let end = if (i + BLOCK_SIZE) < src.len() {
                i + BLOCK_SIZE
            } else {
                src.len()
            };
            let block: [u8; BLOCK_SIZE] = src[i..end].try_into().unwrap();
            blocks.push(GenericArray::from(block));
        }
        blocks
    }

    pub fn encrypt(&self, blocks: &mut [GenericArray<u8, U16>]) {
        self.cipher.encrypt_blocks(blocks);
    }

    pub fn decrypt(&self, blocks: &mut [GenericArray<u8, U16>]) {
        self.cipher.decrypt_blocks(blocks);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex::decode;

    #[test]
    fn test_aes128_encrypt() {
        let key = b"YELLOW SUBMARINE";
        let cipher = Aes128Cipher::new(key);
        let src = "Hello World, this is a test!\u{4}\u{4}\u{4}\u{4}".as_bytes();
        let mut blocks = cipher.split_to_blocks(src);
        cipher.encrypt(&mut blocks);
        let got: Vec<u8> = blocks.into_iter().flatten().collect();
        let want =
            decode(b"646C424369B514BF5ECADB962FE8BD3F7ABCEC648CFA7034AA68CF7034AF1CF1").unwrap();
        assert_eq!(got, want);
    }

    #[test]
    fn test_aes128_decrypt() {
        let key = b"YELLOW SUBMARINE";
        let cipher = Aes128Cipher::new(key);
        let src =
            decode(b"646C424369B514BF5ECADB962FE8BD3F7ABCEC648CFA7034AA68CF7034AF1CF1").unwrap();
        let mut blocks = cipher.split_to_blocks(&src);
        cipher.decrypt(&mut blocks);
        let got: Vec<u8> = blocks.into_iter().flatten().collect();
        let want = "Hello World, this is a test!\u{4}\u{4}\u{4}\u{4}".as_bytes();
        assert_eq!(&got, want);
    }
}
