use std::io;
use std::fs;

fn main() {
    println!("Hello, world!");
}

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
