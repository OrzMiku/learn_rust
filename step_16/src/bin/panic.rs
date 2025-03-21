fn main() {
    // panic is an error that can not be recovered
    // panic!("Something went wrong");
    let v = vec!["one", "two", "three"];
    println!("{}", v[99]); // panic: index out of bounds, we can set RUST_BACKTRACE=1 to see the backtrace
}
