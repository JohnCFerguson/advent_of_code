use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = read_lines(file_path);
    println!("contents: {:?}", contents);
    let ints: Vec<u32> = contents.iter().map(|s| s.parse::<u32>().unwrap()).collect();
    println!("ints {:?}", ints.iter().sum::<u32>());
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in fs::read_to_string(filename).unwrap().lines() {
        let mut new_line: String = line.to_string();
        let digits = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        let digits_map = HashMap::from([
            ("one", "o1e"),
            ("two", "t2o"),
            ("three", "t3t"),
            ("four", "f4r"),
            ("five", "f5e"),
            ("six", "s6x"),
            ("seven", "s7n"),
            ("eight", "e8t"),
            ("nine", "n9e"),
        ]);
        for digit in digits {
            if new_line.contains(digit) {
                let dig = digits_map.get(digit).unwrap();
                let mut dig_to_remove = digit;
                println!("dig_to_remove: {}", dig_to_remove);
                new_line = new_line.replace(dig_to_remove, &dig);
            }
            println!("new_line: {}", new_line);
        }
        let ints = get_digits_from_chars(new_line);
        result.push(ints.first().unwrap().to_string() + &ints.last().unwrap().to_string());
    }
    return result;
}

fn get_digits_from_chars(arr: String) -> Vec<u32> {
    let mut ints = Vec::new();
    let int_vec = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    for char in arr.to_string().chars() {
        let int = char.to_digit(10);
        match int {
            Some(int) => {
                if int_vec.contains(&int) {
                    ints.push(int);
                }
            }
            None => {}
        }
    }
    return ints;
}
