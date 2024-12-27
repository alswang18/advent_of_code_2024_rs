fn main() {
    let embedded_string = include_str!("../data/input.txt");

    // Parse each line's input
    let lines: Vec<&str> = embedded_string.lines().collect();

    let mut abs_total: u32 = 0;
    let mut left_vec: Vec<i32> = Vec::new();
    let mut right_vec: Vec<i32> = Vec::new();

    for (_, line) in lines.iter().enumerate() {
        let parsed = line.split("   ").collect::<Vec<&str>>();
        let left = parsed[0].parse::<i32>().unwrap();
        let right = parsed[1].parse::<i32>().unwrap();
        left_vec.push(left);
        right_vec.push(right);
    }
    // Need to sort the vectors either way.
    left_vec.sort();
    right_vec.sort();

    for (i, left) in left_vec.iter().enumerate() {
        abs_total += (left - right_vec[i]).abs() as u32;
    }

    println!("Total Difference: {}", abs_total);

    // Running Count is used to find the running count.
    let mut count_total: u32 = 0;
    let mut running_count: u32 = 0;
    let mut right_index: usize = 0;
    for (_, left) in left_vec.iter().enumerate() {
        while right_index < right_vec.len() && *left >= right_vec[right_index] {
            if *left == right_vec[right_index] {
                running_count += 1;
            }
            right_index += 1;
        }

        count_total += running_count * (*left as u32);
        running_count = 0;
    }
    println!("Running Count: {}", count_total);
}
