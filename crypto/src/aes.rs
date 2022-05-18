use crate::padding::pkcs_hash7_padding;
use crate::xor::fixed_xor;
use aes::cipher::{generic_array::GenericArray, typenum::U16, BlockDecrypt, BlockEncrypt, KeyInit};
use aes::Aes128;

#[derive(PartialEq, Debug)]
pub enum EncryptionMode {
    UNKNOWN,
    ECB,
    CBC,
}

pub const BLOCK_SIZE: usize = 16;

/// Block is a type alias for a cipher block.
pub type Block = GenericArray<u8, U16>;

#[derive(Debug)]
pub struct Aes128Cipher {
    cipher: Aes128,
}

impl Aes128Cipher {
    /// Initialises an Aes128Cipher with a key. It panics if the key is not 16 bytes.
    pub fn new(key: &[u8]) -> Aes128Cipher {
        let key: [u8; BLOCK_SIZE] = key.try_into().unwrap();
        Aes128Cipher {
            cipher: Aes128::new(&GenericArray::from(key)),
        }
    }

    /// Splits a buffer into blocks of BLOCK_SIZE adding padding if necessary
    pub fn split_to_blocks(&self, src: &[u8]) -> Vec<Block> {
        let mut blocks: Vec<Block> = Vec::with_capacity(
            src.len() / BLOCK_SIZE + (if src.len() % BLOCK_SIZE > 0 { 1 } else { 0 }),
        );

        for (i, _) in src.iter().enumerate().step_by(BLOCK_SIZE) {
            let block: [u8; BLOCK_SIZE] = if (i + BLOCK_SIZE) <= src.len() {
                src[i..i + BLOCK_SIZE].try_into().unwrap()
            } else {
                pkcs_hash7_padding(&src[i..src.len()], BLOCK_SIZE)
                    .try_into()
                    .unwrap()
            };

            blocks.push(GenericArray::from(block));
        }
        blocks
    }

    /// Encrypts the given blocks with the cipher's key and the given IV
    /// according to CBC algorithm.
    pub fn cbc_encrypt(&self, iv: &[u8], blocks: &mut [Block]) -> Vec<Block> {
        let mut cbc_blocks: Vec<Block> = Vec::with_capacity(blocks.len());
        let v: [u8; BLOCK_SIZE] = iv.try_into().unwrap();
        let mut v = GenericArray::from(v);

        for b in blocks.iter() {
            let x = fixed_xor(b.as_slice(), &v).unwrap();
            let x: [u8; BLOCK_SIZE] = x.try_into().unwrap();
            let mut x = GenericArray::from(x);
            self.cipher.encrypt_block(&mut x);
            cbc_blocks.push(x);
            v = x;
        }
        cbc_blocks
    }

    /// Decrypts the CBC-encrypted blocks with the cipher's key and given IV.
    pub fn cbc_decrypt(&self, iv: &[u8], blocks: &mut [Block]) -> Vec<Block> {
        let mut cbc_blocks: Vec<Block> = Vec::with_capacity(blocks.len());
        let v: [u8; BLOCK_SIZE] = iv.try_into().unwrap();
        let mut v = GenericArray::from(v);

        for b in blocks.iter_mut() {
            let temp = *b;
            self.cipher.decrypt_block(b);
            let x = fixed_xor(b.as_slice(), &v).unwrap();
            let x: [u8; BLOCK_SIZE] = x.try_into().unwrap();
            cbc_blocks.push(GenericArray::from(x));
            v = temp;
        }
        cbc_blocks
    }

    /// Encrypts the given blocks according to the ECB algorithm.
    pub fn ecb_encrypt(&self, blocks: &mut [Block]) {
        self.cipher.encrypt_blocks(blocks);
    }

    /// Decrypts the given ECB-encrypted blocks.
    pub fn ecb_decrypt(&self, blocks: &mut [Block]) {
        self.cipher.decrypt_blocks(blocks);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex::decode;

    #[test]
    fn test_ecb_encrypt() {
        let key = b"YELLOW SUBMARINE";
        let cipher = Aes128Cipher::new(key);
        let src = "Hello World, this is a test!\u{4}\u{4}\u{4}\u{4}".as_bytes();
        let mut blocks = cipher.split_to_blocks(src);
        cipher.ecb_encrypt(&mut blocks);
        let got: Vec<u8> = blocks.into_iter().flatten().collect();
        let want =
            decode(b"646C424369B514BF5ECADB962FE8BD3F7ABCEC648CFA7034AA68CF7034AF1CF1").unwrap();
        assert_eq!(got, want);
    }

    #[test]
    fn test_ecb_decrypt() {
        let key = b"YELLOW SUBMARINE";
        let cipher = Aes128Cipher::new(key);
        let src =
            decode(b"646C424369B514BF5ECADB962FE8BD3F7ABCEC648CFA7034AA68CF7034AF1CF1").unwrap();
        let mut blocks = cipher.split_to_blocks(&src);
        cipher.ecb_decrypt(&mut blocks);
        let got: Vec<u8> = blocks.into_iter().flatten().collect();
        let want = "Hello World, this is a test!\u{4}\u{4}\u{4}\u{4}".as_bytes();
        assert_eq!(&got, want);
    }

    #[test]
    fn test_cbc_encrypt() {
        let key = b"YELLOW SUBMARINE";
        let cipher = Aes128Cipher::new(key);
        let src = "Hello World, this is a test!".as_bytes();
        let mut blocks = cipher.split_to_blocks(src);
        let iv = vec![b'0'; 16];
        let blocks = cipher.cbc_encrypt(&iv, &mut blocks);
        let got: Vec<u8> = blocks.into_iter().flatten().collect();
        let want =
            decode(b"49D1FBF0C7E8CD95D533A129F91EF8F8AA94F5BA2088D7A75BA446DCEB8B7358").unwrap();
        assert_eq!(got, want);
    }

    #[test]
    fn test_cbc_decrypt() {
        let key = b"YELLOW SUBMARINE";
        let cipher = Aes128Cipher::new(key);
        let src =
            decode(b"49D1FBF0C7E8CD95D533A129F91EF8F8AA94F5BA2088D7A75BA446DCEB8B7358").unwrap();
        let mut blocks = cipher.split_to_blocks(&src);
        let iv = vec![b'0'; 16];
        let blocks = cipher.cbc_decrypt(&iv, &mut blocks);
        let got: Vec<u8> = blocks.into_iter().flatten().collect();
        let want = String::from("Hello World, this is a test!\u{4}\u{4}\u{4}\u{4}");
        assert_eq!(String::from_utf8(got).unwrap(), want);
    }
}
