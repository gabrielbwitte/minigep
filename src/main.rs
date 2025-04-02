use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let query: &String = &args[1];
    let file_path: &String = &args[2];

    println!("Searching for {query}");
    print!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should hava been able to read the file");

    println!("With text:\n{contents}");
}
