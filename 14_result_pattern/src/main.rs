use std::{fs::File, io::{Error, ErrorKind, Read}};

fn main() {
    let path = "./file.txt";

    let file_content = match read_file_content(path) {
        Ok(content) => content,
        Err(error) => panic!("Problem reading the file: {:?}", error),
    };

    println!("File content:");
    println!("{}", file_content);
}

fn read_file_content(path: &str) -> Result<String, Error> {
    let file_result = File::open(path);

    let mut file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("File not found '{path}'"),
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    let mut string_buffer = String::new();
    let file_content_result = match file.read_to_string(&mut string_buffer) {
        Ok(_) => Ok(string_buffer),
        Err(error) => Err(error),
    };

    file_content_result
}

fn _read_file_content_with_question_mark(path: &str) -> Result<String, Error> {
    let mut string_buffer = String::new();
    
    File::open(path)?.read_to_string(&mut string_buffer)?;

    Ok(string_buffer)
}