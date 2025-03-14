/*
    Author: OrzMiku
    The main.rs is the default entry point of a binary crate.
    This binary crate's name is the same as the project's name.
    In this case, the project's name is "step_8" and this crate's name is "step_8".
    If there are multiple binary crates in a project, it needs to use the --bin flag to specify which binary crate to run.
    Run this crate: cargo run --bin step_8
*/

use step_8::auth_utils::models::Credentials;
use step_8::authenticate;

fn main() {
    let username = "username".to_string();
    let password = "password".to_string();
    let creds = Credentials::new(username, password);
    let result = authenticate(creds);
    match result {
        Ok(msg) => println!("{}", msg),
        Err(msg) => println!("{}", msg),
    }
}
