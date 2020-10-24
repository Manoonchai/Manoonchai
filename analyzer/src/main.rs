mod lib;

use std::fs;
use std::path::Path;
use lib::*;

fn main() {
    println!("Analyzer :");

    process("data/thai1k.txt", "data/thai1k_out.txt");
    process("data/thai5k.txt", "data/thai5k_out.txt");
}

fn process(input_path: &str, output_path: &str) {
    let filename = Path::new(input_path);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let result = analyze_string_sorted(contents.as_str());

    let mut out: String = String::new();

    for (ch, count) in &result {
        out.push_str(format!(" {} : {}", ch, count).as_str());
        out.push_str("\n");
    }

    fs::write(Path::new(output_path), out).unwrap();

    println!("{:?}", result);
}
