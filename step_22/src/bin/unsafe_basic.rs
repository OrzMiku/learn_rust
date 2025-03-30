static mut COUNTER: u32 = 0;

fn main() {
    let mut s = "Hello".to_string();
    // Creating a raw pointer is safe
    let raw_1 = &s as *const String;
    let raw_2 = &mut s as *mut String;
    let address= 0x12345usize;
    let _raw: *const i32 = address as *const i32;

    // but dereferencing it is unsafe
    unsafe {
        println!("raw_1: {}", *raw_1);
        (*raw_2).push_str(" world");
        println!("raw_2: {}", *raw_2);

        // Dereferencing a raw pointer to an invalid address is undefined behavior
        // println!("raw: {}", *raw);

        // unsafe function is to be called in an unsafe block
        my_function();
        s.my_unsafe_method();

        // Incrementing the STATIC COUNTER
        // Changing the value of a static variable is unsafe
        // It's will be unsafe to access the static variable from multiple threads without proper synchronization
        increment_counter();
        increment_counter();
        increment_counter();
        let counter = COUNTER;
        println!("Counter: {}", counter);
    }
}

// Unsafe function
unsafe fn my_function(){
    println!("Hello from my_function");
}

// Unsafe trait
unsafe trait MyUnsafeTrait {
    // Unsafe trait methods
    unsafe fn my_unsafe_method(&self);
}

// Implementing the unsafe trait for String
// This is an example of how to implement an unsafe trait for a type
unsafe impl MyUnsafeTrait for String {
    unsafe fn my_unsafe_method(&self) {
        println!("Unsafe method called on String: {}", self);
    }
}

unsafe fn increment_counter() {
    unsafe {
        COUNTER += 1;
    }
}