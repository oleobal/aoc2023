use petgraph::{graph::Graph, Undirected};
use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct Star {
    id: u32,
    x: u64,
    y: u64,
}

impl Star {
    fn distance(&self, other: &Star) -> u64 {
        u64::try_from(
            (self.x as i64 - other.x as i64).abs() + (self.y as i64 - other.y as i64).abs(),
        )
        .unwrap()
    }
}

fn parse_empty_cols(input: &str) -> HashSet<u64> {
    let mut cols = HashSet::new();

    let lines: Vec<&str> = input.split("\n").collect();
    for x in 0..lines[0].len() {
        if lines
            .iter()
            .map(|l| l.as_bytes()[x] as char)
            .all(|c| c == '.')
        {
            cols.insert(x as u64);
        }
    }
    return cols;
}
fn parse_empty_rows(input: &str) -> HashSet<u64> {
    let mut rows = HashSet::new();
    for (y, line) in input.split("\n").enumerate() {
        if line.chars().all(|c| c == '.') {
            rows.insert(y as u64);
        }
    }
    return rows;
}

fn parse_stars(
    input: String,
    increase_function: fn(u64, &mut u64, &HashSet<u64>),
) -> HashSet<Star> {
    let empty_rows = parse_empty_rows(&input);
    let empty_cols = parse_empty_cols(&input);

    let mut stars = HashSet::new();
    let mut star_count = 0;
    let mut real_y: u64 = 0;
    let mut fake_y: u64 = 0;
    for line in input.split("\n") {
        let mut real_x: u64 = 0;
        let mut fake_x: u64 = 0;
        for character in line.chars() {
            if character == '#' {
                star_count += 1;
                stars.insert(Star {
                    id: star_count,
                    x: fake_x,
                    y: fake_y,
                });
            }
            increase_function(real_x, &mut fake_x, &empty_cols);
            real_x += 1;
        }
        increase_function(real_y, &mut fake_y, &empty_rows);
        real_y += 1;
    }
    return stars;
}

fn build_graph(stars: HashSet<Star>) -> (Graph<Star, u64, Undirected>, u64) {
    let mut stars_buf = stars.clone();

    let mut graph = Graph::<Star, u64, Undirected>::new_undirected();

    let mut star_map = HashMap::new();

    // it is more convenient for debug purposes for the star IDs to be the same in my graph than in the example
    let mut sorted_stars = stars.iter().collect::<Vec<&Star>>();
    sorted_stars.sort_by(|a, b| a.id.cmp(&b.id));
    for star in sorted_stars.iter() {
        star_map.insert(star, graph.add_node(**star));
    }

    let mut edge_sum = 0;
    for star in stars.iter() {
        stars_buf.remove(&star);
        for other_star in &stars_buf {
            let distance = star.distance(&other_star);
            graph.add_edge(star_map[&star], star_map[&other_star], distance);
            edge_sum += distance;
        }
    }
    return (graph, edge_sum);
}

pub fn p1(input: String) {
    fn _increase_index_to_next_nonempty(real_i: u64, fake_i: &mut u64, empty: &HashSet<u64>) {
        if empty.contains(&real_i) {
            *fake_i += 1;
        }
        *fake_i += 1;
    }

    let stars = parse_stars(input.trim().to_string(), _increase_index_to_next_nonempty);
    //println!("{:#?}", stars);
    let (_graph, distances) = build_graph(stars);

    //println!("{:?}", Dot::new(&_graph));
    //println!("{:?}", graph);

    println!("{}", distances);
}

pub fn p2(input: String) {
    fn _increase_index_to_next_nonempty(real_i: u64, fake_i: &mut u64, empty: &HashSet<u64>) {
        if empty.contains(&real_i) {
            *fake_i += 1_000_000;
        } else {
            *fake_i += 1;
        }
    }

    let stars = parse_stars(input.trim().to_string(), _increase_index_to_next_nonempty);
    let (_graph, distances) = build_graph(stars);
    // seems I didn't need to actually build the graph in the end..
    // oh well, might have been useful had p2 been different

    println!("{}", distances);
}
