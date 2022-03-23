use crypto::aes::BLOCK_SIZE;
use std::collections::HashMap;

/// Try to detect whether the ciphertext has been encrypted with AES-128 ECB
/// by checking for duplicated blocks and returns true if at least one is found.
pub fn is_aes_in_ecb_mode(b: &[u8]) -> bool {
    if b.len() % BLOCK_SIZE != 0 {
        return false;
    }

    let mut cs: HashMap<&[u8], usize> = HashMap::new();
    for (i, _) in b.iter().enumerate().step_by(BLOCK_SIZE) {
        let counter = cs.entry(&b[i..i + BLOCK_SIZE]).or_insert(0);
        if *counter > 0 {
            return true;
        }
        *counter += 1;
    }
    false
}

// TODO add local test
