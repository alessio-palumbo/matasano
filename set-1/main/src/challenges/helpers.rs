use crate::challenges::one::challenge_one;
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
            3..=8 => println!("Challenge not yet solved."),
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
