use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "src/puzzles/day4grammar.pest"]
pub struct Day4Parser;

struct Card {
    id: i32,
    winning: Vec<i32>,
    scratched: Vec<i32>,
}

impl Card {
    fn matches(&self) -> u32 {
        return self
            .scratched
            .iter()
            .fold(0, |acc, it| acc + (self.winning.contains(it) as u32));
    }

    fn points(&self) -> u32 {
        let matches = self.matches();
        if matches == 0 {
            return 0;
        }
        return u32::pow(2, matches - 1);
    }
}

fn parse_game(input: &str) -> Vec<Card> {
    let mut tokens = Day4Parser::parse(Rule::Cards, input).unwrap();

    let mut cards = Vec::new();

    while tokens.peek().is_some() {
        let card_id = tokens.next().unwrap().as_str().parse::<i32>().unwrap();
        let winning: Vec<i32> = tokens
            .next()
            .unwrap()
            .into_inner()
            .map(|it| it.as_str().parse::<i32>().unwrap())
            .collect();
        let scratched: Vec<i32> = tokens
            .next()
            .unwrap()
            .into_inner()
            .map(|it| it.as_str().parse::<i32>().unwrap())
            .collect();
        cards.push(Card {
            id: card_id,
            scratched: scratched,
            winning: winning,
        });
    }
    return cards;
}

pub fn p1(input: String) {
    let cards = parse_game(&input);

    for card in cards.iter() {
        println!(
            "{:?} {:?} | {:?} => {:?}",
            card.id,
            card.winning,
            card.scratched,
            card.points()
        );
    }

    let sum = cards.iter().fold(0, |acc, card| acc + card.points());

    println!("{}", sum);
}

fn resolve_scratchcard(card: &Card, following_card_stock: &[Card]) -> u32 {
    let matches = card.matches();

    if matches == 0 {
        return 1;
    }

    let mut won_cards = 1; // including this one
    for m in 1..matches + 1 {
        let index = usize::try_from(m).unwrap();
        won_cards += resolve_scratchcard(
            &following_card_stock[index - 1],
            &following_card_stock[index..],
        )
    }

    return won_cards;
}

pub fn p2(input: String) {
    let original_cards = parse_game(&input);

    let mut sum = 0;
    for i in 0..original_cards.len() {
        let res = resolve_scratchcard(&original_cards[i], &original_cards[i + 1..]);
        sum += res;
    }
    println!("{}", sum);
}
