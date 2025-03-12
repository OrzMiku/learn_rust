/*
    Author: OrzMiku
    Here are:
        - Ownership
        - Move
        - Copy
        - Move to function
        - Copy to function
        - Return ownership
*/

fn main() {
    // s1 is the owner of the string.
    let s1 : String = String::from("Hello, world!");
    println!("s1 is {}", s1);

    // String is a heap-allocated string, so it can be moved implicitly by the compiler.
    // In rust, the resource can only be owned by one variable.
    // After the move, the original variable is no longer valid. 
    let s2 = s1;
    // println!("{}", s1); // Error: value used here after move
    println!("s2 is {}", s2);

    // If you want to copy the value, you can use the clone method.
    // Notice that the string s3 owns is a new string, not the same as s2.
    let s3 = s2.clone();
    println!("s2 is {}", s2); // s2 is still valid.
    println!("s3 is {}", s3);

    // The default performance of the basic data types that are stored on the stack is to copy, not move.
    let i1 = 1;
    let i2 = i1; // Copy
    println!("i1 is {}", i1); // i1 is still valid.
    println!("i2 is {}", i2);

    // The function can also move the value.
    let s4 = String::from("Hello, world!");
    print_string(s4);
    // println!("{}", s4); // Error: value used here after move, the ownership has been moved to the s(inside the function).

    // The basic data type is copied to the function, so the original variable is still valid.
    let i3 = 1;
    print_integer(i3);
    println!("{}", i3); // i3 is still valid.

    // The ownership can be returned.
    let mut s5 = get_string();
    println!("{}", s5);
    
    // If you want to use the value after a function call, you can use the clone method.
    // s5 will keep the ownership of the string. s5.clone() will create a new string and the ownership of the new string will be moved to the function. 
    print_string(s5.clone());
    println!("{}", s5);

    // If you want to keep the ownership of the string, you can also use the return value of the function.
    // The ownership of the string will be moved back to the caller.
    // s5 -> s of the function -> s5
    // Notice that s5 should be mutable.
    s5 = add_to_string(s5);
    println!("{}", s5);

}

fn print_string(s: String) {
    println!("{}", s);
} // s is dropped here.

fn print_integer(i: i32) {
    println!("{}", i);
} // i is dropped here. But the basic data type is copied, so the original variable is still valid.

fn get_string() -> String {
    let s = String::from("Hello, world!");
    s
} // The ownership of s will be moved to the caller.

fn add_to_string(s: String) -> String {
    let s = s + "!";
    s
} // The ownership of s will be moved to the caller.