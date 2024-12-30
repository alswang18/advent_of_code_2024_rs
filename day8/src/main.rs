use std::collections::{HashMap, HashSet};

fn is_inside_grid(grid: &Vec<Vec<char>>, loc: (i32, i32)) -> bool {
    loc.0 >= 0 && loc.1 >= 0 && loc.0 < grid.len() as i32 && loc.1 < grid[0].len() as i32
}

fn gen_valid_antinodes_8a(grid: &Vec<Vec<char>>, locs: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut res: Vec<(i32, i32)> = Vec::new();

    for i in 0..locs.len() {
        for j in i + 1..locs.len() {
            let x_diff = locs[i].0 - locs[j].0;
            let y_diff = locs[i].1 - locs[j].1;

            let antinode_1 = (locs[i].0 + x_diff, locs[i].1 + y_diff);
            if is_inside_grid(grid, antinode_1) {
                res.push(antinode_1);
            }
            let antinode_2 = (locs[j].0 - x_diff, locs[j].1 - y_diff);
            if is_inside_grid(grid, antinode_2) {
                res.push(antinode_2);
            }
        }
    }

    res
}

fn gen_valid_antinodes_8b(grid: &Vec<Vec<char>>, locs: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut res: Vec<(i32, i32)> = Vec::new();

    for loc in locs.iter() {
        res.push(*loc);
    }

    for i in 0..locs.len() {
        for j in i + 1..locs.len() {
            let x_diff = locs[i].0 - locs[j].0;
            let y_diff = locs[i].1 - locs[j].1;

            // This antinode is for the left-wards points.
            let mut left_antinode = (locs[i].0 + x_diff, locs[i].1 + y_diff);
            while is_inside_grid(grid, left_antinode) {
                res.push(left_antinode);
                left_antinode = (left_antinode.0 + x_diff, left_antinode.1 + y_diff);
            }

            let mut right_antinode = (locs[j].0 - x_diff, locs[j].1 - y_diff);
            while is_inside_grid(grid, right_antinode) {
                res.push(right_antinode);
                right_antinode = (right_antinode.0 - x_diff, right_antinode.1 - y_diff);
            }
        }
    }

    res
}

fn main() {
    let grid_string = include_str!("../data/input.txt");

    let grid = grid_string
        .lines()
        .into_iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut symbol_to_loc: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let c = grid[i][j];
            if c == '.' {
                continue;
            }

            symbol_to_loc
                .entry(c)
                .or_default()
                .push((i as i32, j as i32));
        }
    }
    let mut timer = std::time::Instant::now();
    let mut set_8a: HashSet<(i32, i32)> = HashSet::new();
    for v in symbol_to_loc.values() {
        set_8a.extend(gen_valid_antinodes_8a(&grid, v).into_iter());
    }
    println!("Time duration in {} us", timer.elapsed().as_micros());
    println!("Count of unique antinodes 8a: {}", set_8a.len());
    timer = std::time::Instant::now();
    let mut set_8b: HashSet<(i32, i32)> = HashSet::new();
    for v in symbol_to_loc.values() {
        set_8b.extend(gen_valid_antinodes_8b(&grid, v).into_iter());
    }
    println!("Time duration in {} us", timer.elapsed().as_micros());

    println!("Count of unique antinodes 8b: {}", set_8b.len());
}
