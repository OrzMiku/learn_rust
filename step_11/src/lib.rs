/// A savings account
pub struct SavingsAccount {
    balance: i32,
}

impl SavingsAccount {
    // This is a documentation comment, which can be used to generate documentation pages
    // To generate documentation, run `cargo doc --open`
    // You can use `///` to write documentation comments (Support Markdown)
    /// Create a new `SavingsAccount` with a starting balance of zero
    /// 
    /// # Examples
    /// 
    /// ```
    /// use step_11::SavingsAccount;
    /// let account = SavingsAccount::new();
    /// assert_eq!(account.get(), 0, "Expected starting balance to be 0");
    /// ```
    // The code block above will be compiled and run as a doctest
    pub fn new() -> Self {
        SavingsAccount { balance: 0 }
        // SavingsAccount { balance: 100 } // The test `should_have_a_starting_balance_of_0` will fail if this line is uncommented
    }
    
    pub fn get(&self) -> i32 {
        self.balance
    }

    pub fn disposit(&mut self, amount: i32) {
        // If the amount is negative, panic, then the test `should_panic_if_dispositing_negative_amount` will pass
        if amount < 0 {
            panic!("Cannot deposit a negative amount: {}", amount);
        }
        self.balance += amount;
    }

    pub fn withdraw(&mut self, amount: i32) -> Result<i32, String> {
        if amount > self.balance {
            Err(format!("Insufficient funds: {} requested, but only {} available", amount, self.balance))
        } else {
            self.balance -= amount;
            Ok(self.balance)
        }
    }

    pub fn transfer(&mut self, to: &mut SavingsAccount, amount: i32) -> Result<i32, String> {
        if amount > self.balance {
            Err(format!("Insufficient funds: {} requested, but only {} available", amount, self.balance))
        } else {
            self.balance -= amount;
            to.balance += amount;
            Ok(self.balance)
        }
    }
}

// Unit tests for the SavingsAccount struct
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_have_a_starting_balance_of_0() {
        let account = SavingsAccount::new();
        assert_eq!(account.get(), 0, "Expected starting balance to be 0");
    }

    #[test]
    fn should_be_able_to_deposit() {
        let mut account = SavingsAccount::new();
        account.disposit(100);
        assert_eq!(account.get(), 100, "Expected balance to be 100 after deposit");
        assert_ne!(account.get(), 0, "Expected balance to not be 0 after deposit");
        assert!(/* A boolean expression is true */ account.get() > 0, "Expected balance to be greater than 0 after deposit");
    }
    
    #[test]
    #[should_panic]
    // If the function `disposit` is called with a negative amount, it should panic
    // Then the test `should_panic_if_dispositing_negative_amount` will pass
    fn should_panic_if_dispositing_negative_amount() {
        let mut account = SavingsAccount::new();
        account.disposit(-100);
    }

    #[test]
    fn should_transfer_money() {
        let mut account_1 = SavingsAccount::new();
        let mut account_2 = SavingsAccount::new();
        account_1.disposit(100);
        let result = account_1.transfer(&mut account_2, 50);
        assert_eq!(result, Ok(50), "Expected transfer to be successful");
        assert_eq!(account_1.get(), 50, "Expected account_1 balance to be 50 after transfer");
        assert_eq!(account_2.get(), 50, "Expected account_2 balance to be 50 after transfer");
    }
}