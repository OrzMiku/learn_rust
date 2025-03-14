/*
    Author: OrzMiku
    The another_binary_crate.rs is another entry point of a binary crate. The file should be placed in the src/bin directory.
    The binary crate's name is the same as the file's name.
    In this case, the project's name is "step_8" and this crate's name is "another_binary_crate".
    If there are multiple binary crates in a project, it needs to use the --bin flag to specify which binary crate to run.
    Run this crate: cargo run --bin another_binary_crate
*/

fn main() {
    println!("This is another binary crate");
}