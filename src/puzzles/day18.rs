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

fn parse_input_p2(input: String) -> (Vec<Edge>, Point, Point) {
    let re = Regex::new(r"(?:(?:R|D|L|U)) (?:\d+) \(#(?<color>......)\)").unwrap();
    
    let new_input = input.trim().split("\n").map(|line|
    {
        let c = &re.captures(line).unwrap()["color"];
        let mut chars = c.chars();
        let dir = match chars.next_back().unwrap() {
            '0' => 'R',
            '1' => 'D',
            '2' => 'L',
            '3' => 'U',
            _ => panic!(),
        };
        let length = u32::from_str_radix(chars.as_str(), 16).unwrap();
        
        return format!("{} {} (#{})\n", dir, length, c);
    }).collect::<String>();
    
    return parse_input(new_input);
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


fn _represent_grid_p2(grid: Vec<Vec<Option<char>>>) -> String {
    grid.iter()
        .map(|line| {
            line.iter()
                .map(|point| {
                    if point.is_none() {
                        '.'
                    } else {
                        *point.as_ref().unwrap()
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
        for x in 0..grid[y].len() {
            let geo_coords = Point{x: x as i32 - offset.x, y: y as i32 - offset.y};
            for edge in edges.iter() {
                if edge.has_point(&geo_coords) {
                    grid[y][x] = Some( //Some(edge.color.clone());
                        match edge._dir {
                            'U' => "▲".to_string(),
                            'D' => "▼".to_string(),
                            'L' => "◄".to_string(),
                            'R' => "►".to_string(),
                            _ => panic!(),
                        });
                }
            }
        }
    }

    return grid;
}

fn _get_point_inside_polygon(grid: &Vec<Vec<Option<String>>>) -> Option<Point> {
    // heuristic for getting a point that is inside the grid
    let mut wall_encountered = false;
    let y = grid.len()/2;
    for (x, val) in grid[y].iter().enumerate() {
        if val.is_some() {
            wall_encountered = true;
        }
        if val.is_none() && wall_encountered {
            return Some(Point{x: x as i32, y: y as i32}); 
        }
    }
    return None;
}

fn _flood_point(grid: &mut Vec<Vec<Option<String>>>, p: Point) {
    let y = p.y as usize;
    let x = p.x as usize;
    grid[y][x] = Some("ffffff".to_string());
    if y - 1 > 0 && grid[y-1][x].is_none() {
        _flood_point(grid, Point{y: p.y-1, x: p.x});
    }
    if y + 1 < grid.len() && grid[y+1][x].is_none() {
        _flood_point(grid, Point{y: p.y+1, x: p.x});
    }
    if x - 1 > 0 && grid[y][x-1].is_none() {
        _flood_point(grid, Point{y: p.y, x: p.x-1});
    }
    if x + 1 < grid[y].len() && grid[y][x+1].is_none() {
        _flood_point(grid, Point{y: p.y, x: p.x+1});
    }
}

fn flood_fill_grid(grid: &mut Vec<Vec<Option<String>>>) {
    _flood_point(grid, _get_point_inside_polygon(grid).unwrap());
}

pub fn p1(input: String) {
    let (edges, ful, fdr) = parse_input(input);
    let mut grid = build_grid(edges, ful, fdr);

    flood_fill_grid(&mut grid);
    
    //println!("{}", _represent_grid(grid));
    println!("{}", compute_volume(grid));
}


fn compute_polygon(
    edges: Vec<Edge>,
    farthest_ul: Point,
    farthest_dr: Point,
) -> u64 {
    let offset = Point{x: -farthest_ul.x, y: -farthest_ul.y};
    let nb_lines = (farthest_dr.y + 1 + offset.y) as usize;
    let line_length = (farthest_dr.x + 1 + offset.x) as usize;
    
    let mut total_area = 0u64;
    
    //let mut _grid: Vec<Vec<Option<char>>> = Vec::new();
    
    let mut last_line : Vec<Option<char>> = vec![None; line_length];
    for y in 0..nb_lines {
        let mut current_line = vec![None; line_length];
        for x in 0..line_length {
            let geo_coords = Point{x: x as i32 - offset.x, y: y as i32 - offset.y};
            let mut edge_found = false;
            for edge in edges.iter() {
                if edge.has_point(&geo_coords) {
                    current_line[x] = Some(
                        match edge._dir {
                            'U' => '▲',
                            'D' => '▼',
                            'L' => '◄',
                            'R' => '►',
                            _ => panic!(),
                        });
                        total_area += 1;
                    edge_found = true;
                }
            }
            if !edge_found {
                // secret sauce: look at the previous line to guess whether we are inside the polygon
                // this actually would work in all four direction so we could go 4x faster by splitting the grid
                // and going at it in parallel.
                // but that's not going to save me, this is just too slow.
                if let Some(c) = last_line[x] {
                    if (c == '►' || c == '▼' || c == '%') && ( x==0 || current_line[x-1].is_some() ){
                        current_line[x] = Some('%');
                        total_area += 1;
                    }
                }
            }
        }
        //_grid.push(current_line.clone());
        last_line = current_line;
        if y%100 == 0 {
            println!("line {:>20}/{}", y, nb_lines);
        }
    }
    //println!("{}", _represent_grid_p2(_grid));
    
    return total_area;
}


pub fn p2(input: String) {
    let (edges, ful, fdr) = parse_input_p2(input);
    let area = compute_polygon(edges, ful, fdr);

    println!("{}", area);
}
