use std::cmp::{Eq, Ord};
use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;
use std::iter::zip;
use std::str::FromStr;

use pest::Parser;
use pest_derive::Parser;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use strum_macros::EnumString;

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

// order: "AKQJT98765432"
#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Debug, EnumIter, Clone, Copy)]
enum Card {
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    CT,
    CJ,
    CQ,
    CK,
    CA,
}

impl FromStr for Card {
    type Err = ();
    fn from_str(input: &str) -> Result<Card, Self::Err> {
        match input {
            "A" => Ok(Card::CA),
            "K" => Ok(Card::CK),
            "Q" => Ok(Card::CQ),
            "J" => Ok(Card::CJ),
            "T" => Ok(Card::CT),
            "9" => Ok(Card::C9),
            "8" => Ok(Card::C8),
            "7" => Ok(Card::C7),
            "6" => Ok(Card::C6),
            "5" => Ok(Card::C5),
            "4" => Ok(Card::C4),
            "3" => Ok(Card::C3),
            "2" => Ok(Card::C2),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Hand {
    id: String,
    cards: HashMap<Card, u32>,
    hand_type: (HandType, Vec<Card>, Vec<Card>),
}

impl FromStr for Hand {
    type Err = ();
    fn from_str(input: &str) -> Result<Hand, Self::Err> {
        let mut char_buffer: [u8; 4] = [0; 4];

        let mut card_count = HashMap::new();

        input
            .chars()
            .map(|c| Card::from_str(c.encode_utf8(&mut char_buffer)).unwrap())
            .for_each(|c| {
                *(card_count.entry(c).or_insert(0)) += 1;
            });

        let hand_type = get_type(&card_count);
        return Ok(Hand {
            id: input.to_string(),
            cards: card_count,
            hand_type: hand_type,
        });
        //    _ => Err(()),
    }
}

fn find_cards(cards: &HashMap<Card, u32>, target_count: u32) -> Option<(Vec<Card>, Vec<Card>)> {
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
        if results.len() > 1 {
            results.sort_unstable();
        }
        return Some((results, leftovers));
    }
    return None;
}

/// returns (HandType, associated card(s), leftover card(s))
fn get_type(cards: &HashMap<Card, u32>) -> (HandType, Vec<Card>, Vec<Card>) {
    if let Some((res, lo)) = find_cards(&cards, 5) {
        return (HandType::FiveK, res, Vec::new());
    } else if let Some((res, lo)) = find_cards(&cards, 4) {
        return (HandType::FourK, res, lo);
    } else if let Some((res, lo)) = find_cards(&cards, 3) {
        if let Some((res_p, _)) = find_cards(&cards, 2) {
            return (
                HandType::FullHouse,
                Vec::from([res[0], res_p[0]]),
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
                        continue }
                    return Card::from_str(&s.to_string()).cmp(&Card::from_str(&o.to_string()));
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

fn parse_bids(input: &str) -> Vec<(Hand, u32)> {
    let mut tokens = Day6Parser::parse(Rule::Bids, input).unwrap();

    let mut result = Vec::new();
    for bid in tokens {
        let mut i = bid.into_inner();
        result.push((
            Hand::from_str(i.next().unwrap().as_str()).unwrap(),
            i.next().unwrap().as_str().parse::<u32>().unwrap(),
        ));
    }
    return result;
}

pub fn p1(input: String) {
    let mut bids = parse_bids(&input);
    bids.sort_by(|a, b| a.0.cmp(&b.0));

    let mut sum: u64 = 0;
    for (i, (hand, bid)) in bids.iter().enumerate() {
        //println!("{:?}", hand);
        sum += ((i + 1) as u64) * (*bid as u64);
    }

    println!("{}", sum);
}

// 246949886
// 246954242
