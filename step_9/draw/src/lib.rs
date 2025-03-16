/*
    Author: OrzMiku
    Feature flags are used to compile different parts of the code based on the features enabled.
    It can reduce the binary size by excluding unnecessary code.
    The feature flags are defined in the Cargo.toml file. You can check the Cargo.toml file in the root directory.
*/

pub fn draw_line(x: i32, y: i32) {
    // draw line without color
    println!("Draw line at ({}, {})", x, y);
}

// #[cfg(feature = "something")] is used to compile the code when the feature "something" is enabled
// Color module is only compiled when the feature "color" is enabled
#[cfg(feature = "color")]
pub mod color {
    // Export RGB struct from the module, that can be used by the parent module
    pub use rgb::RGB;

    pub fn draw_line(x: i32, y: i32, color: &RGB<u16>) {
        println!("Draw line at ({}, {}) with color {:?}", x, y, color);
        // draw line with color
    }
}

// Shapes module is only compiled when the feature "shapes" is enabled
#[cfg(feature = "shapes")]
pub mod shapes {
    use serde::{Serialize, Deserialize};
    use rgb::RGB;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Rectangle {
        pub color: RGB<u16>,
        pub width: u32,
        pub height: u32,
    }    
}