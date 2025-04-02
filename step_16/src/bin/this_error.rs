use std::io;

#[derive(thiserror::Error, Debug)]
pub enum MyError {
    #[error("Division by zero")]
    DivisionByZero,
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
}

pub fn divide(a: i32, b: i32) -> Result<i32, MyError> {
    if b == 0 {
        Err(MyError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

pub fn parse_input(input: &str) -> Result<i32, MyError> {
    input
        .parse::<i32>()
        .map_err(|_| MyError::InvalidInput(input.to_string()))
}

pub fn read_file(path: &str) -> Result<String, MyError> {
    let content = std::fs::read_to_string(path)?;
    Ok(content)
}

fn main() {
    match divide(10, 0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match parse_input("abc") {
        Ok(result) => println!("Parsed: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match read_file("non_existent_file.txt") {
        Ok(content) => println!("File content: {}", content),
        Err(e) => println!("Error: {}", e),
    }
}