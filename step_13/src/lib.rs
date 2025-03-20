pub struct Point {
    pub x: f32,
    pub y: f32
}

// Orphan rule, the implementation of a trait must be in the same crate as the trait or the type
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn ne(&self, other: &Self) -> bool {
        self.x != other.x || self.y != other.y
    }
}