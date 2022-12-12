use std::io;

pub fn greet_user() -> String {
    println!("Hello, what is your name?");
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer).unwrap();
    buffer
}
