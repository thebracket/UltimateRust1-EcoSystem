use std::io;
use anyhow::{Result, Error};

pub fn greet_user() -> Result<String> {
    println!("Hello, what is your name?");
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;
    if buffer.trim().to_lowercase() != "herbert" {
        Err(Error::msg("Access denied!"))
    } else {
        Ok(buffer)
    }
}
