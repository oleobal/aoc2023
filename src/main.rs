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

    let input = fs::read_to_string(&args.input_path).expect("Unable to read file");

    puzzles[&args.puzzle_id](input);
}
