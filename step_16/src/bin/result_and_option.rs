use std::{io, fs};

fn main() {
    let line_1 = read_first_line_1("src/bin/result_and_option.rs");

    match line_1 {
        Ok(contents) => println!("{}", contents),
        Err(err) => println!("Error: {}", err),
    }

    let line_2 = read_first_line_2("src/bin/result_and_option.rs");

    match line_2 {
        Ok(Some(contents)) => println!("{}", contents),
        Ok(None) => println!("No first line found"),
        Err(err) => println!("Error: {}", err),
    }

    let line_3 = read_first_line_3("src/bin/result_and_option.rs");

    match line_3 {
        Some(contents) => println!("{}", contents),
        None => println!("No first line found"),
    }
}

fn read_first_line_1(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
        // and_then is required to return the same Result type
        // fs::read_to_string returns a Result<String, io::Error>
        // so and_then must return a Result<String, io::Error>
        .and_then(|contents| {
            contents.lines().next().map_or(
                Err(io::Error::new(io::ErrorKind::Other, "No first line found")),
                |s| Ok(s.to_string())
            )
        })
}

fn read_first_line_2(path: &str) -> Result<Option<String>, io::Error> {
    // If use map instead of and_then, we only pay attention to the success case
    // the error will return early
    fs::read_to_string(path)
        .map(|content| {
            content.lines().next().map(|s| s.to_string())
        })
}

fn read_first_line_3(path: &str) -> Option<String> {
    // We can use ok() to convert the Result to an Option
    fs::read_to_string(path)
        .ok()
        .and_then(|content| content.lines().next().map(|s| s.to_string()))
}