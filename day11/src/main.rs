use std::{
    collections::{vec_deque, HashMap, HashSet, VecDeque},
    fs::read_to_string,
};

fn main() {
    let _file = read_to_string("input.txt").unwrap();
    let input = r"0 1 10 99 999";
    let input = _file;
    let stones: Vec<i64> = input
        .split(' ')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    println!("stones: {:?}", stones);

    let mut new_stones: HashMap<i64, i64> = HashMap::new();
    for stone in stones {
        new_stones.entry(stone).and_modify(|c| *c += 1).or_insert(1);
    }

    for iteration in 0..75 {
        new_stones = blink_map(new_stones);
        println!(
            "iteration: {}, new stones len: {:?}",
            iteration,
            new_stones.len(),
        );
    }

    let sum: i64 = new_stones.iter().map(|(_, count)| count).sum();
    println!("Sum: {}", sum);
}

fn blink(stones: &Vec<i64>) -> Vec<i64> {
    let mut new_stones = vec![];

    for stone in stones {
        if *stone == 0 {
            new_stones.push(1);
            continue;
        }
        let s = stone.to_string();
        if s.len() % 2 == 0 {
            let (left, right) = s.split_at(s.len() / 2);
            // println!("left: {}, right: {}", left, right);
            let split_stones = [left, right].map(|s| s.parse::<i64>().unwrap());
            new_stones.extend(split_stones);
        } else {
            new_stones.push(2024 * stone);
        }
    }

    new_stones
}

fn blink_map(stones: HashMap<i64, i64>) -> HashMap<i64, i64> {
    let mut new_stones = HashMap::new();

    for (stone, count) in stones {
        let step = blink(&vec![stone]);
        for s in step {
            new_stones
                .entry(s)
                .and_modify(|c| *c += count)
                .or_insert(count);
        }
    }

    new_stones
}
