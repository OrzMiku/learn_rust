use step_21::*;
use step_21_1::Database;

make_answer!();
make_answer_2!();

fn main() {
    println!("The answer is {}", answer() + answer_2());

    let mut db = Database::new("localhost".to_string());
    for _ in 0..30 {
        db.connect();
    }

    let result = add(1, 2);
    println!("Result: {}", result);
}

#[show_attribute_stream(arg1, arg2)]
fn add(x: i32, y: i32) -> i32 {
    x + y
}