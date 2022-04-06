mod challenges;
use crate::challenges::eleven::challenge_eleven;
use crate::challenges::nine::challenge_nine;
use crate::challenges::ten::challenge_ten;
use crate::challenges::twelve::challenge_twelve;
use common::io::read_input;

fn main() {
    loop {
        let input = read_input("Please select challenge to run:(9-16)");
        let challenge: usize = input
            .parse()
            .unwrap_or_else(|_| panic!("Invalid input: __{}__. Expecting a number (9-16)", input));

        match challenge {
            9 => return challenge_nine(),
            10 => return challenge_ten(),
            11 => return challenge_eleven(),
            12 => return challenge_twelve(),
            13..=16 => println!("Challenge unimplemented"),
            _ => println!("Invalid challenge. Must be 9-16."),
        }
    }
}
