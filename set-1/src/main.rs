mod challenges;
use crate::challenges::eight::challenge_eight;
use crate::challenges::five::challenge_five;
use crate::challenges::four::challenge_four;
use crate::challenges::one::challenge_one;
use crate::challenges::seven::challenge_seven;
use crate::challenges::six::challenge_six;
use crate::challenges::three::challenge_three;
use crate::challenges::two::challenge_two;
use common::io::read_input;

fn main() {
    loop {
        let input = read_input("Please select challenge to run:(1-8)");
        let challenge: usize = input
            .parse()
            .unwrap_or_else(|_| panic!("Invalid input: __{}__. Expecting a number (1-8)", input));

        match challenge {
            1 => return challenge_one(),
            2 => return challenge_two(),
            3 => return challenge_three(),
            4 => return challenge_four(),
            5 => return challenge_five(),
            6 => return challenge_six(),
            7 => return challenge_seven(),
            8 => return challenge_eight(),
            _ => println!("Invalid challenge. Must be 1-8."),
        }
    }
}
