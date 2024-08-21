fn main() {
    let mut r: &str = "Hi";
    
    {
        let x = String::from("Hello");
        // r = &x; // Error: `x` does not live long enough
    }
    println!("r: {r}");
}
