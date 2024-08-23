fn main() {
    let mut _r: &str = "Hi";
    
    {
        let _x = String::from("Hello");
        // r = &x; // Error: `x` does not live long enough
    }
    println!("r: {_r}");
}
