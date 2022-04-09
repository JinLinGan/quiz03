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

trait Area {
    fn area(&self) -> f32;
}

impl Area for Rectangle {
    fn area(&self) -> f32 {
        self.width * self.high
    }
}

impl Area for Circle {
    fn area(&self) -> f32 {
        PI * self.radius * self.radius
    }
}

impl Area for Triangle {
    fn area(&self) -> f32 {
        self.base * self.high / 2.0
    }
}

fn main() {
    let r = Rectangle {
        width: 2.0,
        high: 4.0,
    };
    println!("Area of {:?} is {}", r, r.area());

    let c = Circle { radius: 2.0 };
    println!("Area of {:?} is {}", c, c.area());

    let t = Triangle {
        base: 2.0,
        high: 2.0,
    };
    println!("Area of {:?} is {}", t, t.area());
}
