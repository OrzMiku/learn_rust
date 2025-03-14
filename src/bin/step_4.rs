/*
    Author: OrzMiku
    Here are:
        - String and &str
        - String -> &str
        - &str -> String
        - Modify the string
        - Concatenate the string
        - Format the string
        - Concat macro
        - String indexing
*/

fn main() {
    // String literal is a string slice pointing to the binary.
    let s1 : &str = "Hello Rust!";

    // &str -> String
    let s2 : String = String::from(s1); // s2 is String
    let s3 : String = s1.to_string(); // s3 is String
    let s4 : String = s1.to_owned(); // s4 is String

    // String -> &str
    let s5 : &str = &s2; // &String -> &str is automatically done by the compiler.
    let s6 = &s2; // s6 is &String
    let s7 = &s2[..]; // s7 is &str, String slice

    println!("s1 is {}", s1);
    println!("s2 is {}", s2);
    println!("s3 is {}", s3);
    println!("s4 is {}", s4);
    println!("s5 is {}", s5);
    println!("s6 is {}", s6);
    println!("s7 is {}", s7);

    // Modify the string
    let mut s8 = String::from("Hello, ");
    s8.push_str("world");
    println!("{}", s8);
    s8.replace_range(0..5, "Hi");
    println!("{}", s8);

    // Concatenate the string
    // The left side of the + operator should be a String, not a &str.
    // The right side of the + operator should be a &str, not a String.
    // The + operator needs to konw the ownership of the result. The left side of the + operator offers the ownership.
    // The right side of the + operator does not offer the ownership. If the right side is a String, the ownership will be consumed.
    let s9 = String::from("!");
    let s10 = s8 + &s9; // s8 is moved, so s8 cannot be used anymore.
    println!("{} was added to the tail.", s9);
    println!("{}", s10);

    // Format the string
    // Notice that format! macro is slower than the + operator. 
    // Because the format! macro copies the string (that is why the ownership is not moved).
    let s11 = format!("{}{}", s10, s9); // The ownership of s10 and s9 is not moved. Because the format! macro does not take the ownership.
    println!("s9 is {}", s9); // s9 is still valid.
    println!("s10 is {}", s10); // s10 is still valid.
    println!("s11 is {}", s11);

    // Concat macro
    // The concat! macro returns a string slice, not a String.
    let s12 : &str = concat!("Hello, ", "world!");
    println!("s12 is {}", s12);

    // String indexing
    // String uses the UTF-8 encoding. A character can be multiple bytes (1 to 4 bytes).
    // If we use the index to get the character, it may not be valid.
    let s13 : String = String::from("😊Hello!");
    // println!("{}", s13[0]); // Error: cannot index into a value of type `String`
    // But if we konw the character boundary, we can use the range to get the character.
    println!("{}", &s13[0..4]); // 😊
    // If we do not konw the character boundary, we can use the chars() method to get the character.
    for c in s13.chars() {
        println!("{}", c);
    }
}
