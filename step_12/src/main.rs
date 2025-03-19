use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Command<T> {
    name: String,
    payload: T,
}

impl<T> Command<T> {
    fn new(name: String, payload: T) -> Self {
        Command {name, payload}
    }

    fn get_payload(&self) -> &T {
        &self.payload
    }
}

impl Command<String> {
    fn print_payload(&self) {
        println!("Payload: {}", self.payload);
    }
}

fn print_struct<T: std::fmt::Debug>(payload: &T) {
    println!("{:?}", payload);
}

fn main() {
    let cmd1: Command<Vec<String>> = Command::new(
        "goto".to_string(),
        vec![
            "x".to_string(),
            "y".to_string(),
            "z".to_string()
        ]
    );

    let cmd2: Command<i32> = Command::new(
        "zoom".to_string(),
        200
    );

    let cmd3: Command<String> = Command::new(
        "print".to_string(),
        "Hello, world!".to_string()
    );

    print_struct(&cmd1);
    print_struct(&cmd2);
    print_struct(&cmd3);
    
    // cmd1.print_payload(); // This will not work
    // cmd2.print_payload(); // This will not work
    let payload = cmd1.get_payload();
    println!("Payload of cmd1: {:?}", payload);
    let payload = cmd2.get_payload();
    println!("Payload of cmd2: {:?}", payload);
    cmd3.print_payload();
}
