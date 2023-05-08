// To create a function for a struct, need to make it first as a "TRAIT" (like an interface in C#)
trait Area {
    fn area(&self) -> f64;
}

struct Point<T> {
    x: T,
    y: T,
}


// First, we make the structs with only their variables
struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

// Then we add the functions to the structs, but define the functions and give them their actual code in the "impl" block
impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}


// The main function at the bottom here uses all of this information
fn main() {
    // Can use the Point<T> struct with any type, but has to be 2 of the same type. Cannot cross types with only T.
    // let boolean = Point {x: true, y: false };
    // let integer = Point {x: 1, y: 2 };
    // let float = Point {x: 1.0, y: 2.0 };
    // let string_slice = Point {x: "Hello", y: "World" };
    let circle = Circle {radius: 5.0};
    let rectangle = Rectangle {width: 5.0, height: 10.0};
    println!("Circle area: {}", circle.area());
    println!("Rectangle area: {}", rectangle.area());
}

pub fn example() {
    main();
}