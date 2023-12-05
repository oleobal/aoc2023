use pest::Parser;
use pest_derive::Parser;
use std::cmp::min;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "src/puzzles/day5grammar.pest"]
struct Day5Parser;

struct AlmanacMapping {
    source_start: u64,
    dest_start: u64,
    range: u64,
}

struct Almanac {
    seeds: Vec<u64>,
    maps: HashMap<String, Vec<AlmanacMapping>>,
}

fn get_order_of_operations() -> [&'static str; 7] {
    return [
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ];
}

fn map_with_almanac(source: u64, ranges: &Vec<AlmanacMapping>) -> u64 {
    for range in ranges {
        if source >= range.source_start && source < (range.source_start + range.range) {
            return range.dest_start + (source - range.source_start);
        }
    }
    return source;
}

fn parse_almanac(input: &str) -> Almanac {
    let mut tokens = Day5Parser::parse(Rule::Almanac, input).unwrap();

    let seeds: Vec<u64> = tokens
        .next()
        .unwrap()
        .into_inner()
        .map(|it| it.as_str().parse::<u64>().unwrap())
        .collect();

    let mut almanac_mappings: HashMap<String, Vec<AlmanacMapping>> = HashMap::new();
    for almamap in tokens {
        let mut alma_tokens = almamap.into_inner();
        let header = alma_tokens.next().unwrap().as_str();
        let mut mappings: Vec<AlmanacMapping> = Vec::new();

        for token in alma_tokens {
            let numbers: Vec<u64> = token
                .into_inner()
                .map(|it| it.as_str().parse::<u64>().unwrap())
                .collect();

            mappings.push(AlmanacMapping {
                source_start: numbers[1],
                dest_start: numbers[0],
                range: numbers[2],
            })
        }
        almanac_mappings.insert(header.to_string(), mappings);
    }

    return Almanac {
        seeds: seeds,
        maps: almanac_mappings,
    };
}

pub fn p1(input: String) {
    let almanac = parse_almanac(&input);

    let mut closest_location: Option<u64> = None;
    for seed in almanac.seeds {
        let mut dest = seed;
        for op in get_order_of_operations() {
            dest = map_with_almanac(dest, &almanac.maps[op])
        }
        if closest_location.is_none() {
            closest_location = Some(dest);
        } else {
            closest_location = min(closest_location, Some(dest));
        }
    }
    println!("{}", closest_location.unwrap());
}

// this is inefficient because I compute the same thing many times
// in retrospect a graph might be much better suited
// but it still only took 5mn on my machine to compute so hey
pub fn p2(input: String) {
    let almanac = parse_almanac(&input);

    let mut seeds: Vec<u64> = Vec::new();

    for i in 0..(almanac.seeds.len() / 2) {
        seeds.extend(almanac.seeds[i * 2]..(almanac.seeds[i * 2] + almanac.seeds[i * 2 + 1]))
    }

    let mut closest_location: Option<u64> = None;

    for seed in seeds {
        let mut dest = seed;
        for op in get_order_of_operations() {
            dest = map_with_almanac(dest, &almanac.maps[op])
        }
        if closest_location.is_none() {
            closest_location = Some(dest);
        } else {
            closest_location = min(closest_location, Some(dest));
        }
    }
    println!("{}", closest_location.unwrap());
}
