use pest_derive::Parser;
use pest::Parser;

#[derive(Parser)]
#[grammar = "src/puzzles/day19grammar.pest"]
struct Day19Parser;

pub fn p1(input: String) {
    let tokens = Day19Parser::parse(Rule::Program, &input).unwrap();
}