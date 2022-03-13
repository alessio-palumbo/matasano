mod challenges;
use crate::challenges::nine::challenge_nine;
use common::io::read_input;

fn main() {
    loop {
        let input = read_input("Please select challenge to run:(9-16)");
        let challenge: usize = input
            .parse()
            .unwrap_or_else(|_| panic!("Invalid input: __{}__. Expecting a number (9-16)", input));

        match challenge {
            9 => return challenge_nine(),
            10..=16 => println!("Challenge unimplemented"),
            _ => println!("Invalid challenge. Must be 9-16."),
        }
    }
}
