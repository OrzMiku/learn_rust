# Learn Rust

This repository contains the code which I wrote while learning Rust. If there is any mistake in the code or explanation, please let me know. I will be happy to correct it.

## Run the code

```bash
cargo run --bin step_x
```

## Index

- [step_1](src/bin/step_1.rs)
    - Hello World
    - Variables
    - Mutability
    - Constants
    - Static Variables
    - Data Types
    - Arrays
    - Tuples
    - Unit Tuple
- [step_2](src/bin/step_2.rs)
    - Function and function call
    - If Else Statement
    - Loop
    - While loop
    - For loop and Range
- [step_3](src/bin/step_3.rs)
    - [Smart Pointer implementation in C++](src/others/step_3/raii_cpp)
    - Ownership
    - Move
    - Copy
    - Move to function
    - Copy to function
    - Return ownership
    - Borrowing (reference)
    - Immutable and mutable reference
    - String slice
- [step_4](src/bin/step_4.rs)
    - String and &str
    - String -> &str
    - &str -> String
    - Modify the string
    - Concatenate the string
    - Format the string
    - Concat macro
    - String indexing
- [step_5](src/bin/step_5.rs)
    - Struct
    - Tuple struct
    - Implementation
        - Constructor
        - Associated function
        - Method
            - &self
            - &mut self
            - self
    - Use struct
        - Create a struct instance
        - Access the field
        - Change the value of the field
        - Call the method
        - Create a struct instance by using the constructor
- [step_6](src/bin/step_6.rs)
    - Enum
    - Enum Implementation
        - Usage of match in enum implementation
    - Use enum
        - Create a enum instance
        - Use enum in the struct
        - Call the method of the enum instance
    - Option enum
    - Result enum
- [step_7](src/bin/step_7.rs)
    - Create a vector 
    - Add elements to the vector
    - Access the element of the vector
        - Use the &vec[index]
        - Use the get method
    - Iterate over the vector
    - Move the ownership of the vector