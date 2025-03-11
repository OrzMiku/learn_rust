/*
    Author: OrzMiku
    Hello there! This is a simple Rust program. It's my first step to learn Rust.
    It's contains:
        - Hello World
        - Variables
        - Mutability
        - Constants
        - Static Variables
        - Data Types
        - Arrays
        - Tuples
        - Unit Tuple
*/

fn main() {
    // Hello World
    /*
    Println! is a macro, all macros in Rust end with a !.
     */
    println!("Hello, world!");

    // Variables
    // In rust, variables are declared using the let keyword. The type of the variable is inferred by the compiler.
    let x = 5;
    // Also, we can use the : to specify the type of the variable.
    let y: i32 = 10;
    // We can shadow a variable by declaring a new variable with the same name.
    // let y: f32 = 1.01;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    
    // Mutability
    // By default, variables are immutable in Rust.
    // x = 6; // This will throw an error. Because x is immutable.
    // To make a variable mutable, we can use the mut keyword.
    let mut z = 5;
    println!("The value of z is: {}", z);
    z = 6;
    println!("Changed value of z is: {}", z);

    // Constants
    // Constants are something that never changes. Constants are declared using the const keyword.
    // Constants can be declared in global scope, but variables always need to be declared in a local scope.
    // The value of a constant must be determined at compile time. So we can't use a function or expression that is computed at runtime.
    // const MAX_VALUE : i32 = get_max_value(); // This will throw an error.
    // And the type of a constant must be specified.
    // const MAX_VALUE = 100; // This will throw an error.
    const MAX_VALUE : i32 = 100;
    println!("The value of MAX_VALUE is: {}", MAX_VALUE);

    // Static Variables
    // Static variables are similar to constants. It can also be used in global scope and the type of a static variable must be specified like constants.
    // But here are some differences between static variables and constants.
    // Static variables can be mutable. But constants are always immutable.
    // Notice that the mutable static variable must be declared as unsafe, because it can be changed by multiple threads.
    // Static variables have a fixed memory location. But constants are inlined wherever they are used.
    static mut MIN_VALUE : i32 = 0;
    unsafe {
        // Access the mutable static directly inside the unsafe block
        let value = MIN_VALUE;
        println!("The value of MIN_VALUE is: {}", value);
        MIN_VALUE = -100;
        let new_value = MIN_VALUE;
        println!("Changed value of MIN_VALUE is: {}", new_value);
    }

    // Data Types
    // I really don't want to show all the data types here. So I will just show some specific ones I think are important.
    // If you want your code is platform independent, you can use the isize and usize types.
    // The real bit size of these types depends on the platform. On a 64-bit system, isize and usize are 64-bit.
    // On a 32-bit system, isize and usize are 32-bit.
    let _a: isize = 10;
    // If a variable will never be used. You can use the _ to ignore the warning.
    let _b: usize = 10;

    // Arrays
    // Arrays in Rust have a fixed length. Once declared, the length of the array cannot be changed.
    // We can declare an array using the [] operator. The type of the array is [type; length].
    let arr : [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of arr is: {:?}", arr);
    // We can access the elements of an array using the [] operator.
    let first = arr[0];
    println!("The value of first is: {}", first);
    // We can also use the get method to access the elements of an array.
    let second = arr.get(1);
    println!("The value of second is: {:?}", second);
    // Notice that the get method returns an Option type. This is because the index might be out of bounds.
    // If the index is out of bounds, the get method will return None. It's better to use the get method to avoid panics.
    // let third = arr[10]; // This will panic.
    let tenth = arr.get(10);
    println!("The value of tenth is: {:?}", tenth); // This will return None.
    
    // Tuples
    // Tuples are a way to group different types of values together. (Arrays can only have one type of value)
    let tup = (1, 2.0, "Hello");
    println!("The value of tup is: {:?}", tup);
    // We can access the values of a tuple using the . operator.
    let str = tup.2;
    println!("The value of str is: {}", str);
    // We can also destructure a tuple.
    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
    // Unit tuple
    // A tuple with no elements is called a unit tuple. () is a type (unit type) and also a value of the unit tuple.
    let _tup = ();
}