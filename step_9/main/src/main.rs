use draw::draw_line;
use draw::color;
use draw::color::RGB;
use draw::shapes::Rectangle;

fn main() {
    draw_line(1, 2);

    color::draw_line(3, 4, &RGB { r: 255, g: 0, b: 0 });

    let rect = Rectangle {
        color: RGB { r: 0, g: 255, b: 0 },
        width: 10,
        height: 20,
    };
    println!("Rectangle: {:?}", rect);
}
