use std::collections::HashMap;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "src/puzzles/day6grammar.pest"]
struct Day6Parser;

fn parse_records(input: &str) -> HashMap<u64, u64> {
    let mut tokens = Day6Parser::parse(Rule::Records, input).unwrap();

    let times = tokens
        .next()
        .unwrap()
        .into_inner()
        .map(|it| it.as_str().parse::<u64>().unwrap());
    let distances = tokens
        .next()
        .unwrap()
        .into_inner()
        .map(|it| it.as_str().parse::<u64>().unwrap());

    let mut m = HashMap::new();
    for (t, d) in times.zip(distances) {
        m.insert(t, d);
    }
    return m;
}

fn parse_record(input: &str) -> (u64, u64) {
    let mut tokens = Day6Parser::parse(Rule::OneBigRecord, input).unwrap();
    let time = tokens.next().unwrap().as_str().replace(" ", "").parse::<u64>().unwrap();
    let distance = tokens.next().unwrap().as_str().replace(" ", "").parse::<u64>().unwrap();
    return (time, distance);
}

fn tally_record_breaks(time: &u64, record_distance: &u64) -> usize {
    (1..*time)
        .map(|time_held| time_held * (time - time_held))
        .filter(|dist| *dist > *record_distance)
        .count()
}

// this is a linear optimization problem
// unfortunately I remember none of my classes on the subject
// let's just try every option..
pub fn p1(input: String) {
    let records = parse_records(&input);

    let res = records.iter().map(|(t, d)| tally_record_breaks(t, d)).fold(1, |acc, x| acc*x);
    
    println!("{}", res);
}

pub fn p2(input: String) {
    let (record_time, record_distance) = parse_record(&input);
    
    let res = tally_record_breaks(&record_time, &record_distance);
    println!("{}", res);
}
