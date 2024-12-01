use std::{collections::HashMap, fs::read_to_string};

fn str_to_pair(s: &str) -> (i32, i32) {
    let split = s.split_whitespace();
    let parts: Vec<i32> = split.map(|x| x.parse::<i32>().unwrap()).collect();
    (parts[0], parts[1])
}

fn parse_pairs(content: String) -> (Vec<i32>, Vec<i32>) {
    content.lines().map(|s| str_to_pair(s)).unzip()
}

fn main() {
    let file = read_to_string("input.txt").unwrap();
    let (mut v1, mut v2) = parse_pairs(file);
    v1.sort();
    v2.sort();

    let pairs = v1.iter().zip(v2.iter());
    let diff = pairs.fold(0, |acc, x| acc + (x.1 - x.0).abs());
    dbg!("diff: {}", diff);

    let mut map = HashMap::<i32, i32>::new();
    for x in &v2 {
        map.entry(x.clone()).and_modify(|c| *c += 1).or_insert(1);
    }
    dbg!("map {}", &map);

    let sim = v1.iter().fold(0, |acc, x| {
        let c = map.get(x).unwrap_or(&0);
        acc + (*x * *c)
    });
    dbg!("{}", sim);
}
