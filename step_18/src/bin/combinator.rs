use std::fmt::Display;

struct Student {
    name: String,
    gpa: f32,
}

impl Display for Student {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} (GPA: {})", self.name, self.gpa)
    }
}

fn main() {
    let students = vec![
        "Alice 3.9",
        "Bob 2.8",
        "Charlie 3.5",
        "David 3.7",
        "Eve 4.0",
    ];

    // Combinators are used to chain operations on types like iterators, Option, Result, etc.
    let good_students: Vec<Student> = students.iter()
    // The map function is a combinator that applies a function to each element of an iterator.
    .map(|student| {
        let mut student = student.split_whitespace();
        let name = student.next()?.to_string();
        let gpa = student.next()?.parse().ok()?;
        Some(Student { name, gpa })
    })
    // The flatten function is a combinator that flattens an iterator of iterators.
    // In this case, it removes the Option type from the iterator. Option<T> -> T
    .flatten()
    // The filter function is a combinator that filters an iterator based on a predicate.
    // If false, the element is removed from the iterator.
    .filter(|student| student.gpa > 3.5)
    // The collect function is a combinator that collects the elements of an iterator into a collection.
    .collect();

    println!("Good students:");
    for student in good_students {
        println!("{}", student);
    }
}
