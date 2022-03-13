use crate::challenges::helpers::{ok_or_continue, read_input};
use crate::challenges::three::single_char_force_decrypt;
use std::fs;

/// Challenge 6 is the sixth Matasano challenge of Set 1.
pub fn challenge_six() {
    println!("\n# Challenge 6 #");
    loop {
        let filename = read_input("Please input filename of encrypted file:");
        let src = fs::read_to_string(filename).unwrap();
        let guessed = break_repeating_key_xor(src.as_bytes());
        println!("{}", String::from_utf8(guessed).unwrap());
    }
}

/// Define the maximum length for a key.
const MAX_SIZE: usize = 40;

/// Try to find out the repeating_xor key from an encrypted plaintext.
pub fn break_repeating_key_xor(src: &[u8]) -> Vec<u8> {
    let block_size = guess_key_size(src, MAX_SIZE);
    let blocks = if src.len() % block_size == 0 {
        src.len() / block_size
    } else {
        (src.len() / block_size) + 1
    };

    let mut split: Vec<Vec<u8>> = vec![vec![0; block_size]; blocks];
    break_to_blocks(src, &mut split, block_size);

    let mut transposed: Vec<Vec<u8>> = vec![vec![0; blocks]; block_size];
    transpose(&split, &mut transposed, block_size);

    find_key(&transposed)
}

/// Try to guess the size of the key by repeatedly calculating the hamming
/// distance between adjecent blocks for a given key size and normalising it.
/// It then returns the key with the smallest hamming distance.
fn guess_key_size(buf: &[u8], max_size: usize) -> usize {
    let mut shortest: (usize, f64) = (max_size, 10000.0);
    for ks in 2..=max_size {
        let mut bs = 0;
        let mut distances: Vec<u32> = Vec::new();

        while bs + (ks * 2) <= buf.len() {
            let be = bs + ks;
            distances.push(compute_hamming_distance(&buf[bs..be], &buf[be..(be + ks)]));
            bs += ks;
        }

        if !distances.is_empty() {
            let normalised: f64 =
                (f64::from(distances.iter().sum::<u32>()) / distances.len() as f64) / ks as f64;
            if normalised < shortest.1 {
                shortest = (ks, normalised);
            }
        }
    }
    shortest.0
}

/// Computes Hamming distance between two slices of bytes of equal length
/// by XORing them and counting the total number of 1s.
fn compute_hamming_distance(a: &[u8], b: &[u8]) -> u32 {
    let mut hd = 0;
    for i in 0..a.len() {
        hd += count_ones(a[i] ^ b[i]) as u32;
    }
    hd
}

/// Returns the sum of 1s bit int the byte.
fn count_ones(b: u8) -> u8 {
    let mut c = 0;
    for i in 0..8 {
        c += b >> i & 1;
    }
    c
}

/// Break up a slice into blocks of the given block size.
fn break_to_blocks(src: &[u8], dst: &mut [Vec<u8>], block_size: usize) {
    let mut outer_index: usize = 0;
    for (i, _) in src.iter().enumerate() {
        let inner_index = i % block_size;
        // Push previous block to outer Vec and re-initialise inner Vec.
        if i != 0 && inner_index == 0 {
            outer_index += 1;
        }
        dst[outer_index][inner_index] = src[i];
    }
}

/// Builds an array of Vectors with transposes values.
fn transpose(src: &[Vec<u8>], dst: &mut [Vec<u8>], block_size: usize) {
    let mut inner_index: usize = 0;
    for (bi, block) in src.iter().enumerate() {
        for (i, _) in block.iter().enumerate() {
            let outer_index = i % block_size;
            if bi != 0 && outer_index == 0 {
                inner_index += 1;
            }
            dst[outer_index][inner_index] = src[bi][i];
        }
    }
}

/// Try to find the key by force xor decrypting the given slice.
fn find_key(src: &[Vec<u8>]) -> Vec<u8> {
    let mut key: Vec<u8> = Vec::new();
    for block in src.iter() {
        let (_, b) = ok_or_continue!(single_char_force_decrypt(block));
        key.push(b);
    }
    key
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_break_repeating_key_xor() {
        let src = fs::read_to_string("src/challenges/testdata/repeating_key_xor_raw.txt").unwrap();
        let got = break_repeating_key_xor(src.as_bytes());
        let want = String::from("Terminator X: Bring the noise");
        assert_eq!(String::from_utf8(got).unwrap(), want);
    }

    #[test]
    fn test_guess_key_size() {
        let src = fs::read_to_string("src/challenges/testdata/repeating_key_xor_raw.txt").unwrap();
        let got = guess_key_size(src.as_bytes(), MAX_SIZE);
        assert_eq!(got, 29);
    }

    #[test]
    fn test_compute_hamming_distance() {
        let a = "this is a test".as_bytes();
        let b = "wokka wokka!!!".as_bytes();
        assert_eq!(compute_hamming_distance(a, b), 37);
    }

    #[test]
    fn test_count_ones() {
        assert_eq!(count_ones(10), 2);
        assert_eq!(count_ones(245), 6);
    }

    #[test]
    fn test_break_to_blocks() {
        let src: &[u8] = "012345".as_bytes();
        let mut dst: Vec<Vec<u8>> = vec![vec![0; 3]; 2];
        break_to_blocks(src, &mut dst, 3);
        let want: Vec<Vec<u8>> = vec![vec![48, 49, 50], vec![51, 52, 53]];
        assert_eq!(dst, want);
    }

    #[test]
    fn test_transpose() {
        let src: Vec<Vec<u8>> = vec![vec![48, 49, 50], vec![51, 52, 53]];
        let mut dst: Vec<Vec<u8>> = vec![vec![0; 2]; 3];
        transpose(&src, &mut dst, 3);
        let want: Vec<Vec<u8>> = vec![vec![48, 51], vec![49, 52], vec![50, 53]];
        assert_eq!(dst, want);
    }

    // TODO Use correct data.
    #[test]
    fn test_find_key() {
        let src: Vec<Vec<u8>> = vec![vec![48, 49, 50], vec![51, 52, 53]];
        let got = find_key(&src);
        let want: Vec<u8> = vec![0, 0];
        assert_eq!(got, want);
    }
}
