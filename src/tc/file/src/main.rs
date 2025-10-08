use std::env;
mod magic;
mod detector;
use crate::detector::matcher::detect_file_type;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: file <filename>");
        std::process::exit(1);
    }
    let filename = &args[1];
    match detect_file_type(filename) {
        Ok(ftype) => println!("{}", ftype),
        Err(e) => eprintln!("Error: {}", e),
    }
}