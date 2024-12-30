use nalgebra::{Matrix2, Vector2};
use regex::Regex;

#[derive(Debug)]
struct ClawMachine {
    button_a: (f64, f64),
    button_b: (f64, f64),
    prize: (f64, f64),
}

fn is_pos_whole_number(x: f64) -> bool {
    x >= 0.0 && (x - x.round()).abs() < 1e-4
}

impl ClawMachine {
    fn solve_prize(&self) -> Option<u64> {
        let a = Matrix2::new(
            self.button_a.0 as f64,
            self.button_b.0 as f64,
            self.button_a.1 as f64,
            self.button_b.1 as f64,
        );

        let b = Vector2::new(self.prize.0 as f64, self.prize.1 as f64);
        let lu = a.lu();
        match lu.solve(&b) {
            Some(solution) => {
                if is_pos_whole_number(solution[0]) && is_pos_whole_number(solution[1]) {
                    debug_assert!(
                        ((solution[0].round() * self.button_a.0
                            + solution[1].round() * self.button_b.0)
                            - self.prize.0)
                            .abs()
                            < 1e-4
                    );
                    debug_assert!(
                        (solution[0].round() * self.button_a.1
                            + solution[1].round() * self.button_b.1
                            - self.prize.1)
                            .abs()
                            < 1e-4
                    );
                    return Some(solution[0].round() as u64 * 3 + solution[1].round() as u64);
                }
            }
            None => println!("No solution exists."),
        }
        None
    }
}
fn main() {
    let button_re = Regex::new(r"[XY]\+(\d+)").unwrap();
    let score_re = Regex::new(r"[XY]\=(\d+)").unwrap();
    let input = include_str!("../data/input.txt");
    let lines = input.lines().collect::<Vec<&str>>();

    let mut machines_a: Vec<ClawMachine> = Vec::new();
    let mut machines_b: Vec<ClawMachine> = Vec::new();
    for i in (0..lines.len()).step_by(4) {
        let line_a = button_re
            .captures_iter(lines[i])
            .filter_map(|caps| caps.get(1))
            .filter_map(|c| c.as_str().parse::<f64>().ok())
            .collect::<Vec<f64>>();
        debug_assert!(line_a.len() == 2);
        let line_b = button_re
            .captures_iter(lines[i + 1])
            .filter_map(|caps| caps.get(1))
            .filter_map(|c| c.as_str().parse::<f64>().ok())
            .collect::<Vec<f64>>();

        debug_assert!(line_b.len() == 2);

        let line_prize = score_re
            .captures_iter(lines[i + 2])
            .filter_map(|caps| caps.get(1))
            .filter_map(|c| c.as_str().parse::<f64>().ok())
            .collect::<Vec<f64>>();

        debug_assert!(line_prize.len() == 2);

        machines_a.push(ClawMachine {
            button_a: (line_a[0], line_a[1]),
            button_b: (line_b[0], line_b[1]),
            prize: (line_prize[0], line_prize[1]),
        });
        machines_b.push(ClawMachine {
            button_a: (line_a[0], line_a[1]),
            button_b: (line_b[0], line_b[1]),
            prize: (
                line_prize[0] + 10000000000000.0,
                line_prize[1] + 10000000000000.0,
            ),
        });
    }

    let mut timer = std::time::Instant::now();
    let score_a = machines_a
        .into_iter()
        .filter_map(|cm| cm.solve_prize())
        .sum::<u64>();
    println!("Score of all available claw machines in A is: {}", score_a);
    println!("Time in {} us", timer.elapsed().as_micros());
    timer = std::time::Instant::now();

    let score_b = machines_b
        .into_iter()
        .filter_map(|cm| cm.solve_prize())
        .sum::<u64>();
    println!("Score of all available claw machines in B is: {}", score_b);
    println!("Time in {} us", timer.elapsed().as_micros());
}
