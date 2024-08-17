fn main() {
    prompt_user_name_and_age();
    number_guessing();
}

#[test]
fn basic_sum() {
    let x: i8 = 5;
    let y: i8 = 10;

    println!("The sum of {} and {} is {}", x, y, sum(x, y));

    fn sum(x: i8, y: i8) -> i8 {
        x + y
    }
}

fn prompt_user_name_and_age() {
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

    let age: u8 = age.trim().parse().expect("Failed to parse age");

    println!("Hello, {}. You are {} years old", name.trim(), age);

    if age >= 18 {
        println!("You can buy beer, yeah!");
    } else {
        println!("Sorry, you are too young to buy beer, kiddo!");
    }
}

fn number_guessing() {
    let random_number = generate_random_number(1, 10);
    println!("Guess the number between 1 and 10");

    loop {
        let mut guess: String = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read guess");

        let guess: u32 = guess.trim().parse().expect("Failed to parse guess");

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

#[test]
fn shuffle_words() {
    let mut word1 = "Hello".chars();
    let mut word2 = ",World!".chars();
    let mut result = String::new();

    loop {
        let char1 = word1.next();
        let char2 = word2.next();

        if char1.is_none() && char2.is_none() {
            break;
        }

        if char1.is_some() {
            result.push(char1.unwrap());
        }
        if char2.is_some() {
            result.push(char2.unwrap());
        }
    }

    assert_eq!("H,eWlolrold!", result);
}

#[test]
fn user_data() {
    enum LoginData {
        None,
        Invalid,
        NotRegistered,
        Username(String)
    }

    let none_user = LoginData::None;
    let invalid_user = LoginData::Invalid;
    let not_registered_user = LoginData::NotRegistered;
    let current_user = LoginData::Username("John Doe".to_string());

    print_user_data(none_user);
    print_user_data(invalid_user);
    print_user_data(not_registered_user);
    print_user_data(current_user);

    fn print_user_data(data: LoginData) {
        print!("User data: ");
        match data {
            LoginData::None => println!("No user data"),
            LoginData::Invalid => println!("Invalid user data"),
            LoginData::NotRegistered => println!("User not registered"),
            LoginData::Username(name) => println!("{name}")
        }
    }
}
