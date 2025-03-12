/*
    Author: OrzMiku
    Here are:
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
*/

// The struct is a custom data type that lets you create a named collection of fields.
struct Product {
    name: String,
    price: f32,
    in_stock: bool,
}

// implementation is a block of code that defines the behavior of a struct.
impl Product {
    // This is a constructor. It returns a struct instance.
    // The name of the constructor is new. It is a convention in Rust. But you can use any name.
    fn new(name: String, price: f32, in_stock: bool) -> Product {
        Product {
            name,
            price,
            in_stock
        }
    }

    // This is a associated function. It does not take &self or &mut self as a parameter.
    // It's a instance-independent function like a static method in other programming languages.
    fn get_default_tax() -> f32 {
        0.1
    }

    // &self is a reference to the struct instance.
    fn tax(&self) -> f32 {
        // get_default_tax() is a associated function. So it should be called by using the Product:: prefix.
        self.price * Product::get_default_tax()
    }

    // &mut self is a mutable reference to the struct instance.
    fn set_price(&mut self, price: f32) -> f32 {
        let old_price = self.price;
        self.price = price;
        old_price
    }

    // self is the struct instance. It is consumed by the function. So the struct instance cannot be used anymore.
    fn buy(self) -> i32 {
        let order_id = 114514;
        println!("You bought {}", self.name);
        order_id
    }
}

// Tuple struct
struct Color(u8, u8, u8);

// Tuple struct implementation
impl Color {
    fn luminance(&self) -> f32 {
        0.299 * self.0 as f32 + 0.587 * self.1 as f32 + 0.114 * self.2 as f32
    }
}

fn main() {
    // Create a struct instance.
    let mut book = Product {
        name: String::from("The Rust Programming Language"),
        price: 54.99,
        in_stock: true
    };

    // Use . to access the field of the struct instance.
    println!("The price of {} is {}", book.name, book.price);

    // Change the value of the field. The struct instance should be mutable.
    book.in_stock = false;

    // Call the method of the struct instance.
    let tax = book.tax();
    println!("The tax for {} is {}", book.name, tax);

    // Call the method that changes the value of the field.
    let old_price = book.set_price(59.99);
    println!("The price of {} was changed from {} to {}", book.name, old_price, book.price);

    // Call the method that consumes the struct instance.
    let order_id = book.buy();
    // println!("The price of {} is {}", book.name, book.price); // Error: value used here after move
    println!("The order id is {}", order_id);

    // Create a struct instance by using the constructor.
    book = Product::new(String::from("The Rust Programming Language"), 54.99, true);
    book.buy();

    let blue : Color = Color(0, 0, 255);
    let luminance: f32 = blue.luminance();
    println!("The luminance of blue is {}", luminance);
}
