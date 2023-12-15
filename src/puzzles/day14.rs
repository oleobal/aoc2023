use std::{collections::HashMap, iter};

fn full_tilt_north(grid: &mut Vec<Vec<char>>) {
    let mut northernmost_taken_space: Vec<i32> = iter::repeat(-1).take(grid[0].len()).collect();

    // this feels more like univ C than idiomatic Rust
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let c = grid[y][x];
            if c == '#' {
                northernmost_taken_space[x] = y as i32;
            } else if c == 'O' {
                if northernmost_taken_space[x] < (y as i32) - 1 {
                    grid[(northernmost_taken_space[x] + 1) as usize][x] = 'O';
                    grid[y][x] = '.';
                    northernmost_taken_space[x] = northernmost_taken_space[x] + 1;
                } else {
                    northernmost_taken_space[x] = y as i32;
                }
            }
        }
    }
}

fn full_tilt_south(grid: &mut Vec<Vec<char>>) {
    let mut southernmost_taken_space: Vec<i32> = iter::repeat(grid.len() as i32)
        .take(grid[0].len())
        .collect();

    for y in (0..grid.len()).rev() {
        for x in 0..grid[y].len() {
            let c = grid[y][x];
            if c == '#' {
                southernmost_taken_space[x] = y as i32;
            } else if c == 'O' {
                if southernmost_taken_space[x] > (y as i32) + 1 {
                    grid[(southernmost_taken_space[x] - 1) as usize][x] = 'O';
                    grid[y][x] = '.';
                    southernmost_taken_space[x] = southernmost_taken_space[x] - 1;
                } else {
                    southernmost_taken_space[x] = y as i32;
                }
            }
        }
    }
}

fn full_tilt_west(grid: &mut Vec<Vec<char>>) {
    let mut westernmost_taken_space: Vec<i32> = iter::repeat(-1 as i32).take(grid.len()).collect();

    // not an ideal access pattern but whatever
    for x in 0..grid[0].len() {
        for y in 0..grid.len() {
            let c = grid[y][x];
            if c == '#' {
                westernmost_taken_space[y] = x as i32;
            } else if c == 'O' {
                if westernmost_taken_space[y] < (x as i32) - 1 {
                    grid[y][(westernmost_taken_space[y] + 1) as usize] = 'O';
                    grid[y][x] = '.';
                    westernmost_taken_space[y] = westernmost_taken_space[y] + 1;
                } else {
                    westernmost_taken_space[y] = x as i32;
                }
            }

            //println!("(x={} y={})\n{:?}\n{}\n", x, y, westernmost_taken_space, _represent_grid(grid.to_vec()));
        }
    }
}

fn full_tilt_east(grid: &mut Vec<Vec<char>>) {
    let mut easternmost_taken_space: Vec<i32> = iter::repeat(grid[0].len() as i32)
        .take(grid.len())
        .collect();

    for x in (0..grid[0].len()).rev() {
        for y in 0..grid.len() {
            let c = grid[y][x];
            if c == '#' {
                easternmost_taken_space[y] = x as i32;
            } else if c == 'O' {
                if easternmost_taken_space[y] > (x as i32) + 1 {
                    grid[y][(easternmost_taken_space[y] - 1) as usize] = 'O';
                    grid[y][x] = '.';
                    easternmost_taken_space[y] = easternmost_taken_space[y] - 1;
                } else {
                    easternmost_taken_space[y] = x as i32;
                }
            }
        }
    }
}

fn cycle_grid(grid: &mut Vec<Vec<char>>) {
    full_tilt_north(grid);
    full_tilt_west(grid);
    full_tilt_south(grid);
    full_tilt_east(grid);
}

fn compute_north_load(grid: &Vec<Vec<char>>) -> u64 {
    grid.iter()
        .rev()
        .enumerate()
        .map(|(i, line)| ((i + 1) as u64) * line.iter().filter(|c| **c == 'O').count() as u64)
        .sum()
}

fn parse(input: String) -> Vec<Vec<char>> {
    return input
        .trim()
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();
}

fn represent_grid(grid: &Vec<Vec<char>>) -> String {
    return grid
        .into_iter()
        .map(|line| line.iter().collect())
        .collect::<Vec<String>>()
        .join("\n");
}

pub fn p1(input: String) {
    let grid = &mut parse(input);
    full_tilt_north(grid);
    println!("{}", compute_north_load(grid));
}

pub fn p2(input: String) {
    let mut grid = parse(input);

    let target_cycles = 1_000_000_000;

    let mut states_map = HashMap::<String, u32>::new();
    let mut states_list = Vec::<Vec<Vec<char>>>::new();

    states_map.insert(represent_grid(&grid), 0);
    states_list.push(grid.clone());

    for i in 1..(500 as u32) {
        cycle_grid(&mut grid);
        //print!("{:>5.2}%\r", i as f32 /target_cycles as f32 * 100 as f32)

        let repr = represent_grid(&grid);
        if let Some(early_iter) = states_map.get(&repr) {
            // f(n) = f(init_size+((n-init_size)%cycle_size)
            let equivalent_target =
                (*early_iter + ((target_cycles - *early_iter) % (i - *early_iter))) as usize;

            println!(
                "{}",
                compute_north_load(states_list.get(equivalent_target).unwrap())
            );
            break;
        } else {
            states_map.insert(repr, i);
            states_list.push(grid.clone());
        }
    }
}
