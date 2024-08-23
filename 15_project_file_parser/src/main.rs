use std::{fs::{self, File}, io::{Error, Read}, process};

use colored::Colorize;
use regex::Regex;

#[derive(Debug)]
struct Arguments<'a> {
    input_file: &'a String,
    output_file: &'a String,
    pattern: &'a String,
    replace: &'a String,
}

fn parse_args(argc: usize, argv: &Vec<String>) -> Arguments {
    if argc != 5 {
        eprintln!("{} wrong number of arguments!", "ERROR:".red());
        process::exit(1);
    }

    Arguments {
        pattern: &argv[1],
        replace: &argv[2],
        input_file: &argv[3],
        output_file: &argv[4],
    }
}

fn read(path: &str) -> Result<String, Error> {
    let mut string_buffer = String::new();
    
    File::open(path)?.read_to_string(&mut string_buffer)?;

    Ok(string_buffer)
}

fn replace(content: &str, pattern: &str, replace: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(pattern)?;
    let replaced_content = regex.replace_all(content, replace);
    Ok(replaced_content.to_string())
}

fn write(path: &str, content: &str) -> Result<(), Error> {
    fs::write(path, content)?;

    Ok(())
}

fn run(argc: usize, argv: Vec<String>) {
    let args = parse_args(argc, &argv);
    println!("{:?}", args);

    let file_content = match read(args.input_file) {
        Ok(content) => content,
        Err(_) => {
            eprintln!("{} problem reading the file '{}'", "ERROR:".red(), &args.input_file);
            process::exit(1);
        }
    };

    let replaced_content = match replace(&file_content, &args.pattern, &args.replace) {
        Ok(content) => content,
        Err(_) => {
            eprintln!("{} problem replacing the pattern", "ERROR:".red());
            process::exit(1);
        }
    };

    match write(&args.output_file, &replaced_content) {
        Ok(_) => println!("{} file '{}' written successfully", "INFO:".green(), &args.output_file),
        Err(_) => {
            eprintln!("{} problem writing the file '{}'", "ERROR:".red(), &args.output_file);
            process::exit(1);
        }
    };
}

fn main() {
    // 1. File input
    // 2. File output
    // 3. Pattern
    // 4. Replace
    let argv = std::env::args().collect::<Vec<String>>();
    let argc = argv.len();
    run(argc, argv)
}

