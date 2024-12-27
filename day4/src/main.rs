fn is_valid_coord(grid: &Vec<Vec<char>>, row: i32, col: i32) -> bool {
    row >= 0 && col >= 0 && row < grid.len() as i32 && col < grid[0].len() as i32
}

fn word_in_dir(grid: &Vec<Vec<char>>, word: &str, row: i32, col: i32, dir: (i32, i32)) -> bool {
    let mut cursor = (row, col);
    for i in 0..word.len() {
        if !is_valid_coord(grid, cursor.0, cursor.1)
            || grid[cursor.0 as usize][cursor.1 as usize] != word.chars().nth(i as usize).unwrap()
        {
            return false;
        }
        cursor = (cursor.0 + dir.0, cursor.1 + dir.1);
    }
    true
}

fn mas_in_coord(grid: &Vec<Vec<char>>, row: i32, col: i32) -> bool {
    if grid[row as usize][col as usize] != 'A' {
        return false;
    }

    let diag_indexes = ((row - 1, col - 1), (row + 1, col + 1));
    let counter_diag_indexes = ((row - 1, col + 1), (row + 1, col - 1));
    if !is_valid_coord(grid, diag_indexes.0 .0, diag_indexes.0 .1)
        || !is_valid_coord(grid, diag_indexes.1 .0, diag_indexes.1 .1)
        || !is_valid_coord(grid, counter_diag_indexes.0 .0, counter_diag_indexes.0 .1)
        || !is_valid_coord(grid, counter_diag_indexes.1 .0, counter_diag_indexes.1 .1)
    {
        return false;
    }

    let diag = (
        grid[diag_indexes.0 .0 as usize][diag_indexes.0 .1 as usize],
        grid[diag_indexes.1 .0 as usize][diag_indexes.1 .1 as usize],
    );

    let counter_diag = (
        grid[counter_diag_indexes.0 .0 as usize][counter_diag_indexes.0 .1 as usize],
        grid[counter_diag_indexes.1 .0 as usize][counter_diag_indexes.1 .1 as usize],
    );
    let cross_mas = (diag == ('M', 'S') || diag == ('S', 'M'))
        && (counter_diag == ('M', 'S') || counter_diag == ('S', 'M'));

    if !cross_mas {
        return false;
    }

    true
}

fn count_word_in_coord_4a(grid: &Vec<Vec<char>>, word: &str, row: i32, col: i32) -> i32 {
    static DIRECTIONS: [(i32, i32); 8] = [
        (0, 1),   // up
        (1, 1),   // right up
        (1, 0),   // right
        (0, -1),  // down
        (-1, 1),  // left up
        (-1, -1), // left down
        (-1, 0),  // left
        (1, -1),  // right down
    ];
    let mut count = 0;
    for dir in DIRECTIONS {
        if word_in_dir(grid, word, row, col, dir) {
            count += 1;
        }
    }
    count
}

fn main() {
    let embedded_string = include_str!("../data/input.txt");
    let grid = embedded_string
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut count_4a = 0;
    let mut timer = std::time::Instant::now();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            count_4a += count_word_in_coord_4a(&grid, "XMAS", i as i32, j as i32);
        }
    }
    println!("Time taken: {}ms", timer.elapsed().as_millis());
    println!("Value 4a: {}", count_4a);

    let mut count_4b = 0;
    timer = std::time::Instant::now();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            count_4b += if mas_in_coord(&grid, i as i32, j as i32) {
                1
            } else {
                0
            };
        }
    }
    println!("Time taken: {}ms", timer.elapsed().as_millis());
    println!("Value 4b: {}", count_4b);
}
