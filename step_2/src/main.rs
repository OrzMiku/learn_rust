/*
    Author: OrzMiku
    Here are:
        - Function and function call
        - If Else Statement
        - Loop
        - While loop
        - For loop and Range
*/

fn main() {
    // Function call
    let a = 5;
    let b: i32 = 10;
    println!("The value of a is: {}", a);
    println!("The value of b is: {}", b);
    println!("The value of a / b is: {}", divide(a as f32, b as f32));

    // If Else Statement
    // Not like other programming languages, If Else statement in Rust doesn't require parentheses.
    let a = 5;
    if a > 5 {
        println!("a is greater than 5");
    } else if a < 5 {
        println!("a is less than 5");
    } else {
        println!("a is equal to 5");
    }

    // And also, in rust dosn't have a ternary operator. But we can use if else statement to do the same thing.
    // All the branches of an if statement must return the same type.
    let b = if a > 5 { 10 } else { 0 };
    println!("The value of b is: {}", b);

    // Simple loop
    // Loop is an infinite loop. We should use break to exit the loop.
    // And loop can be named by using the 'name: before the loop keyword.
    let mut i = 0;
    'outer: loop {
        println!("Outer loop");
        loop {
            println!("Inner loop");
            i += 1;
            if i == 5 {
                break 'outer;
            }
        }
        // println!("This line will never be executed"); // Because the loop is already broken.
    }

    // While loop
    // While loop is similar to other programming languages.
    let mut num = 0;
    while num < 5 {
        println!("The value of num is: {}", num);
        num += 1;
    }

    // For loop
    let arr = [5, 10, 15, 20, 25];
    for i in arr.iter() {
        println!("The value of i is: {}", i);
    }

    // Range
    // We can use the .. operator to create a range.
    // The range is inclusive on the left and exclusive on the right. So 0..5 is equal to [0, 5).
    for i in 0..5 {
        println!("The value of arr[{}] is: {}", i, arr[i]);
    }
}

// Using the fn keyword to declare a function.
// The return type of a function is specified after the -> operator.
// The return type of a function is optional. If the return type is not specified, the function will return the unit type ().
// The last expression in a function is the return value of the function and it's not necessary to use the return keyword. (But need to NOT use the ;)
// If you want to return a value early, you can use the return keyword.
fn divide(x: f32, y: f32) -> f32 {
    if y == 0.0 {
        println!("Error: Division by zero!");
        // Return early
        return 0.0;
    }
    // Return the result of the division
    x / y
}