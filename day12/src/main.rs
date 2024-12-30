use std::collections::HashSet;

fn is_valid(row: i32, col: i32, rows: usize, cols: usize) -> bool {
    row >= 0 && col >= 0 && (row as usize) < rows && (col as usize) < cols
}

#[derive(Debug)]
struct Island {
    points: HashSet<(usize, usize)>,
}

impl Island {
    fn get_area(&self) -> u32 {
        self.points.len() as u32
    }

    fn get_perimeter(&self) -> u32 {
        const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];
        let mut perimeter = 0;
        for (row, col) in self.points.iter() {
            for (d_r, d_c) in DIRECTIONS {
                let pos_row = *row as i32 + d_r;
                let pos_col = *col as i32 + d_c;

                if pos_row < 0 || pos_col < 0 {
                    perimeter += 1;
                    continue;
                }

                let new_row = pos_row as usize;
                let new_col = pos_col as usize;

                if self.points.contains(&(new_row, new_col)) {
                    continue;
                }
                perimeter += 1;
            }
        }
        perimeter
    }

    fn get_number_of_sides(&self, rows: usize, cols: usize) -> u32 {
        const DIRECTIONS: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
        // Counting corners is a 1:1 proxy for number of sides.
        let mut num_corners = 0;
        for (row, col) in self.points.iter() {
            for (row_offset, col_offset) in DIRECTIONS {
                let row_neigh = (*row as i32 + row_offset, *col as i32);
                let col_neigh = (*row as i32, *col as i32 + col_offset);
                let diag_neigh = (*row as i32 + row_offset, *col as i32 + col_offset);

                let row_inside = is_valid(row_neigh.0, row_neigh.1, rows, cols)
                    && self
                        .points
                        .contains(&(row_neigh.0 as usize, row_neigh.1 as usize));
                let col_inside = is_valid(col_neigh.0, col_neigh.1, rows, cols)
                    && self
                        .points
                        .contains(&(col_neigh.0 as usize, col_neigh.1 as usize));
                let diag_inside = is_valid(diag_neigh.0, diag_neigh.1, rows, cols)
                    && self
                        .points
                        .contains(&(diag_neigh.0 as usize, diag_neigh.1 as usize));

                // Is an exterior corner.
                if !row_inside && !col_inside {
                    num_corners += 1;
                }

                // Is an interior corner.
                if row_inside && col_inside && !diag_inside {
                    num_corners += 1;
                }
            }
        }
        num_corners
    }
}

struct Solver {
    visited_points: HashSet<(usize, usize)>,
    islands: Vec<Island>,
}

impl Solver {
    fn solve(&mut self, grid: &Vec<Vec<char>>) {
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if let Some(island) = self.bfs(grid, row, col) {
                    self.islands.push(island);
                }
            }
        }
    }

    fn bfs(&mut self, grid: &Vec<Vec<char>>, row: usize, col: usize) -> Option<Island> {
        if self.visited_points.contains(&(row, col)) {
            return None;
        }
        const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

        let symbol = grid[row][col];

        let mut island = Island {
            points: HashSet::default(),
        };
        let mut queue: Vec<(usize, usize)> = Vec::new();
        queue.push((row, col));
        while queue.len() > 0 {
            let mut next_queue: Vec<(usize, usize)> = Vec::new();
            let iters = queue.len();

            for i in 0..iters {
                let (row, col) = queue[i];
                if self.visited_points.contains(&(row, col)) || symbol != grid[row][col] {
                    continue;
                }

                island.points.insert((row, col));
                self.visited_points.insert((row, col));

                for (d_r, d_c) in DIRECTIONS {
                    let row_cand = row as i32 + d_r;
                    let col_cand = col as i32 + d_c;

                    if is_valid(row_cand, col_cand, grid.len(), grid[0].len()) {
                        let new_row = row_cand as usize;
                        let new_col = col_cand as usize;
                        next_queue.push((new_row, new_col));
                    }
                }
            }
            queue = next_queue;
        }
        Some(island)
    }
}

fn main() {
    let input_string = include_str!("../data/input.txt");

    let grid = input_string
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut solver = Solver {
        visited_points: HashSet::default(),
        islands: Vec::new(),
    };

    solver.solve(&grid);

    let mut price_a = 0;
    let mut timer = std::time::Instant::now();

    for island in solver.islands.iter() {
        price_a += island.get_area() * island.get_perimeter();
    }

    println!("Price of islands A are: {}", price_a);
    println!("Time duration in {} us", timer.elapsed().as_micros());

    let mut price_b = 0;
    timer = std::time::Instant::now();
    for island in solver.islands.iter() {
        price_b += island.get_area() * island.get_number_of_sides(grid.len(), grid[0].len());
    }
    println!("Price of islands B are: {}", price_b);
    println!("Time duration in {} us", timer.elapsed().as_micros());
}
