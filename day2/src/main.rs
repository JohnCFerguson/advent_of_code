use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("In file {}", file_path);

    read_lines(file_path);
}

fn read_lines(filename: &str) {
    for line in fs::read_to_string(filename).unwrap().lines() {
        println!("{}", line);
    }
}
