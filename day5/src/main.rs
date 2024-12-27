use std::collections::{HashMap, HashSet};

fn is_line_valid(line: &str, graph: &HashMap<i32, HashSet<i32>>) -> bool {
    let nums = line
        .split(",")
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mut past: Vec<i32> = Vec::new();
    for n in nums.iter().rev() {
        if graph.contains_key(n) {
            let set: &HashSet<i32> = graph.get(n).unwrap();
            for entry in set.iter() {
                if past.contains(entry) {
                    return false;
                }
            }
        }
        past.push(*n);
    }
    true
}

fn are_nums_valid(nums: &Vec<i32>, graph: &HashMap<i32, HashSet<i32>>) -> bool {
    let mut past: Vec<i32> = Vec::new();
    for n in nums.iter().rev() {
        if graph.contains_key(n) {
            let set: &HashSet<i32> = graph.get(n).unwrap();
            for entry in set.iter() {
                if past.contains(entry) {
                    return false;
                }
            }
        }
        past.push(*n);
    }
    true
}

fn find_first_invalid_index_paid(
    nums: &Vec<i32>,
    graph: &HashMap<i32, HashSet<i32>>,
) -> Option<(usize, usize)> {
    let mut past: Vec<i32> = Vec::new();
    for (i, n) in nums.iter().rev().enumerate() {
        if graph.contains_key(n) {
            let set: &HashSet<i32> = graph.get(n).unwrap();
            for entry in set.iter() {
                for j in 0..past.len() {
                    if past[j] == *entry {
                        return Some((nums.len() - 1 - i, nums.len() - 1 - j));
                    }
                }
            }
        }
        past.push(*n);
    }
    None
}

fn fix_invalid_line(line: &str, graph: &HashMap<i32, HashSet<i32>>) -> Vec<i32> {
    let mut nums: Vec<i32> = line
        .split(",")
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .iter()
        .rev()
        .map(|&x| x)
        .collect();
    while !are_nums_valid(&nums, &graph) {
        let pair = find_first_invalid_index_paid(&nums, &graph);
        assert!(pair.is_some());

        nums.swap(pair.unwrap().0, pair.unwrap().1);
    }
    nums
}

fn get_median_number_line(line: &str) -> i32 {
    let nums = line
        .split(",")
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    assert!(nums.len() > 0);
    assert!(nums.len() % 2 == 1);

    nums[(nums.len() / 2) as usize]
}

fn get_median_number_vec(nums: &Vec<i32>) -> i32 {
    assert!(nums.len() > 0);
    assert!(nums.len() % 2 == 1);

    nums[(nums.len() / 2) as usize]
}

fn main() {
    let dependency_strings = include_str!("../data/dependency_input.txt");
    let line_strings = include_str!("../data/lines.txt");

    let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();

    let mut timer = std::time::Instant::now();
    for line in dependency_strings.lines() {
        let pair: Vec<&str> = line.split("|").collect();
        assert!(pair.len() == 2);
        let key = pair[1].parse::<i32>().expect("Expects to be number");
        let dependency = pair[0].parse::<i32>().expect("Expects to be number");
        if !graph.contains_key(&key) {
            graph.insert(key, HashSet::new());
        }

        if let Some(set) = graph.get_mut(&key) {
            set.insert(dependency);
        }
    }

    let count_5a: i32 = line_strings
        .lines()
        .filter(|line| is_line_valid(line, &graph))
        .map(|line| get_median_number_line(line))
        .sum();

    println!("Count 5a: {}", count_5a);
    println!("Time taken: {}ms", timer.elapsed().as_millis());

    timer = std::time::Instant::now();

    let count_5b: i32 = line_strings
        .lines()
        .filter(|line| !is_line_valid(line, &graph))
        .map(|l| get_median_number_vec(&fix_invalid_line(l, &graph)))
        .sum();

    println!("Count 5b: {}", count_5b);
    println!("Time taken: {}ms", timer.elapsed().as_millis());
}
