use regex::Regex;

fn day3a_value(string: &str) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("Invalid regex");

    let mut sum = 0;
    for (_, [a, b]) in re.captures_iter(string).map(|cap| cap.extract()) {
        let left = a.parse::<u32>().unwrap();
        let right = b.parse::<u32>().unwrap();
        sum += left * right;
    }

    sum
}

fn day3b_value(string: &str) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").expect("Invalid regex");

    let mut sum = 0;
    let mut add_up = true;
    for m in re.captures_iter(string) {
        let instruction = m.get(0).unwrap().as_str();

        match instruction {
            "do()" => add_up = true,
            "don't()" => add_up = false,
            _ => {
                if add_up {
                    let left: u32 = m.get(1).unwrap().as_str().parse::<u32>().unwrap();
                    let right: u32 = m.get(2).unwrap().as_str().parse::<u32>().unwrap();
                    sum += left * right;
                }
            }
        }
    }
    sum
}

fn main() {
    let embedded_string = include_str!("../data/input.txt");
    // Time to parse the input.
    let mut timer = std::time::Instant::now();
    let value3_a = day3a_value(embedded_string);
    println!("Value 3a: {}", value3_a);
    println!("Time taken: {}ms", timer.elapsed().as_millis());

    timer = std::time::Instant::now();
    let value3_b = day3b_value(embedded_string);
    println!("Value 3b: {}", value3_b);
    println!("Time taken: {}ms", timer.elapsed().as_millis());
}
