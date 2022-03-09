use crate::challenges::five::challenge_five;
use crate::challenges::four::challenge_four;
use crate::challenges::one::challenge_one;
use crate::challenges::seven::challenge_seven;
use crate::challenges::six::challenge_six;
use crate::challenges::three::challenge_three;
use crate::challenges::two::challenge_two;
use std::io;

/// Select the challenge to run in range 1-8.
/// It panics if the input is not an integer.
pub fn select_challenge() {
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
            8 => println!("Challenge not yet solved."),
            _ => println!("Invalid challenge. Must be 1-8."),
        }
    }
}

/// Prints a message then return the user's input stripped of whitespaces.
pub fn read_input(msg: &str) -> String {
    let mut input = String::new();
    println!("{}", msg);

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

/// Only look for an Ok Result otherwise continue outer loop.
macro_rules! ok_or_continue {
    ($res:expr) => {
        match $res {
            Ok(v) => v,
            Err(_) => continue,
        }
    };
}

pub(crate) use ok_or_continue;
