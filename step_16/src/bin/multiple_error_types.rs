use std::{fs, error::Error};

fn main() {
    let file_1 = "data/i32.txt";
    let file_2 = "data/hello.txt";
    let invalid_file = "data/invalid_file.txt";
    match parse_file_1(file_1) {
        Ok(contents) => println!("{}", contents),
        Err(err) => println!("Error: {}", err),
    }

    match parse_file_2(file_1) {
        Ok(contents) => println!("{}", contents),
        Err(ParseFileError::IoError(err)) => println!("IoError: {}", err),
        Err(ParseFileError::ParseIntError(err)) => println!("ParseIntError: {}", err)
    }

    match parse_file_2(file_2) {
        Ok(contents) => println!("{}", contents),
        Err(ParseFileError::IoError(err)) => println!("IoError: {}", err),
        Err(ParseFileError::ParseIntError(err)) => println!("ParseIntError: {}", err)
    }

    match parse_file_2(invalid_file) {
        Ok(contents) => println!("{}", contents),
        Err(ParseFileError::IoError(err)) => println!("IoError: {}", err),
        Err(ParseFileError::ParseIntError(err)) => println!("ParseIntError: {}", err)
    }
}

// Because fs::read_to_string returns a Result<String, io::Error>
// and parse() returns a Result<i32, std::num::ParseIntError>
// we need to use Box<dyn Error>
// Box<dyn Error> is a trait object that can represent any type that implements the Error trait
fn parse_file_1(filename: &str) -> Result<i32, Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let parse = contents.parse()?;
    Ok(parse)
}

// We can define our own error type to handle multiple error types
// We can use an enum to define the different types of errors
// We can then match on the enum to handle the different error types
// This is more flexible than using Box<dyn Error>
enum ParseFileError {
    IoError(std::io::Error),
    ParseIntError(std::num::ParseIntError),
}

fn parse_file_2(filename: &str) -> Result<i32, ParseFileError> {
    let contents = fs::read_to_string(filename).map_err(ParseFileError::IoError)?;
    let parse = contents.parse().map_err(ParseFileError::ParseIntError)?;
    Ok(parse)
}