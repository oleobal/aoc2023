use std::collections::HashMap;
use pest::Parser;
use pest_derive::Parser;

pub fn p1(input: String) {
    let lines = input.trim().split("\n");
    let mut sum = 0;
    for line in lines {
        let first = line.chars().nth(line.find(char::is_numeric).unwrap()).unwrap();
        let last = line.chars().nth(line.rfind(char::is_numeric).unwrap()).unwrap();
        sum += format!("{}{}", first, last).parse::<i32>().unwrap();
    }
    println!("{}", sum)
}

#[derive(Parser)]
#[grammar = "src/puzzles/day1grammar.pest"]
pub struct Day1Parser;

pub fn p2(input: String) {
    let digits = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut sum = 0;
    let lines = input.trim().split("\n");
    for line in lines {
        let tokens = Day1Parser::parse(Rule::expr, line).unwrap();
        let first = digits[tokens.clone().next().unwrap().as_str()];
        let last = digits[tokens.clone().last().unwrap().as_str()];
        sum += first*10+last;
    }
    println!("{}", sum)
}