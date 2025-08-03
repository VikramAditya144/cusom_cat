use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Exit if the filename wasn't passed
    if args.len() < 2 {
        eprintln!("Usage: rust_cat <filename>");
        std::process::exit(1);
    }

    // Take the filename from args[1]
    let filename = &args[1];

    // Try to open the file
    let file = File::open(filename).unwrap_or_else(|error| {
        eprintln!("Failed to open {}: {}", filename, error);
        std::process::exit(1);
    });

    // Create a buffered reader to read the file line-by-line
    let reader = BufReader::new(file);

    // Loop through each line in the file
    for line in reader.lines() {
        match line {
            Ok(text) => println!("{}", text),
            Err(error) => eprintln!("Error reading a line: {}", error),
        }
    }
}
