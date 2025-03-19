// In rust, we do not have inheritance like in other languages
// Instead, we use traits to define shared behavior
// Trait is a collection of methods that can be implemented by different types
// Trait is similar to an interface in other languages
// Vehicle: Paint means that Vehicle trait is a super trait of Paint trait
// This means that any type that implements Vehicle trait must also implement Paint trait
trait Vehicle: Paint{
    // Abstract method
    // This method must be implemented by any type that implements the Vehicle trait
    fn unload(&self);
}

trait Paint {
    // With default implementation, also can be overridden
    fn paint(&self, color: &str) {
        println!("Painting with color: {}", color);
    }
}

// Abstracting the vehicle information to avoid code duplication
struct VehicleInfo {
    make: String,
    model: String,
    year: u32,
}

struct Car {
    info: VehicleInfo,
}

impl Vehicle for Car {
    fn unload(&self) {
        println!("Unloading car: {} {} {}", self.info.make, self.info.model, self.info.year);
    }
}

// If comment out the below line, you will get an error
// Because Vehicle trait is a super trait of Paint trait, Car implements Vehicle trait, but not Paint trait
impl Paint for Car {}

struct Truck {
    info: VehicleInfo,
}

impl Vehicle for Truck {
    fn unload(&self) {
        println!("Unloading car: {} {} {}", self.info.make, self.info.model, self.info.year);
    }
}

impl Paint for Truck {}

struct House {}

impl Paint for House {}

fn main() {
    let car = Car {
        info: VehicleInfo {
            make: String::from("Toyota"),
            model: String::from("Corolla"),
            year: 2020,
        },
    };

    let truck = Truck {
        info: VehicleInfo {
            make: String::from("Ford"),
            model: String::from("F-150"),
            year: 2021,
        },
    };

    let house = House {};

    car.unload();
    // car.paint("Red");
    paint_red(&car);
    paint_vehicle_cyan(&car);

    truck.unload();
    // truck.paint("Blue");
    paint_blue(&truck);
    paint_vehicle_purple(&truck);
    paint_vehicle_yellow(&truck);

    // house.paint("Green");
    paint_green(&house);
    // _paint_vehicle_pink(&house); // This will not work because House does not implement Vehicle trait

    let paintable_object = create_paintable_object_1(); // This will return a House
    paintable_object.paint("Black");

    let paintable_object_2 = create_paintable_object_2(true); // This will return a Car, but we don't know the type at compile time
    let paintable_object_3 = create_paintable_object_2(false); // This will return a House, but we don't know the type at compile time

    // below code will not work because we don't know the type at compile time, but paint_red() needs a type implementing Paint trait
    // paint_red(&paintable_object_2); 
    // paint_red(&paintable_object_3);

    // We use * to dereference the Box<dyn Paint> to get the actual type `dyn Paint`
    // Then use & to borrow the value
    // paint_color(&*paintable_object_2, "White");
    // paint_color(&*paintable_object_3, "Black");
    // We can use as_ref() of Box to get a reference to the value inside the Box to instead of dereferencing it
    paint_color(paintable_object_2.as_ref(), "Black");
    paint_color(paintable_object_3.as_ref(), "White");
}

// This function takes a reference to a trait object
// This means that the type is not known at compile time
fn paint_color(obj: &dyn Paint, color: &str) {
    obj.paint(color);
}

// Trait bounds by using T: <TraitName>
fn paint_red<T: Paint>(obj: &T) {
    obj.paint("Red");
}

// Trait bounds by using impl syntax, it's a syntax sugar for the above
fn paint_blue(obj: &impl Paint) {
    obj.paint("Blue");
}

// Trait bounds by using where clause
fn paint_green<T>(obj: &T) where T: Paint {
    obj.paint("Green");
}

// Use + to combine multiple traits
fn _paint_vehicle_pink<T: Vehicle + Paint>(obj: &T) {
    println!("Vehicle Painting");
    obj.paint("Pink");
}

// Use + to combine multiple traits with impl syntax
fn paint_vehicle_purple(obj: &(impl Vehicle + Paint)) {
    println!("Vehicle Painting");
    obj.paint("Purple");
}

// Use + to combine multiple traits with where clause
fn paint_vehicle_cyan<T>(obj: &T) where T: Vehicle + Paint {
    println!("Vehicle Painting");
    obj.paint("Cyan");
}

// Because Vehicle trait is a super trait of Paint trait, we can only use Vehicle trait as a bound
fn paint_vehicle_yellow<T>(obj: &T) where T: Vehicle {
    println!("Vehicle Painting");
    obj.paint("Yellow");
}

// This function returns a type that implements the Paint trait
fn create_paintable_object_1() -> impl Paint {
    House {}
}

// We return Car in if statement and House in else statement
// This is not allowed because the return type must be the same in all branches
// The compiler cannot infer the return type
// The below code will get an error
// fn create_paintable_object_2(isVehicle: bool) -> impl Paint {
//     if isVehicle {
//         Car {
//             info: VehicleInfo {
//                 make: String::from("Toyota"),
//                 model: String::from("Corolla"),
//                 year: 2020,
//             },
//         }
//     } else {
//         House {}
//     }
// }

// We can use Box<dyn Trait> to return different types
// This is called dynamic dispatch
// dyn keyword is used to indicate that the type is a trait object
// This means that the type is not known at compile time
fn create_paintable_object_2(is_vehicle: bool) -> Box<dyn Paint> {
    if is_vehicle {
        Box::new(Car {
            info: VehicleInfo {
                make: String::from("Toyota"),
                model: String::from("Corolla"),
                year: 2020,
            },
        })
    } else {
        Box::new(House {})
    }
}