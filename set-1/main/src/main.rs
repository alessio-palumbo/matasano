use hex;

fn main() {
    let h = "012".as_bytes();
    let b = hex::from_hex(&h).unwrap();
    println!("Hello, world! {:?}", b);
}
