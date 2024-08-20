fn sum(a: i32, b: i32, callback: fn(i32)) -> i32 {
    let r = a + b;
    
    callback(r);

    r
}

fn show_result(x: i32) {
    println!("The function result: {}", x);
}

fn main() {
    let b = 5;

    let r = sum(10, b, show_result);
    println!("The result is: {}", r);

    let closure = |x: i32| println!("The closure result is: {}", x);
    let r = sum(10, b, closure);
    println!("The result is: {}", r);
}
