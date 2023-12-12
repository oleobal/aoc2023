use std::{
    fmt::{Display, Formatter},
    iter,
    str::Chars,
};

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "src/puzzles/day12grammar.pest"]
struct Day12Parser;

#[derive(Debug, Hash)]
struct Record {
    map: String,
    damaged_count: Vec<u32>,
}

impl Display for Record {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "({}, {:?})", self.map, self.damaged_count)
    }
}

fn parse_input(input: &str) -> Vec<Record> {
    let tokens = Day12Parser::parse(Rule::Records, input).unwrap();

    let mut result = Vec::new();
    for record in tokens {
        let mut inner = record.into_inner();
        let spring_map = inner.next().unwrap().as_str();

        let dcount: Vec<u32> = inner
            .next()
            .unwrap()
            .into_inner()
            .map(|it| it.as_str().parse::<u32>().unwrap())
            .collect();

        result.push(Record {
            map: spring_map.to_string(),
            damaged_count: dcount,
        });
    }
    return result;
}

/// Returns a list of (option, counts that option fullfils)
///
/// # Arguments
///
/// * `candidate_block` - list of either '#' or '?'
/// * `damaged_count` - list of length of continuous `#` blocks that can be found in the block
fn match_candidate_block(
    candidate_block: Vec<char>,
    damaged_count: &Vec<u32>,
) -> Vec<(Vec<char>, Vec<u32>)> {
    let mut options = Vec::new();
    // todo

    if damaged_count.len() == 0 {
        // nothing to be found there so it should be all dots
        return Vec::from([(
            candidate_block
                .iter()
                .map(|c| if *c == '?' { '.' } else { *c })
                .collect(),
            Vec::new(),
        )]);
    }
    
    if true {
        
        
    }

    return options;
}

fn complete(next: &mut Chars, damaged_count: &Vec<u32>) -> Vec<Vec<char>> {
    let mut ongoing_damaged_block: Vec<char> = Vec::new();

    let mut options = Vec::new();

    while let Some(c) = next.next() {
        if c == '#' || c == '?' {
            ongoing_damaged_block.push(c)
        } else if c == '.' {
            if !ongoing_damaged_block.is_empty() {
                let possibilities = match_candidate_block(ongoing_damaged_block, damaged_count);

                // todo

                return complete(next, damaged_count);
            }
            // do nothing
        } else {
            panic!("unknown symbol")
        }
    }
    return options;
}

pub fn p1(input: String) {
    let records = parse_input(&input);

    for record in records {
        println!("{}", record);
    }

    let m = match_candidate_block(Vec::from(['#', '#', '#']), &Vec::from([3]));

    println!("{:?}", m);
}
