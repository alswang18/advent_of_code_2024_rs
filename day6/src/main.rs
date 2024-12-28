#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
enum Direction {
    Up = 0,
    Right,
    Down,
    Left,
}

#[derive(Clone)]
struct Guard {
    direction: Direction,
    location: (i32, i32),
    // location hashes direction of guard and the row col coordinates.
    seen_locations: std::collections::HashSet<(Direction, (i32, i32))>,
}

impl Guard {
    fn turn_90_deg(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn next_loc(&mut self) -> (i32, i32) {
        match self.direction {
            Direction::Up => (self.location.0 - 1, self.location.1),
            Direction::Right => (self.location.0, self.location.1 + 1),
            Direction::Down => (self.location.0 + 1, self.location.1),
            Direction::Left => (self.location.0, self.location.1 - 1),
        }
    }

    fn symbol_to_direction(symbol: char) -> Result<Direction, String> {
        match symbol {
            '<' => Ok(Direction::Left),
            '^' => Ok(Direction::Up),
            '>' => Ok(Direction::Right),
            'v' => Ok(Direction::Down),
            _ => Err(format!(
                "Unexpected Symbol {}. Needs to be one of <, ^, >, or v.",
                symbol
            )),
        }
    }
}

fn is_inside_grid(grid: &Vec<Vec<char>>, r: i32, c: i32) -> bool {
    r >= 0 && c >= 0 && (r as usize) < grid.len() && (c as usize) < grid[0].len()
}

fn causes_loop(grid: &Vec<Vec<char>>, mut guard: Guard, obstruction: (i32, i32)) -> bool {
    if grid[obstruction.0 as usize][obstruction.1 as usize] == '#' {
        return false;
    }

    loop {
        debug_assert!(is_inside_grid(grid, guard.location.0, guard.location.1));

        if guard
            .seen_locations
            .contains(&(guard.direction, guard.location))
        {
            return true;
        }

        guard
            .seen_locations
            .insert((guard.direction, guard.location));

        let (next_r, next_c) = guard.next_loc();
        if !is_inside_grid(grid, next_r, next_c) {
            return false;
        }

        if grid[next_r as usize][next_c as usize] == '#' || (next_r, next_c) == obstruction {
            guard.turn_90_deg();
        } else {
            guard.location = (next_r, next_c);
        }
    }
}

fn simulation_6b(grid: &Vec<Vec<char>>, guard: Guard) -> i32 {
    let mut count = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            count += if causes_loop(&grid, guard.clone(), (i as i32, j as i32)) {
                1
            } else {
                0
            }
        }
    }
    count
}

fn simulation_6a(grid: &Vec<Vec<char>>, mut guard: Guard) -> i32 {
    loop {
        debug_assert!(is_inside_grid(grid, guard.location.0, guard.location.1));

        guard
            .seen_locations
            .insert((guard.direction, guard.location));

        let (next_r, next_c) = guard.next_loc();
        if !is_inside_grid(grid, next_r, next_c) {
            break;
        }

        if grid[next_r as usize][next_c as usize] == '#' {
            guard.turn_90_deg();
        } else {
            guard.location = (next_r, next_c);
        }
    }

    let locs = guard
        .seen_locations
        .clone()
        .into_iter()
        .map(|(_, location)| location)
        .collect::<std::collections::HashSet<_>>()
        .len();

    locs as i32
}

fn main() {
    let grid_string = include_str!("../data/input.txt");

    let grid: Vec<Vec<char>> = grid_string
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    // We need to find the guard to initialize it.
    let guard_symbols = ['<', '^', '>', 'v'];

    let rows = grid.len();
    let cols = grid[0].len();

    let mut guard = Guard {
        direction: Direction::Up,
        location: (0, 0),
        seen_locations: std::collections::HashSet::new(),
    };
    for r in 0..rows {
        for c in 0..cols {
            if guard_symbols.contains(&grid[r][c]) {
                match Guard::symbol_to_direction(grid[r][c]) {
                    Ok(dir) => guard.direction = dir,
                    Err(msg) => {
                        eprintln!("Error: {}", msg);
                        std::process::exit(1);
                    }
                }
                guard.location.0 = r as i32;
                guard.location.1 = c as i32;
            }
        }
    }

    let mut timer = std::time::Instant::now();
    let count_6a = simulation_6a(&grid, guard.clone());
    println!("6a count is: {}", count_6a);
    println!("Time in is {}ms", timer.elapsed().as_millis());

    timer = std::time::Instant::now();
    let count_6b = simulation_6b(&grid, guard.clone());
    println!("6b count is: {}", count_6b);
    println!("Time in is {}ms", timer.elapsed().as_millis());
}
