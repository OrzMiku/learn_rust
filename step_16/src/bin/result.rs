fn main() {
    let users = vec!["admin", "user1", "user2"];
    for user in users {
        match get_user_id(user) {
            Ok(id) => println!("User ID: {}", id),
            Err(err) => println!("Error: {}", err), // Err Handler
        }
    }
}

// Err in Result is used to return an error message
// It's an error that can be recovered
fn get_user_id(username: &str) -> Result<u32, String> {
    if username == "admin" {
        Ok(1)
    } else {
        Err("User not found".to_string())
    }
}