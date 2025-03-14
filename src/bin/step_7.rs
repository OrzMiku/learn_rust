/*
    Author: OrzMiku
    Here are:
        - Create a vector 
        - Add elements to the vector
        - Access the element of the vector
            - Use the &vec[index]
            - Use the get method
        - Iterate over the vector
        - Move the ownership of the vector
*/

// Vec is a growable array stored on the heap.
fn main() {
    // If you want to create a vector by using the Vec::new() function, you need to specify the type of the vector.
    // The type of the vector is Vec<String> in this case.
    let mut v1 : Vec<String> = Vec::new();

    // You can also use the vec! macro to create a vector.
    // It agrees to infer the type of the vector. And you can initialize the vector with some elements.
    let v2 = vec!["one", "two", "three"];

    // You can also use the push method to add elements to the vector.
    v1.push(String::from("Hello"));
    v1.push(String::from("World"));

    // Use the &vec[index] to access the element of the vector may cause a panic if the index is out of bounds.
    // It's unsafe because the compiler can't check the index at compile time.
    let _s1 = &v1[0];
    // And "let s1 = v1[0];" will cause a compile error.
    // If "let s1 = v1[0];" is allowed, the ownership of the element will be transferred to s1.
    // Then the index 0 of the vector will be invalid.
    // let s1 = v1[0]; // That will cause a compile error.
    // If you want transfer the ownership of the element to a variable, you can use the remove method.
    let s1 = v1.remove(0);
    println!("s1 is {}", s1);

    // A safe way to access the element of the vector is to use the get method.
    // It returns an Option<&T> (Not Option<T>).
    // If the index is out of bounds, it will return None.
    match v2.get(1) {
        Some(s) => println!("{}", s),
        None => println!("Out of bounds")
    }
    match v2.get(100) {
        Some(s) => println!("{}", s),
        None => println!("Out of bounds")
    }

    // You can use the for loop to iterate over the vector.
    for s in &v2 {
        println!("{}", s);
    }

    // Here use the v2 (not &v2) to move the ownership of the vector to v1.
    // Then you can't use v2 anymore.
    for s in v2 {
        v1.push(s.to_string());
    }

    for s in &v1 {
        println!("{}", s);
    }
}
