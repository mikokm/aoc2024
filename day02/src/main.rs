use std::fs::read_to_string;

fn str_to_vec(s: &str) -> Vec<i32> {
    let split = s.split_whitespace();
    split.map(|x| x.parse::<i32>().unwrap()).collect()
}

fn parse_file(content: String) -> Vec<Vec<i32>> {
    content.lines().map(|s| str_to_vec(s)).collect()
}

fn is_safe(input: &Vec<i32>) -> bool {
    let mut windows = input.windows(2);
    let dir = windows.clone().take(1).fold(0, |acc, x| acc + x[0] - x[1]);
    let is_safe = windows.all(|x| {
        let diff = x[0] - x[1];
        diff.abs() >= 1 && diff.abs() <= 3 && diff.signum() == dir.signum()
    });
    println!("dir: {:?}", dir);

    is_safe
}

fn can_be_safe(input: &Vec<i32>) -> bool {
    for i in 0..input.len() {
        // Soo many copies
        let mut new_input = input.clone();
        new_input.remove(i);
        if is_safe(&new_input) {
            return true;
        }
    }
    false
}

fn main() {
    let file = read_to_string("input.txt").unwrap();
    let reports = parse_file(file);

    let mut safe_reports = 0;
    for report in reports {
        if is_safe(&report) || can_be_safe(&report) {
            safe_reports += 1
        }
    }

    println!("Safe reports: {}", safe_reports);
}
