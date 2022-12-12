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

fn is_valid_user(user: &str) -> bool {
    user.trim().to_lowercase() == user
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid() {
        assert!(is_valid_user("herbert"));
    }

    #[test]
    fn test_valid_case() {
        assert!(is_valid_user("HeRbErT\r\n"));
    }

    #[test]
    fn test_invalid() {
        assert!(!is_valid_user("Bob"));
    }
}