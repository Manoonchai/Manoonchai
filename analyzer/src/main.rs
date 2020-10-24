mod lib;

use std::fs;
use std::path::Path;
use lib::*;

fn main() {
    println!("Hello, world!");

    let filename = Path::new("data/thai1k.txt");
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let result = analyze_string_sorted(contents.as_str());

    println!("{:?}", result);
}
