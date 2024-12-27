fn is_valid_sequence(numbers: &Vec<i32>) -> bool {
    numbers
        .windows(2)
        .all(|window| window[0] - window[1] >= 1 && window[0] - window[1] <= 3)
        || numbers
            .windows(2)
            .all(|window| window[1] - window[0] >= 1 && window[1] - window[0] <= 3)
}

fn day2b_filter(numbers: &Vec<i32>) -> bool {
    // Check if the original sequence is valid
    if is_valid_sequence(numbers) {
        return true;
    }

    // Try removing each element once
    for i in 0..numbers.len() {
        let mut num_clones = numbers.clone();
        num_clones.remove(i);
        // If removing this element makes the sequence valid
        if is_valid_sequence(&num_clones) {
            return true;
        }
    }

    false
}

fn day2a_filter(numbers: &Vec<i32>) -> bool {
    return is_valid_sequence(numbers);
}

fn main() {
    let embedded_string = include_str!("../data/input.txt");
    let count_2a = embedded_string
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|numbers| day2a_filter(numbers))
        .count();

    println!("Valid count 2a: {}", count_2a);

    let count_2b = embedded_string
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|numbers| day2b_filter(numbers))
        .count();

    println!("Valid count 2b: {}", count_2b);
}
