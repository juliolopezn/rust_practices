fn main() {
    let s = "Hello, world!";

    println!("First word: {}", first_word(&s));

    println!("Slice 1: {}", &s[0..5]);
    println!("Slice 2: {}", &s[7..12]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (idx, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..idx];
        }
    }

    &s
}
