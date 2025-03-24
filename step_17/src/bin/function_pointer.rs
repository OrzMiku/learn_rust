fn main() {
    // Closures
    let _min = 10;
    // let greater_than = |x: &i32| -> bool { *x > _min }; // Error: cannot capture `_min` in an `Fn` closure
    let greater_than = |x: &i32| -> bool { *x > 10 };
    // let less_than = |x: &i32| -> bool { *x < 20 };
    let x = 16;
    println!("Are both true: {:?}", are_both_true(greater_than, less_than, &x));
    // println!("Are both true: {:?}", are_both_true(greater_than, less_than, &x));
}

fn less_than(x: &i32) -> bool {
    *x < 20
}

// Closure as a parameter
// fn are_both_true<T, U, V>(f1: T, f2: U, item: &V) -> bool 
//     where T: Fn(&V) -> bool, U: Fn(&V) -> bool
// {
//     f1(item) && f2(item)
// }

// We can also use function (pointers) instead of closures
// Notice that the function pointers cannot capture variables from the environment
fn are_both_true<V> (f1: fn(&V) -> bool, f2: fn(&V) -> bool, item: &V) -> bool {
    f1(item) && f2(item)
}