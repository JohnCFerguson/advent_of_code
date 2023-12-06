fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = read_lines(file_path);
    println!("contents: {:?}", contents);
}
