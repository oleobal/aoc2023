use pest::Parser;
use pest_derive::Parser;
use std::cmp::max;
use std::ops::{Index, IndexMut};

#[derive(Parser)]
#[grammar = "src/puzzles/day2grammar.pest"]
pub struct Day2Parser;

struct Game {
    game_id: i32,
    red: i32,
    green: i32,
    blue: i32,
}

impl Index<&'_ str> for Game {
    type Output = i32;
    fn index(&self, s: &str) -> &i32 {
        match s {
            "game_id" => &self.game_id,
            "red" => &self.red,
            "green" => &self.green,
            "blue" => &self.blue,
            _ => panic!("unknown field: {}", s),
        }
    }
}

impl IndexMut<&'_ str> for Game {
    fn index_mut(&mut self, s: &str) -> &mut i32 {
        match s {
            "game_id" => &mut self.game_id,
            "red" => &mut self.red,
            "green" => &mut self.green,
            "blue" => &mut self.blue,
            _ => panic!("unknown field: {}", s),
        }
    }
}



fn parse_game(input: &str) -> Game {
    let mut tokens = Day2Parser::parse(Rule::Game, input).unwrap();
    let game_id = tokens.nth(0).unwrap().as_str().parse::<i32>().unwrap();
    
    let mut required_cubes = Game {game_id: game_id, red:0, green:0, blue:0 };
    for draw in tokens.clone()
    {
        for cubeset in draw.into_inner()
        {
            let mut pairs = cubeset.into_inner();
            let nb = pairs.next().unwrap().as_str().trim().parse::<i32>().unwrap();
            let color = pairs.last().unwrap().as_str().trim();
            required_cubes[color] = max(required_cubes[color], nb);
        }
    }
    return required_cubes;
}

pub fn p1(input: String) {
    let mut sum = 0;
    let lines = input.trim().split("\n");
    for line in lines {
        
        let required_cubes = parse_game(line);
        if required_cubes.red <= 12 && required_cubes.green <= 13 && required_cubes.blue <= 14
        {
            sum += required_cubes.game_id;
        }
    }
    println!("{}", sum);
}


pub fn p2(input: String) {
    let mut sum = 0;
    let lines = input.trim().split("\n");
    for line in lines {
        let required_cubes = parse_game(line);
        sum += required_cubes.red * required_cubes.green * required_cubes.blue;
    }
    println!("{}", sum);
}