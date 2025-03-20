use step_13::Point;

// Orphan rule, the implementation of a trait must be in the same crate as the trait or the type
// This will get an error, because Point struct is defined in the library crate
// impl PartialEq for Point {
//     fn eq(&self, other: &Self) -> bool {
//         self.x == other.x && self.y == other.y
//     }

//     fn ne(&self, other: &Self) -> bool {
//         self.x != other.x || self.y != other.y
//     }
// }

// We can use a wrapper struct to implement the trait
struct PointWrapper(Point);

impl PartialEq for PointWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0.x == other.0.x && self.0.y == other.0.y
    }

    fn ne(&self, other: &Self) -> bool {
        self.0.x != other.0.x || self.0.y != other.0.y
    }
}

fn main() {
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = Point { x: 1.0, y: 2.0 };

    let pw1 = PointWrapper(p1);
    let pw2 = PointWrapper(p2);

    println!("p1 == p2: {}", pw1 == pw2);
    println!("p1 != p2: {}", pw1 != pw2);
}