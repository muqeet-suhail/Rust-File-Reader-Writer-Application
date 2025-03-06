use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::{BufRead, BufReader};
use std::io::{BufWriter, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1]; // The first argument is the path that was used to call the program and second is the file name (cargo run -- example_file.txt).
    let file = OpenOptions::new().read(true).write(true).open(path.trim()); // The last argument is the file name (example.txt).

    println!("Choose an option:");
    println!("1. Write to file");
    println!("2. Read from file");
    println!("Enter Choice: ");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    match file {
        Ok(file) => {
            if choice.trim() == "1" {
                let mut input = String::new();
                println!("Enter text to write to file: ");
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                input = input.trim().to_string();
                write_to_file(&input, &file);
            } else if choice.trim() == "2" {
                println!("Reading from file:");
                read_file(&file);
            } else {
                println!("Invalid choice");
            }
        }
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                panic!("File not found: {}", error)
            }
            _ => {
                panic!("Error opening file: {}", error)
            }
        },
    };
}

fn read_file(file: &File) {
    let reader = BufReader::new(file);
    let mut i = 0;
    for line in reader.lines() {
        match line {
            Ok(line) => {
                if line.is_empty() && i==0{
                    println!("File is empty")
                }
                i+=1;
                println!("{}", line)
            }
            Err(error) => match error.kind() {
                _ => {
                    panic!("Error reading line: {}", error)
                }
            },
        };
    }
}

fn write_to_file(message: &String, file: &File) {
    let mut writer = BufWriter::new(file);
    let _ = writer.write_all(&message.as_bytes());
}
