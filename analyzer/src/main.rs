mod lib;

use std::fs;
use std::path::Path;
use lib::*;

fn main() {
    println!("Analyzer :");

    process("data/thai1k.txt", "data/thai1k_out.txt");
    process("data/thai5k.txt", "data/thai5k_out.txt");

    process_triads("data/thai1k.txt", "data/thai1k_triads_out.txt");
    process_triads("data/thai5k.txt", "data/thai5k_triads_out.txt");

    // https://github.com/wongnai/wongnai-corpus
    // process_triads("/Users/narze/Downloads/w_review_train.csv", "data/wongnai_triads_out.txt"); // ~3.30m

    // https://github.com/PyThaiNLP/wisesight-sentiment
    // cat neg.txt neu.txt pos.txt q.txt > all.txt
    // process_triads("/Users/narze/Code/github.com/PyThaiNLP/wisesight-sentiment/all.txt", "data/wisesight_sentiment_out.txt") // 28s
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

fn process_triads(input_path: &str, output_path: &str) {
    let filename = Path::new(input_path);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let result = analyze_string_triads_sorted(contents.as_str());

    let mut out: String = String::new();
    let mut total_count = 0u32;

    for (ch, count) in &result {
        total_count += count;

        if *count == 1 { continue; }

        let entry = format!(" {} : {}", ch, count);

        println!("{:?}", entry);

        out.push_str(entry.as_str());
        out.push_str("\n");
    }

    out.insert_str(0, format!("Total : {}\n", total_count).as_str());

    fs::write(Path::new(output_path), out).unwrap();
}
