#[derive(Debug)]
struct Circle {
    radius: f64,
    x: f64,
    y: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn circunference(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }

    fn diameter(&self) -> f64 {
        2.0 * self.radius
    }

    fn new(radius: f64, x: f64, y: f64) -> Circle {
        Circle { radius, x, y }
    }

    fn smaller_than(&self, other: &Self) -> bool {
        self.radius < other.radius
    }
}

fn main() {
    let circle1 = Circle::new(5.0, 1.0, 2.0);
    let circle2 = Circle::new(7.0, 3.0, 4.0);

    println!("----------------------------------------\n");
    println!("Circle 1 area: {}", circle1.area());
    println!("Circle 1 circunference: {}", circle1.circunference());
    println!("Circle 1 diameter: {}", circle1.diameter());
    println!("\n----------------------------------------\n");
    println!("Circle 2 area: {}", circle2.area());
    println!("Circle 2 circunference: {}", circle2.circunference());
    println!("Circle 2 diameter: {}", circle2.diameter());
    println!("\n----------------------------------------\n");
    println!("Circle 1 is smaller than Circle 2: {}", circle1.smaller_than(&circle2));
    println!("\n----------------------------------------");

    let circle3 = Circle { radius: 10.0, ..circle2 };

    println!("Circle 1: X({}), Y({}), Radius({})", circle1.x, circle1.y, circle1.radius);
    println!("Circle 2: X({}), Y({}), Radius({})", circle2.x, circle2.y, circle2.radius);
    println!("Circle 3: X({}), Y({}), Radius({})", circle3.x, circle3.y, circle3.radius);
}