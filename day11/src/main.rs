use std::collections::HashMap;

struct BaseStone {
    memo: HashMap<(u32, u64), u64>,
}

impl BaseStone {
    fn dfs(&mut self, iters: u32, stone: u64) -> u64 {
        if iters == 0 {
            return 1;
        }
        if self.memo.contains_key(&(iters, stone)) {
            return *self.memo.get(&(iters, stone)).unwrap();
        }

        let mut res = 0;

        if stone == 0 {
            res += self.dfs(iters - 1, 1);
            self.memo.insert((iters, stone), res);
            return res;
        }

        // Always handle the zero case for unsigned types before calling ilog.
        let digits = stone.ilog10() + 1;
        if digits % 2 == 0 {
            let mut dup = stone;
            let half = digits / 2;

            for _ in 0..half {
                dup = dup / 10;
            }

            let left_half = dup;

            let right_half = stone - (left_half * (10 as u64).pow(half));

            res += self.dfs(iters - 1, left_half);
            res += self.dfs(iters - 1, right_half);
            self.memo.insert((iters, stone), res);
            return res;
        }

        res += self.dfs(iters - 1, stone * 2024);
        self.memo.insert((iters, stone), res);
        res
    }
}

fn main() {
    let input_text = include_str!("../data/input.txt");

    let stones = input_text
        .split(' ')
        .filter_map(|num| num.parse::<u64>().ok())
        .collect::<Vec<u64>>();

    let mut count_a = 0;
    for &stone in stones.iter() {
        let mut base_stone = BaseStone {
            memo: HashMap::default(),
        };
        count_a += base_stone.dfs(10000, stone);
    }

    // println!("{:?}", stones);
    println!("Now I have {} stones.", count_a);
}
