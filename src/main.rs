use clap::Parser;
use std::collections::HashMap;
use std::fs;

mod puzzles;

use crate::puzzles::*;
use util::*;

#[derive(Parser)]
struct CliArgs {
    puzzle_id: String,
    input_path: std::path::PathBuf,
}

fn main() {
    let args = CliArgs::parse();

    let mut puzzles: HashMap<String, fn(String)> = HashMap::new();
    register_puzzles_for_days!(7);
    register_puzzle!(11, 1);
    register_puzzle!(11, 2);
    register_puzzle!(12, 1);
    register_puzzle!(12, 2);
    register_puzzle!(14, 1);
    register_puzzle!(14, 2);
    register_puzzle!(15, 1);
    register_puzzle!(15, 2);
    register_puzzle!(18, 1);

    let input = fs::read_to_string(&args.input_path).expect("Unable to read file");

    puzzles[&args.puzzle_id](input);
}
