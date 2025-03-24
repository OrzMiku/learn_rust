struct Person {
    firstname: String,
    lastname: String,
    occupation: String,
}

// PersonIterator is unnecessary, because we can use std::vec::IntoIter directly
// It's just for showing how to implement Iterator trait
// struct PersonIterator {
//     values: Vec<String>
// }

// impl Iterator for PersonIterator {
//     type Item = String;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.values.is_empty() {
//             None
//         } else {
//             Some(self.values.remove(0))
//         }
//     }
// }

// impl IntoIterator for Person {
//     type Item = String;
//     type IntoIter = PersonIterator;

//     fn into_iter(self) -> Self::IntoIter {
//         PersonIterator {
//             values: vec![self.firstname, self.lastname, self.occupation]
//         }
//     }
// }

impl IntoIterator for Person {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.firstname, self.lastname, self.occupation].into_iter()
    }
}

fn main() {
    // Manually iterate
    let person = Person {
        firstname: "Orz".to_string(),
        lastname: "Miku".to_string(),
        occupation: "Rust Developer".to_string(),
    };

    let mut i = person.into_iter();
    println!("{:?}", i.next());
    println!("{:?}", i.next());
    println!("{:?}", i.next());
    println!("{:?}", i.next());

    // Using for loop
    let person = Person {
        firstname: "Orz".to_string(),
        lastname: "Miku".to_string(),
        occupation: "Rust Developer".to_string(),
    };

    // for value in person.into_iter() { // into_iter() is unnecessary, because it's automatically called
    for value in person {
        println!("{}", value);
    }
}
