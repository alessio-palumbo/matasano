use common::io::read_input;
use crypto::aes::{Aes128Cipher, EncryptionMode, BLOCK_SIZE};
use guess::aes::is_aes_in_ecb_mode;
use rand::Rng;
use std::fs;

/// Challenge 11 is the eleventh Matasano challenge of Set 2.
pub fn challenge_eleven() {
    println!("\n# Challenge 11 #");
    loop {
        let filename =
            read_input("Please input filename of an AES-128-ECB or AES-128-CBC encrypted file:");
        let src = fs::read_to_string(filename).unwrap();
        match is_aes_ecb_or_cbc(src.as_bytes()) {
            EncryptionMode::ECB => println!("File encrypted in ECB mode"),
            _ => println!("File encrypted in CBC mode"),
        };
    }
}

fn is_aes_ecb_or_cbc(src: &[u8]) -> EncryptionMode {
    match is_aes_in_ecb_mode(src) {
        true => EncryptionMode::ECB,
        false => EncryptionMode::CBC,
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct RandomEncryptor {
    key: Vec<u8>,
    mode: EncryptionMode,
}

#[allow(dead_code)]
impl RandomEncryptor {
    /// Initialises a RandomEncryptor with a random key.
    pub fn new() -> RandomEncryptor {
        RandomEncryptor {
            key: Self::random_key(BLOCK_SIZE),
            mode: EncryptionMode::UNKNOWN,
        }
    }

    /// Appends a salt to a plaintext and encrypts it with the random key.
    pub fn random_append_ecb_encrypt(&self, src: &[u8], salt: &[u8]) -> Vec<u8> {
        let src = [src.to_vec(), salt.to_vec()].concat();
        self.ecb_encrypt(&src)
    }

    /// Encrypts with a random key with Aes128Cipher in ECB mode.
    pub fn ecb_encrypt(&self, src: &[u8]) -> Vec<u8> {
        self.encrypt(src, EncryptionMode::ECB)
    }

    /// Encrypts with a random key and IV with Aes128Cipher in CBC mode.
    pub fn cbc_encrypt(&self, src: &[u8]) -> Vec<u8> {
        self.encrypt(src, EncryptionMode::CBC)
    }

    /// Decrypts with the previously generated random key with Aes128Cipher in ECB mode.
    pub fn ecb_decrypt(&self, src: &[u8]) -> Vec<u8> {
        let cipher = Aes128Cipher::new(&self.key);
        let mut blocks = cipher.split_to_blocks(&src);
        cipher.ecb_decrypt(&mut blocks);
        blocks.into_iter().flatten().collect()
    }

    /// Encrypts with a random key with Aes128Cipher according to the given mode.
    fn encrypt(&self, src: &[u8], mode: EncryptionMode) -> Vec<u8> {
        let cipher = Aes128Cipher::new(&self.key);
        let mut blocks = cipher.split_to_blocks(src);
        match mode {
            EncryptionMode::ECB => cipher.ecb_encrypt(&mut blocks),
            EncryptionMode::CBC => {
                blocks = cipher.cbc_encrypt(&Self::random_key(BLOCK_SIZE), &mut blocks)
            }
            _ => panic!("Invalid mode"),
        }
        blocks.into_iter().flatten().collect()
    }

    /// Generates an Aes128Cipher in either ECB or CBC mode, with randomly padded
    /// start and end and with random key/iv.
    fn random_encrypt(&mut self, src: &[u8]) -> Vec<u8> {
        let src = Self::add_random_start_end(src, 5, 10);
        match rand::thread_rng().gen() {
            true => {
                self.mode = EncryptionMode::ECB;
                self.ecb_encrypt(&src)
            }
            false => {
                self.mode = EncryptionMode::CBC;
                self.cbc_encrypt(&src)
            }
        }
    }

    /// Generates a random key of the given size.
    fn random_key(size: usize) -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let mut v: Vec<u8> = Vec::with_capacity(size);
        for _ in 0..size {
            v.push(rng.gen::<u8>());
        }
        v
    }

    /// Prepends and appends generated slices of bytes to the given slice and returns a vector.
    fn add_random_start_end(src: &[u8], min: usize, max: usize) -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let start = Self::random_key(rng.gen_range(min..=max));
        let end = Self::random_key(rng.gen_range(min..=max));
        [start, src.to_vec(), end].concat()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_aes_ecb_or_cbc() {
        let plaintext =
            "I'm back and I'm ringin' the bell \nA rockin' on the mike while the fly girls yell \nIn ecstasy in the back of me \n\
            Well that's my DJ Deshay cuttin' all them Z's \nHittin' hard and the girlies goin' crazy \nVanilla's on the mike, man \
            I'm not lazy. \n\nI'm lettin' my drug kick in \nIt controls my mouth and I begin \nTo just let it flow, let my concepts \
            go \nMy posse's to the side yellin', Go Vanilla Go! \n\nSmooth 'cause that's the way I will be \nAnd if you don't give \
            a damn, then \nWhy you starin' at me \nSo get off 'cause I control the stage \nThere's no dissin' allowed \nI'm in my \
            own phase \nThe girlies sa y they love me and that is ok \nAnd I can dance better than any kid n' play \n\nStage 2 -- \
            Yea the one ya' wanna listen to \nIt's off my head so let the beat play through \nSo I can funk it up and make it sound \
            good \n1-2-3 Yo -- Knock on some wood \nFor good luck, I like my rhymes atrocious \nSupercalafragilisticexpialidocious \n\
            I'm an effect and that you can bet \nI can take a fly girl and make her wet. \n\nI'm like Samson -- Samson to Delilah \n\
            There's no denyin', You can try to hang \nBut you'll keep tryin' to get my style \nOver and over, practice makes perfect \
            \nBut not if you're a loafer. \n\nYou'll get nowhere, no place, no time, no girls \nSoon -- Oh my God, homebody, you \
            probably eat \nSpaghetti with a spoon! Come on and say it! \n\nVIP. Vanilla Ice yep, yep, I'm comin' hard like a rhino \
            \nIntoxicating so you stagger like a wino \nSo punks stop trying and girl stop cryin' \nVanilla Ice is sellin' and you \
            people are buyin' \n'Cause why the freaks are jockin' like Crazy Glue \nMovin' and groovin' trying to sing along \nAll \
            through the ghetto groovin' this here song \nNow you're amazed by the VIP posse. \n\nSteppin' so hard like a German Nazi \
            \nStartled by the bases hittin' ground \nThere's no trippin' on mine, I'm just gettin' down \nSparkamatic, I'm hangin' \
            tight like a fanatic \nYou trapped me once and I thought that \nYou might have it \nSo step down and lend me your ear \n\
            '89 in my time! You, '90 is my year. \n\nYou're weakenin' fast, YO! and I can tell it \nYour body's gettin' hot, so, so \
            I can smell it \nSo don't be mad and don't be sad \n'Cause the lyrics belong to ICE, You can call me Dad \nYou're pitchin' \
            a fit, so step back and endure \nLet the witch doctor, Ice, do the dance to cure \nSo come up close and don't be square \n\
            You wanna battle me -- Anytime, anywhere \n\nYou thought that I was weak, Boy, you're dead wrong \nSo come on, everybody \
            and sing this song \n\nSay -- Play that funky music Say, go white boy, go white boy go \nplay that funky music Go white \
            boy, go white boy, go \nLay down and boogie and play that funky music till you die. \n\nPlay that funky music Come on, \
            Come on, let me hear \nPlay that funky music white boy you say it, say it \nPlay that funky music A little louder now \n\
            Play that funky music, white boy Come on, Come on, Come on \nPlay that funky music \n\u{4}\u{4}\u{4}\u{4}".as_bytes();

        for _ in 0..10 {
            let mut cipher = RandomEncryptor::new();
            let src = cipher.random_encrypt(plaintext);
            let got = is_aes_ecb_or_cbc(&src);
            let want = cipher.mode;
            assert_eq!(got, want);
        }
    }
}
