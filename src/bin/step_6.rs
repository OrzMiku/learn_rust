/*
    Author: OrzMiku
    Here are:
        - Enum
        - Enum Implementation
            - Usage of match in enum implementation
        - Use enum
            - Create a enum instance
            - Use enum in the struct
            - Call the method of the enum instance
        - Option enum
        - Result enum
*/

struct Product {
    name: String,
    // Enum can be used as a type of a field.
    categroy: Categroy,
    price: f32,
    in_stock: bool
}

impl Product {
    fn serialize(&self) -> String {
        format!("name: {}, categroy: {}, price: {}, in_stock: {}", self.name, self.categroy.serialize(), self.price, self.in_stock)
    }
}

// Enum is a custom data type that lets you define a type by enumerating its possible values.
enum Categroy {
    Book,
    Clothing,
    Electrics
}

// The item of enum can contain values.
enum Command {
    Undo,
    Redo,
    MoveCursor { x: i32, y: i32 },
    Replace {
        from: String,
        to: String
    }
}

// implementation is a block of code that defines the behavior of a struct. Also, it can be used for enum.
impl Categroy {
    fn serialize(&self) -> String {
        // Use match to access the item of the enum.
        // Match is a control flow operator like switch in other programming languages.
        match self {
            Categroy::Book => "Book".to_string(),
            Categroy::Clothing => "Clothing".to_string(),
            Categroy::Electrics => "Electronics".to_string(),
        }
    }
}

impl Command {
    fn serialize(&self) -> String {
        match self {
            Command::Undo => "Undo".to_string(),
            Command::Redo => "Redo".to_string(),
            // Use {} to access the value of the item.
            Command::MoveCursor { x, y } => format!("MoveCursor x: {}, y: {}", x, y),
            Command::Replace { from, to } => format!("Replace from: {}, to: {}", from, to)
        }
    }
}

// Option is a generic enum with two variants: Some<T> and None.
fn get_username(user_id: i32) -> Option<String> {
    if user_id == 1 {
        Some(String::from("OrzMiku"))
    } else {
        None
    }
}

fn get_username_dbquery(user_id: i32) -> Option<String> {
    let mut query : String = format!("SELECT username FROM users WHERE id = {}", user_id);
    
    // Simulate the situation that the query is empty.
    if user_id == 2 {
        query = String::from("");
    }

    let result = db_query(query);
    match result {
        Ok(username) => Some(username),
        Err(msg) => {
            println!("{}", msg);
            None
        }
    }
}

// Result is a generic enum with two variants: Ok<T> and Err<E>.
// It is used to handle the situation that the function may return an error.
fn db_query(query : String) -> Result<String, String> {
    if query.is_empty() {
        return Err(String::from("Query is empty"));
    } else {
        return Ok(String::from("OrzMiku"));
    }
}

fn main() {
    // Use :: to access the item of the enum.
    let categroy = Categroy::Electrics;
    
    // Use enum in the struct.
    let product_1 = Product {
        name: String::from("Laptop"),
        categroy, // You can omit the field name if the variable name is the same as the field name.
        price: 1000.0,
        in_stock: true
    };

    let product_2 = Product {
        name: String::from("T-shirt"),
        categroy: Categroy::Clothing,
        price: 20.0,
        in_stock: false
    };
    
    let product_3 = Product {
        name: String::from("Rust Programming"),
        categroy: Categroy::Book,
        price: 30.0,
        in_stock: true
    };

    println!("{}", product_1.serialize());
    println!("{}", product_2.serialize());
    println!("{}", product_3.serialize());

    // Create a enum instance by using the item of the enum.
    let cmd_undo = Command::Undo;
    let cmd_redo = Command::Redo;
    let cmd_move_cursor = Command::MoveCursor { x: 10, y: 20 };
    let cmd_replace = Command::Replace { from: "Hello".to_string(), to: "World".to_string() };

    // Call the method of the enum instance.
    println!("{}", cmd_undo.serialize());
    println!("{}", cmd_redo.serialize());
    println!("{}", cmd_move_cursor.serialize());
    println!("{}", cmd_replace.serialize());

    // Use Option to handle the situation that the function may return null.
    match get_username(2) {
        Some(username) => println!("Username: {}", username),
        None => println!("User not found")
    }

    // If you only care about one case, you can use if let to simplify the code.
    if let Some(name) = get_username(1) {
        println!("Username: {}", name);
    }

    let username = get_username_dbquery(1);
    if let Some(name) = username {
        println!("Username: {}", name);
    }

    let username = get_username_dbquery(2);

    // Because the query is empty, the query function will return an error.
    // And Some(username) will not be executed.
    if let Some(name) = username {
        println!("Username: {}", name);
    }
}
