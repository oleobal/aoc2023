use std::cmp::{Eq, Ord};
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::zip;
use std::str::FromStr;

use pest::Parser;
use pest_derive::Parser;

const REAL_POKER_RULES: bool = false;

#[derive(Parser)]
#[grammar = "src/puzzles/day7grammar.pest"]
struct Day6Parser;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    Pair,
    TwoPairs,
    ThreeK,
    FullHouse,
    FourK,
    FiveK,
}


fn get_card_rank(c: char) -> usize {
    "AKQJT98765432X" // using X for special joker type
        .chars()
        .rev()
        .position(|it| it == c)
        .unwrap()
}

#[derive(Debug)]
struct Hand {
    id: String,
    cards: HashMap<char, u32>,
    hand_type: (HandType, Vec<char>, Vec<char>),
}

impl FromStr for Hand {
    type Err = ();
    fn from_str(input: &str) -> Result<Hand, Self::Err> {
        let mut char_buffer: [u8; 4] = [0; 4];

        let mut card_count = HashMap::new();

        input.chars().for_each(|c| {
            *(card_count.entry(c).or_insert(0)) += 1;
        });

        let hand_type = get_jokerized_type(&card_count);
        return Ok(Hand {
            id: input.to_string(),
            cards: card_count,
            hand_type: hand_type,
        });
        //    _ => Err(()),
    }
}

/// find what keys in <cards> are present <target_count> times
fn find_cards(cards: &HashMap<char, u32>, target_count: u32) -> Option<(Vec<char>, Vec<char>)> {
    // there's probably a more elegant way to do this..
    let mut results = Vec::new();
    let mut leftovers = Vec::new();
    for (card, count) in cards {
        if *count == target_count {
            results.push(*card);
        } else {
            leftovers.push(*card)
        }
    }
    if results.len() > 0 {
        results.sort_unstable_by(|a, b| get_card_rank(*a).cmp(&get_card_rank(*b)));
        leftovers.sort_unstable_by(|a, b| get_card_rank(*a).cmp(&get_card_rank(*b)));
        return Some((results, leftovers));
    }
    return None;
}

/// returns (HandType, associated card(s), leftover card(s))
fn get_type(cards: &HashMap<char, u32>) -> (HandType, Vec<char>, Vec<char>) {
    if let Some((res, _)) = find_cards(&cards, 5) {
        return (HandType::FiveK, res, Vec::new());
    } else if let Some((res, lo)) = find_cards(&cards, 4) {
        return (HandType::FourK, res, lo);
    } else if let Some((res, lo)) = find_cards(&cards, 3) {
        if let Some((res_p, _)) = find_cards(&cards, 2) {
            return (
                HandType::FullHouse,
                Vec::from([res_p[0], res[0]]), // less valuable pair goes first
                Vec::new(),
            );
        }
        return (HandType::ThreeK, res, lo);
    } else if let Some((res, lo)) = find_cards(&cards, 2) {
        return (
            if res.len() == 2 {
                HandType::TwoPairs
            } else {
                HandType::Pair
            },
            res,
            lo,
        );
    }
    let (res, lo) = find_cards(&cards, 1).unwrap();
    return (HandType::HighCard, res, lo);
}

fn get_jokerized_type(cards: &HashMap<char, u32>) -> (HandType, Vec<char>, Vec<char>) {
    let base = get_type(cards);

    if !cards.contains_key(&'X') {
        return base;
    }

    let ideal_card = *base
        .2.iter()
        .chain(base.1.iter())
        .filter(|c| **c != 'X').last().unwrap_or(&'A');
    let mut new_hand = cards.clone();
    *(new_hand.entry(ideal_card).or_insert(0)) += new_hand.remove(&'X').unwrap();

    return get_type(&new_hand);
}

impl std::cmp::Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type.0 == other.hand_type.0 {
            if REAL_POKER_RULES {
                // iterate through associated cards first, then leftovers
                for (s, o) in zip(
                    self.hand_type.1.iter().rev(),
                    other.hand_type.1.iter().rev(),
                ) {
                    if s == o {
                        continue;
                    }
                    return s.cmp(o);
                }
                for (s, o) in zip(
                    self.hand_type.2.iter().rev(),
                    other.hand_type.2.iter().rev(),
                ) {
                    if s == o {
                        continue;
                    }
                    return s.cmp(o);
                }
                return std::cmp::Ordering::Equal;
            } else {
                for (s, o) in zip(self.id.chars(), other.id.chars()) {
                    if s == o {
                        continue;
                    }
                    return get_card_rank(s).cmp(&get_card_rank(o));
                }
            }
        }
        return self.hand_type.0.cmp(&other.hand_type.0);
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl Eq for Hand {}

fn parse_bids(input: &str, string_op: fn(&str) -> String) -> Vec<(Hand, u32)> {
    let tokens = Day6Parser::parse(Rule::Bids, input).unwrap();

    let mut result = Vec::new();
    for bid in tokens {
        let mut i = bid.into_inner();
        result.push((
            Hand::from_str(&string_op(i.next().unwrap().as_str())).unwrap(),
            i.next().unwrap().as_str().parse::<u32>().unwrap(),
        ));
    }
    return result;
}

fn compute_sum(input: String, string_op: fn(&str) -> String) -> u64 {
    let mut bids = parse_bids(&input, string_op);
    bids.sort_by(|a, b| a.0.cmp(&b.0));

    let mut sum: u64 = 0;
    for (i, (_hand, bid)) in bids.iter().enumerate() {
        //println!("{:?}", _hand);
        sum += ((i + 1) as u64) * (*bid as u64);
    }

    return sum;
}

pub fn p1(input: String) {
    println!("{}", compute_sum(input, |x| x.to_string()));
}



pub fn p2(input: String) {
    println!("{}", compute_sum(input, |x| x.replace("J", "X")));
}
