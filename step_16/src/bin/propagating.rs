use std::{fs::File, io::{self, Read}};

fn main() {
    let filename = "src/bin/propagating.rs";
    // let filename = "invalid_file.txt";
    match read_file(filename) {
        Ok(contents) => println!("{}", contents),
        Err(err) => println!("Error: {}", err),
    }

    let user = User {
        firstname: "John".to_string(),
        lastname: "Doe".to_string()
    };
    match get_initials(user) {
        Some(initials) => println!("Initials: {}", initials),
        None => println!("Error: Could not get initials"),
    }
}

fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut contents = String::new();
    // The ? operator can be used to propagate errors
    // It will return the error to the caller early
    File::open(filename)?.read_to_string(&mut contents)?;
    Ok(contents)
}

struct User {
    firstname: String,
    lastname: String
}

fn get_initials(user: User) -> Option<String> {
    // The ? operator can be used to propagate errors
    // It will return the None to the caller early
    let first_initial = user.firstname.chars().next()?;
    let last_initial = user.lastname.chars().next()?;
    Some(format!("{}.{}.", first_initial, last_initial))
}