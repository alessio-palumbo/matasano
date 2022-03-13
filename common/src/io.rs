use std::io;

/// Prints a message then return the user's input stripped of whitespaces.
pub fn read_input(msg: &str) -> String {
    let mut input = String::new();
    println!("{}", msg);

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}
