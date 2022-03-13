use common::io::read_input;
use crypto::padding::pkcs_hash7_padding;

pub fn challenge_nine() {
    println!("\n# Challenge 9 #");
    loop {
        let src = read_input("Please input string to be padded:");
        let size = read_input("Please input block size:");

        let padded = pkcs_hash7_padding(src.as_bytes(), size.parse::<usize>().unwrap());
        println!(
            "Padded input:\n>>>> {:?}",
            String::from_utf8(padded).unwrap()
        );
    }
}
