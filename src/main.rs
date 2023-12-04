use clap::Parser;
use std::collections::HashMap;
use std::fs;

use crate::puzzles::*;

pub mod puzzles;

#[derive(Parser)]
struct CliArgs {
    puzzle_id: String,
    input_path: std::path::PathBuf,
}

fn main() {
    let args = CliArgs::parse();

    let mut puzzles: HashMap<String, fn(String)> = HashMap::new();
    puzzles.insert("1-1".to_string(), day1::p1);
    puzzles.insert("1-2".to_string(), day1::p2);
    puzzles.insert("2-1".to_string(), day2::p1);
    puzzles.insert("2-2".to_string(), day2::p2);
    puzzles.insert("3-1".to_string(), day3::p1);
    puzzles.insert("3-2".to_string(), day3::p2);
    puzzles.insert("4-1".to_string(), day4::p1);
    puzzles.insert("4-2".to_string(), day4::p2);

    let input = fs::read_to_string(&args.input_path).expect("Unable to read file");

    puzzles[&args.puzzle_id](input);
}
