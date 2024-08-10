
fn main() {
    exercise_04();
}

fn _exercise_01() {
    println!("Hello, world!");
}

fn _exercise_02() {
    let x: i8 = 5;
    let y: i8 = 10;
    
    println!("The sum of {} and {} is {}", x, y, sum(x, y));

    fn sum(x: i8, y: i8) -> i8 {
        x + y
    }
}

fn _exercise_03() {
    println!("Input your name: ");
    let mut name: String = String::new();
    std::io::stdin()
        .read_line(&mut name)
        .expect("Failed to read name");

    println!("Input your age: ");
    let mut age: String = String::new();
    std::io::stdin()
        .read_line(&mut age)
        .expect("Failed to read age");

    let age: u8 = age.trim()
        .parse()
        .expect("Failed to parse age");

    println!("Hello, {}. You are {} years old", name.trim(), age);

    if age >= 18 {
        println!("You can buy beer, yeah!");
    } else {
        println!("Sorry, you are too young to buy beer, kiddo!");
    }
}

fn exercise_04() {
    let random_number = generate_random_number(1, 10);
    println!("Guess the number between 1 and 10");
    
    loop {
        let mut guess: String = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read guess");

        let guess: u32 = guess.trim()
            .parse()
            .expect("Failed to parse guess");

        if guess != random_number {
            println!("Wrong guess, try again!");
        } else {
            println!("Yeah, you got it!");
            break;
        }
    }

    fn generate_random_number(min: u32, max: u32) -> u32 {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Failed to get current time");
    
        let random_seed = current_time.as_secs() as u32;
        let random_number = (random_seed % (max - min + 1)) + min;
    
        random_number
    }
}
