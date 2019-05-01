use std::io;
use std::io::Read;
use std::fs::File;

fn main() {
    println!("{}", awesome_function());
}

fn awesome_function() -> String {

    println!("Input a file name.");
    // User input
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // File access
    let mut my_file = File::open(input).unwrap();

    // String parsing
    let mut my_result = String::new();
    my_file.read_to_string(&mut my_result).unwrap();

    my_result
}
