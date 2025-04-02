
#[derive(thiserror::Error, Debug)]
pub enum MyError {
    #[error("Division by zero")]
    DivisionByZero,
    #[error("Invalid input: {0}")]
    InvalidInput(String),
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

fn main() {
    match divide(10, 0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match parse_input("abc") {
        Ok(result) => println!("Parsed: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}