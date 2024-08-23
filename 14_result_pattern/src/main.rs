use std::{fs::File, io::{ErrorKind, Read}};

fn main() {
    let path = "./file.txt";
    let file_result = File::open(path);

    let mut file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("File not found '{path}'"),
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    let mut file_content = String::new();
    match file.read_to_string(&mut file_content) {
        Ok(_) => {},
        Err(error) => panic!("Problem reading the file: {:?}", error),
    }

    println!("File content:");
    println!("{}", file_content);
}
