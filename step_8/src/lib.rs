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

mod database {
    pub enum Status {
        Connected,
        Interrupted
    }
    
    pub fn connect_to_database() -> Status {
        return Status::Connected;
    }

    pub fn get_user() {
        // ...Get the user
    }
}

mod auth_utils {
    // You can use relative paths to refer to the parent module
    pub fn login(creds : models::Credentials) {
        // ...Authenticate the user

        // You can also use super to refer to the parent module
        // super::database::get_user();

        // Or you can use the full path
        // crate:: is an absolute path that starts from the crate root
        crate::database::get_user();
    }
    
    fn logout() {
        // ...Logout the user
    }

    pub(crate) mod models {
        pub struct Credentials {
            username: String,
            password: String,
        }
    }
}

// The use keyword brings a path into scope
// It is similar to the import keyword in other languages
use database::connect_to_database;
use auth_utils::login;
use database::Status;
use auth_utils::models::Credentials;

pub fn authenticate(creds : Credentials) {
    if let Status::Connected = connect_to_database() {
        login(creds);
    } else {
        panic!("Failed to connect to the database");
    }
}
