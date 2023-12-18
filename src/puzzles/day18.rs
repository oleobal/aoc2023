use std::{
    cmp::{max, min},
    iter,
};

use regex::Regex;

/*
after much thought given to line rasterization, let's go with the naivest solution
because all lines are parallel to an axis
*/

#[derive(Debug)]
struct Edge {
    points: Vec<Point>,
    color: String,
    _origin: Point,
    _dir: char,
    _length: u32,
}

impl Edge {
    fn has_point(&self, p: &Point) -> bool {
        // this works because we built our points vector from top left to bottom right
        return self.points.first().unwrap().x <= p.x
            && self.points.first().unwrap().y <= p.y
            && self.points.last().unwrap().x >= p.x
            && self.points.last().unwrap().y >= p.y;
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Point {
    x: i32,
    y: i32,
}

/// returns (list of ops, up left coords, down right coords)
fn parse_input(input: String) -> (Vec<Edge>, Point, Point) {
    let re = Regex::new(r"(?<direction>(?:R|D|L|U)) (?<length>\d+) \(#(?<color>......)\)").unwrap();

    let mut edges = Vec::<Edge>::new();
    let mut current_coords = Point { x: 0, y: 0 };
    let mut farthest_ul_point = Point { x: 0, y: 0 };
    let mut farthest_dr_point = Point { x: 0, y: 0 };
    for line in input.trim().split("\n") {
        let c = re.captures(line).unwrap();
        let length = c["length"].parse::<i32>().unwrap();

        let points: Vec<Point> = match &c["direction"] {
            "U" => ((current_coords.y - length)..(current_coords.y))
                .map(|y| Point {
                    x: current_coords.x,
                    y: y,
                })
                .collect(),
            "L" => ((current_coords.x - length)..(current_coords.x))
                .map(|x| Point {
                    x: x,
                    y: current_coords.y,
                })
                .collect(),
            "D" => ((current_coords.y + 1)..(current_coords.y + length + 1))
                .map(|y| Point {
                    x: current_coords.x,
                    y: y,
                })
                .collect(),
            "R" => ((current_coords.x + 1)..(current_coords.x + length + 1))
                .map(|x| Point {
                    x: x,
                    y: current_coords.y,
                })
                .collect(),
            _ => panic!(),
        };

        edges.push(Edge {
            points: points,
            color: c["color"].to_string(),
            _origin: current_coords.clone(),
            _dir: c["direction"].chars().next().unwrap(),
            _length: length as u32,
        });

        match &c["direction"] {
            "U" => current_coords.y -= length,
            "L" => current_coords.x -= length,
            "D" => current_coords.y += length,
            "R" => current_coords.x += length,
            _ => panic!(),
        }
        farthest_dr_point.x = max(farthest_dr_point.x, current_coords.x);
        farthest_dr_point.y = max(farthest_dr_point.y, current_coords.y);
        farthest_ul_point.x = min(farthest_ul_point.x, current_coords.x);
        farthest_ul_point.y = min(farthest_ul_point.y, current_coords.y);
    }

    // for edge in &edges {
    //     println!("{:?}", edge);
    //     for point in &edge.points {
    //         println!(" => {:?}", *point);
    //     }
    // }
    return (edges, farthest_ul_point, farthest_dr_point);
}

fn _represent_grid(grid: Vec<Vec<Option<String>>>) -> String {
    grid.iter()
        .map(|line| {
            line.iter()
                .map(|point| {
                    if point.is_none() {
                        '.'
                    } else {
                        if point.as_ref().unwrap() == "ffffff" {
                            '$'
                        } else {
                            '#'
                        }
                    }
                })
                .collect::<String>()
                + "\n"
        })
        .collect()
}

fn compute_volume(grid: Vec<Vec<Option<String>>>) -> u32 {
    grid.iter()
        .map(|line| {
            line.iter()
                .map(|point| if point.is_some() { 1 } else { 0 })
                .sum::<u32>()
        })
        .sum()
}

fn build_grid(
    edges: Vec<Edge>,
    farthest_ul: Point,
    farthest_dr: Point,
) -> Vec<Vec<Option<String>>> {
    let offset = Point{x: -farthest_ul.x, y: -farthest_ul.y};
    let mut grid: Vec<Vec<Option<String>>> = iter::repeat(
        iter::repeat(None)
            .take(farthest_dr.x as usize + 1 + offset.x as usize)
            .collect(),
    )
    .take(farthest_dr.y as usize + 1 + offset.y as usize)
    .collect();

    for y in 0..grid.len() {
        let mut inside_polygon = false;
        let mut just_encountered_wall = false;
        for x in 0..grid[y].len() {
            let geo_coords = Point{x: x as i32 - offset.x, y: y as i32 - offset.y};
            for edge in edges.iter() {
                if edge.has_point(&geo_coords) {
                    grid[y][x] = Some(edge.color.clone());
                    
                    if edge._dir == 'U' || edge._dir == 'D' {
                        inside_polygon ^= true;
                    }
                    just_encountered_wall = true;
                }
            }
            if grid[y][x].is_none() {
                just_encountered_wall = false;
                if inside_polygon {
                    grid[y][x] = Some("ffffff".to_string());
                }
            }
        }
    }

    return grid;
}

pub fn p1(input: String) {
    let (edges, ful, fdr) = parse_input(input);
    let grid = build_grid(edges, ful, fdr);

    println!("{}", _represent_grid(grid));
    //println!("{}", compute_volume(grid));
}
