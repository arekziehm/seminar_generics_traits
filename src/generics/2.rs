// https://doc.rust-lang.org/book/ch10-01-syntax.html
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    //let mixed = Point {x: 1, y: 4.0};

    println!("int Point x: {}", integer.x());
    println!("float Point x: {}", float.x());
    println!("float Point dist: {}", float.distance_from_origin());
    //println!("int Point dist: {}", integer.distance_from_origin());
    // float.cmp_display();
}


//Traits
use std::fmt::Display;


impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Point<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}