use std::collections::HashSet;

fn is_valid_pos(row: i32, col: i32, rows: usize, cols: usize) -> bool {
    row >= 0 && col >= 0 && row < rows as i32 && col < cols as i32
}

struct Solver {
    set: HashSet<(usize, usize)>,
}

impl Solver {
    fn dfs_number_of_9s(&mut self, grid: &Vec<Vec<u32>>, row: usize, col: usize, cur: u32) -> u32 {
        if self.set.contains(&(row, col)) {
            return 0;
        }

        self.set.insert((row, col));

        if cur == 9 && grid[row][col] == 9 {
            return 1;
        }

        let mut res = 0;
        const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

        for dir in DIRECTIONS {
            let d_row = row as i32 + dir.0;
            let d_col = col as i32 + dir.1;

            if is_valid_pos(d_row, d_col, grid.len(), grid[0].len()) {
                let new_row = d_row as usize;
                let new_col = d_col as usize;

                if grid[new_row][new_col] == (cur + 1) {
                    res += self.dfs_number_of_9s(grid, new_row, new_col, cur + 1);
                }
            }
        }
        res
    }

    fn dfs_number_of_paths(&self, grid: &Vec<Vec<u32>>, row: usize, col: usize, cur: u32) -> u32 {
        if cur == 9 && grid[row][col] == 9 {
            return 1;
        }

        let mut res = 0;
        const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

        for dir in DIRECTIONS {
            let d_row = row as i32 + dir.0;
            let d_col = col as i32 + dir.1;

            if is_valid_pos(d_row, d_col, grid.len(), grid[0].len()) {
                let new_row = d_row as usize;
                let new_col = d_col as usize;

                if grid[new_row][new_col] == (cur + 1) {
                    res += self.dfs_number_of_paths(grid, new_row, new_col, cur + 1);
                }
            }
        }
        res
    }
}

fn main() {
    let input = include_str!("../data/input.txt");

    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    debug_assert!(grid.len() > 0);
    let rows = grid.len();
    let cols = grid[0].len();

    let mut count_a = 0;
    let mut timer = std::time::Instant::now();
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == 0 {
                let mut solver = Solver {
                    set: HashSet::new(),
                };
                count_a += solver.dfs_number_of_9s(&grid, r, c, 0);
            }
        }
    }
    println!("Count 10a: {}", count_a);
    println!("Time duration in {} us", timer.elapsed().as_micros());
    timer = std::time::Instant::now();
    let mut count_b = 0;
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == 0 {
                let solver = Solver {
                    set: HashSet::new(),
                };
                count_b += solver.dfs_number_of_paths(&grid, r, c, 0);
            }
        }
    }

    println!("Count 10b: {}", count_b);
    println!("Time duration in {} us", timer.elapsed().as_micros());
}
