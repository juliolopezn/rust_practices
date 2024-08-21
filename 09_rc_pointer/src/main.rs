use std::rc::Rc;

#[derive(Debug)]
struct Car {
    name: String,
}

impl Drop for Car {
    fn drop(&mut self) {
        println!("Dropping Car {}", self.name);
    }
}

fn box_example() {
    println!("Box Example");
    let car1 = Box::new(Car {
        name: String::from("BMW"),
    });

    let car2 = car1;
    // println!("{car1:?}"); // Error because car1 is moved to car2
    println!("{car2:?}");
}

fn rc_example() {
    println!("Rc Example");
    let car1 = Rc::new(Car {
        name: String::from("BMW"),
    });

    {
        let car2 = car1.clone();
        println!("{car1:?}");
        println!("{car2:?}");
    }

    println!("{car1:?}");
}

fn main() {
    box_example();
    rc_example();
}
