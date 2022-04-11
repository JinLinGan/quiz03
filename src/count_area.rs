use core::fmt::Debug;
use std::f32::consts::PI;

#[derive(Debug)]
struct Rectangle {
    width: f32,
    high: f32,
}
#[derive(Debug)]
struct Circle {
    radius: f32,
}
#[derive(Debug)]
struct Triangle {
    base: f32,
    high: f32,
}

trait Shape {
    fn area(&self) -> f32;
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        self.width * self.high
    }
}

impl Shape for Circle {
    fn area(&self) -> f32 {
        PI * self.radius * self.radius
    }
}

impl Shape for Triangle {
    fn area(&self) -> f32 {
        self.base * self.high / 2.0
    }
}

fn print_area<T>(shape: &T)
where
    T: Shape + Debug,
{
    println!("Area of {:?} is {}", shape, shape.area());
}

fn main() {
    print_area(&Rectangle {
        width: 2.0,
        high: 4.0,
    });

    print_area(&Circle { radius: 2.0 });

    print_area(&Triangle {
        base: 2.0,
        high: 2.0,
    });
}
