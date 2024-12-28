fn compress_slots(slots: &Vec<Option<u32>>) -> Vec<u32> {
    let mut res: Vec<u32> = Vec::new();
    let mut left = 0;
    let mut right = slots.len() - 1;

    while left <= right {
        if slots[left].is_some() {
            res.push(slots[left].unwrap());
            left += 1;
            continue;
        }

        debug_assert!(slots[left].is_none());

        if slots[right].is_none() {
            right -= 1;
            continue;
        }

        debug_assert!(slots[left].is_none() && slots[right].is_some());

        res.push(slots[right].unwrap());
        left += 1;
        right -= 1;
    }
    res
}

fn compress_files(slots: &Vec<Option<u32>>) -> Vec<Option<u32>> {
    let mut res: Vec<Option<u32>> = slots.clone();
    let mut right = slots.len() - 1;

    while right > 0 {
        let index = right;
        if res[index].is_none() {
            if right.checked_sub(1).is_none() {
                break;
            }
            right -= 1;
            continue;
        }

        let id = res[index].unwrap();

        let mut size = 0;

        let mut pointer = right;
        while res[pointer].is_some() && res[pointer].unwrap() == id {
            size += 1;

            if pointer.checked_sub(1).is_none() {
                break;
            }
            pointer -= 1;
        }

        let pos_opt = res
            .windows(size)
            .position(|window| window.iter().all(|item| item.is_none()));

        if pos_opt.is_none() || pos_opt.unwrap() > right {
            if right.checked_sub(size).is_none() {
                break;
            }
            right -= size;
            continue;
        }

        let start_1 = pos_opt.unwrap();

        for i in 0..size {
            res.swap(start_1 + i, right - i);
        }

        if right.checked_sub(size).is_none() {
            break;
        }

        right -= size;
    }

    res
}

fn main() {
    let input = include_str!("../data/input.txt");
    let blocks = input
        .chars()
        .into_iter()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<u32>>();

    let mut id: u32 = 0;
    let mut slots: Vec<Option<u32>> = Vec::new();
    for (i, &num) in blocks.iter().enumerate() {
        if i % 2 == 0 {
            for _ in 0..num {
                slots.push(Some(id));
            }
        } else {
            for _ in 0..num {
                slots.push(None);
            }
            id += 1;
        }
    }

    let mut checksum_6a: u64 = 0;

    for (i, num) in compress_slots(&slots).into_iter().enumerate() {
        checksum_6a += i as u64 * num as u64;
    }

    let mut checksum_6b: u64 = 0;

    for (i, num) in compress_files(&slots).into_iter().enumerate() {
        if num.is_none() {
            continue;
        }
        checksum_6b += i as u64 * num.unwrap() as u64;
    }

    println!("9a checksum is: {}", checksum_6a);
    println!("9b checksum is: {}", checksum_6b);
}
