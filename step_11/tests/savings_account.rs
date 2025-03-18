use step_11::SavingsAccount;

// This module is for unit tests. It is compiled as part of the library crate
// It stored in the utils directory, not in the utils.rs file (If it was in the utils.rs file, it would be treated as a test module)
mod utils;

// {root_dir}/tests/*.rs are treated as integration tests
// All the rs files will be compiled as a separate crate
// Integration tests are going to be external to our library and will only test the public interface
// Unit are meant to test small units of code. Integration tests test the interaction between multiple units of code.
#[test]
fn should_have_a_starting_balance_of_0() {
    utils::common_setup();
    let account = SavingsAccount::new();
    assert_eq!(account.get(), 0, "Expected starting balance to be 0");
}