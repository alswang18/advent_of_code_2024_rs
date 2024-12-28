#[derive(Clone)]
struct Solver {
    target: u64,
    values: Vec<u64>,
    success: usize,
}

impl Solver {
    fn dfs_7a(&mut self, i: usize, cur: u64) {
        if i == self.values.len() {
            self.success += if cur == self.target { 1 } else { 0 };
            return;
        }

        if i == 0 {
            self.dfs_7a(i + 1, self.values[0]);
            return;
        }

        self.dfs_7a(i + 1, cur + self.values[i]);
        self.dfs_7a(i + 1, cur * self.values[i]);
    }

    fn dfs_7b(&mut self, i: usize, cur: u64) {
        if i == self.values.len() {
            self.success += if cur == self.target { 1 } else { 0 };
            return;
        }

        if i == 0 {
            self.dfs_7b(i + 1, self.values[0]);
            return;
        }

        self.dfs_7b(i + 1, cur + self.values[i]);
        self.dfs_7b(i + 1, cur * self.values[i]);

        let mut digits = 0;
        let mut v = self.values[i];
        while v > 0 {
            v /= 10;
            digits += 1;
        }

        self.dfs_7b(i + 1, (cur * u64::pow(10, digits)) + self.values[i]);
    }
}

fn main() {
    let input_string = include_str!("../data/input.txt");

    let targets = input_string
        .lines()
        .filter_map(|line| {
            line.split(':').collect::<Vec<&str>>()[0]
                .parse::<u64>()
                .ok()
        })
        .collect::<Vec<u64>>();

    let values: Vec<Vec<u64>> = input_string
        .lines()
        .map(|line| {
            line.split(':').collect::<Vec<&str>>()[1]
                .split(' ')
                .filter_map(|num| num.parse::<u64>().ok())
                .collect()
        })
        .collect();

    debug_assert!(targets.len() == values.len());

    let mut timer = std::time::Instant::now();
    let mut count_7a = 0;
    for i in 0..targets.len() {
        let mut solver_7a: Solver = Solver {
            target: targets[i],
            values: values[i].clone(),
            success: 0,
        };

        solver_7a.dfs_7a(0, 0);
        if solver_7a.success > 0 {
            count_7a += solver_7a.target
        }
    }
    println!("Count 7a: {}", count_7a);
    println!("Timer is {}ms", timer.elapsed().as_millis());
    let mut count_7b = 0;
    timer = std::time::Instant::now();
    for i in 0..targets.len() {
        let mut solver_7b: Solver = Solver {
            target: targets[i],
            values: values[i].clone(),
            success: 0,
        };
        solver_7b.dfs_7b(0, 0);
        if solver_7b.success > 0 {
            count_7b += solver_7b.target
        }
    }
    println!("Count 7b: {}", count_7b);
    println!("Timer is {}ms", timer.elapsed().as_millis());
}
