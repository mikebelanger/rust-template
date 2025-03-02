use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind};

fn main() {
    // First, get the filename argument
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            let filename = &args[1];

            let file = File::open(filename);
            let file = match file {
                Ok(file) => file,
                Err(e) => match e.kind() {
                    ErrorKind::NotFound => panic!("File not found"),
                    _ => panic!("Error opening file: {}", e),
                },
            };

            let reader = BufReader::new(file);
            for line in reader.lines() {
                match line {
                    Ok(line) => println!("{}", line),
                    Err(e) => panic!("error reading line: {}", e),
                }
            }
        }
        _ => panic!("Usage: cargo run <filename>"),
    }
}
