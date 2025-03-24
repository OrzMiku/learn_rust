struct User<T> where T: Fn(&str, &str) -> bool {
    username: String,
    password: String,
    validator: T
}

impl<T> User<T> where T: Fn(&str, &str) -> bool {
    fn new(username: String, password: String, validator: T) -> User<T> {
        User{username: username, password: password, validator}
    }

    fn is_valid(&self) -> bool {
        (self.validator)(&self.username, &self.password)
    }
}

fn main() {
    let validator1 = |username: &str, password: &str| -> bool {
        username.len() > 0 && password.len() > 0
    };

    let validator2 = |username: &str, password: &str| -> bool {
        username.len() > 5 
        && password.len() > 8 
        && password.contains(['!', '@', '#', '$', '%', '^', '&', '*'].as_ref())
    };

    let user1 = User::new("user_1".to_string(), "password1".to_string(), validator1);
    let user2 = User::new("user_2".to_string(), "password2".to_string(), validator2);
    let user3 = User::new("user_3".to_string(), "passowrd3!".to_string(), validator2);

    println!("User1: {:?}", user1.is_valid());
    println!("User2: {:?}", user2.is_valid());
    println!("User3: {:?}", user3.is_valid());

    let default_user = get_default_user(validator1);
    println!("Default User: {:?}", default_user.is_valid());

    let pwd_validator = get_password_validator(3, false);
    let user4 = User::new("user_4".to_string(), "password4".to_string(), pwd_validator);
    println!("User4: {:?}", user4.is_valid());
}

fn get_default_user<T: Fn(&str, &str) -> bool>(validator: T) -> User<T> {
    User::new("default_user".to_string(), "default_password".to_string(), validator)
}

fn get_password_validator(min_length: usize, special_char: bool) -> Box<dyn Fn(&str, &str) -> bool> {
    if special_char {
        // move is necessary to take ownership of min_length
        // because it is a reference to a local variable which will be dropped after the function returns
        Box::new(move |_: &str, password: &str| -> bool {
            password.len() > min_length && password.contains(['!', '@', '#', '$', '%', '^', '&', '*'].as_ref())
        })
    } else {
        Box::new(move |_: &str, password: &str| -> bool {
            password.len() > min_length
        })
    }
}

// In practical applications, it is more recommended to code it this way. The above only show how to handle it if you need to return multiple closures.
// fn get_password_validator(min_length: usize, special_char: bool) -> impl Fn(&str, &str) -> bool {
//     move |_: &str, password: &str| -> bool {
//         if special_char {
//             password.len() > min_length && password.contains(['!', '@', '#', '$', '%', '^', '&', '*'].as_ref())
//         } else {
//             password.len() > min_length
//         }
//     }
// }
