pub trait Area {
    fn area(&self) -> f32;
}

#[derive(Debug)]
pub struct Circle {
    pub radius: f32,
}

impl Area for Circle {
    fn area(&self) -> f32 {
        self.radius * self.radius * std::f32::consts::PI
    }
}

#[derive(Debug)]
pub struct Square {
    length: f32,
}

impl Area for Square {
    fn area(&self) -> f32 {
        self.length * self.length
    }
}

// static dispatch
// create function copy at comple time, faster in runtime, larger exe size
fn generic_call_area<T>(obj: &T) -> f32
where
    T: Area,
{
    obj.area()
}

// static dispatch
// this is a syntactic sugar for generic_call_area
fn impl_call_area(obj: &impl Area) -> f32
{
    obj.area()
}

// dynamic dispath
// fat pointer: points to data and virtual function table
// slower at runtime, smaller exe
fn dyn_call_area(obj: &dyn Area) -> f32
{
    obj.area()
}

fn main() {
    let circle = Circle { radius: 4.0 };
    let square = Square { length: 2.0 };

    println!("Generic call area: {}", generic_call_area(&circle));
    println!("Generic call area: {}", generic_call_area(&square));

    println!("Impl call area: {}", impl_call_area(&circle));
    println!("Impl call area: {}", impl_call_area(&square));
    
    println!("Dyn call area: {}", dyn_call_area(&circle));
    println!("Dyn call area: {}", dyn_call_area(&square));

    println!("Vector of shapes");
    let shapes: Vec<&dyn Area> = vec![&circle, &square];
    for shape in shapes {
        println!("Dynamic vector area: {}", shape.area());
    }
}