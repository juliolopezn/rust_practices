fn main() {
    let name = String::from("John Doe");

    print_name(&name);
    print_name_len(&name);

    ownership();
}

fn print_name(name: &String) {
    println!("The name is: {}", name);
}

fn print_name_len(name: &String) {
    println!("The name length is: {}", name.len());
}

fn ownership() {
    let mut s = String::from("John Connor");
    
    // Inmutable reference
    {
        let _r1 = &s;
        let _r2 = &s;
        let _r3 = &s;
        // let _w1 = &mut s; // Error: cannot borrow `s` as mutable because it is also borrowed as immutable

        println!("Inmutable reference {_r1}");
        println!("Inmutable reference {_r2}");
        println!("Inmutable reference {_r3}");
    }

    // Mutable reference
    {
        let _w1 = &mut s;
        // let _w2 = &mut s; // Error: cannot borrow `s` as mutable more than once at a time
        // let _r1 = &s; // Error: cannot borrow `s` as mutable because it is also borrowed as immutable

        println!("Mutable reference {_w1}");
    }

    // Inmutable reference after mutable reference
    {
        let _w1 = &mut s;

        _w1.push_str(" - The Terminator");

        let _r1 = &_w1;
        let _r2 = &_w1;
        let _r3 = &_w1;

        // _w1.push_str(" - The Terminator"); // Error: cannot borrow `_w1` as mutable because it is also borrowed as immutable

        println!("Mutable reference {_w1}");
        println!("Inmutable reference {_r1}");
        println!("Inmutable reference {_r2}");
        println!("Inmutable reference {_r3}");
    }
}