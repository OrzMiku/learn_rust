/*
    Author: OrzMiku
    The lib.rs is the entry point of a library crate.
    One project can only have one library crate.
    Here is a example of a library crate.

    Different from other programming languages, the module system is NOT based on the file system in Rust.
    The module system is based on the module tree by using the mod keyword.
    The visibility of the modules in Rust:
        - private by default
        - public with the pub keyword
        - pub(crate) to make the module public only within the current crate

    Here is a useful tool to visualize the module structure: cargo-modules
    Install it with the following command: cargo install cargo-modules
    And show the module tree with the following command:  cargo modules structure --crate_name
    For example: cargo modules structure --lib
*/

#![allow(dead_code, unused_variables)]

// The mod keyword is used to define a module.
// Although the module system is not based on the file system, it is a good practice to create a module in a separate file.
// When you create a module in a separate file, you still need to use the mod keyword to define the module.
mod database; // database/mod.rs is a way to organize the code.
pub mod auth_utils; // auth_utils.rs is a another way to organize the code.

// The use keyword brings a path into scope
// It is similar to the import keyword in other languages
use database::connect_to_database;
use auth_utils::login;
use database::Status;
use auth_utils::models::Credentials;

// Rand is an external dependency, you need to add it manually in the Cargo.toml or use the command cargo add rand
use rand::Rng;

pub fn authenticate(creds : Credentials) -> Result<String, String> {
    let mut rng = rand::rng();
    let timeout = rng.random_range(100..=500);
    println!("Timeout: {}", timeout);

    if let Status::Connected = connect_to_database() {
        login(creds);
        return Ok(String::from("Successfully authenticated."));
    } else {
        return Err(String::from("Cannot connect to the database."));
    }
}
