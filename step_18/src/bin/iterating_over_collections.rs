use std::collections::HashMap;

fn main() {
    // Iterating over collections
    // HashMap is a collection type that stores key-value pairs
    // Other collections like Vec, HashSet, BTreeMap, BTreeSet, LinkedList, etc. also have iterators
    let mut scoreboard = HashMap::new();
    scoreboard.insert("Blue", 10);
    scoreboard.insert("Yellow", 50);
    scoreboard.insert("Red", 30);

    // iter() returns an iterator that borrows the collection
    let mut iter_1 = scoreboard.iter();
    println!("{:?}", iter_1.next());
    println!("{:?}", iter_1.next());
    println!("{:?}", iter_1.next());
    println!("{:?}", iter_1.next());

    // iter_mut() returns an iterator that mutably borrows the collection
    let iter_2 = scoreboard.iter_mut();
    for (key, value) in iter_2 {
        *value += 10;
        println!("{}: {}", key, value);
    }

    // into_iter() returns an iterator that will consume the collection
    let iter_3 = scoreboard.into_iter();
    for (key, value) in iter_3 {
        println!("{}: {}", key, value);
    }
}