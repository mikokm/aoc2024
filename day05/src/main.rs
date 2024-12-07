use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    vec,
};

fn main() {
    let _file = read_to_string("input.txt").unwrap();
    let input = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    let input = _file;

    let mut order_rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut orderings: Vec<Vec<i32>> = vec![];
    let mut section = 0;

    fn split_to_vec(separator: char, s: &str) -> Vec<i32> {
        s.split(separator)
            .map(|s| s.parse::<i32>().unwrap())
            .collect()
    }

    for line in input.lines() {
        println!("{:?}", &line);

        if line.is_empty() {
            section = 1;
            continue;
        }

        if section == 0 {
            let parts = split_to_vec('|', line);
            let key = parts[0];
            let val = parts[1];
            order_rules
                .entry(key)
                .and_modify(|s| {
                    s.insert(val);
                })
                .or_insert(HashSet::from([val]));
        }

        if section == 1 {
            let ordering = split_to_vec(',', line);
            orderings.push(ordering);
        }
    }

    println!("Rules: {:?}", order_rules);
    println!("Orderings: {:?}", orderings);

    let mut middle_total = 0;
    let mut fixed_middle_total = 0;

    for ordering in orderings {
        println!("Ordering: {:?}", &ordering);
        let is_valid = is_valid_ordering(&ordering, &order_rules);
        println!("Ordering is {}", if is_valid { "valid" } else { "invalid" });

        if is_valid {
            let middle = ordering.get((ordering.len() / 2) as usize);
            middle_total += middle.unwrap();
        } else {
            let ordering = fix_ordering(&ordering, &order_rules);
            let middle = ordering.get((ordering.len() / 2) as usize);
            fixed_middle_total += middle.unwrap();
        }
    }

    println!("Middle total: {}", middle_total);
    println!("Fixed Middle total: {}", fixed_middle_total);
}

fn is_valid_ordering(ordering: &Vec<i32>, order_rules: &HashMap<i32, HashSet<i32>>) -> bool {
    let mut is_valid = true;
    let mut seen = HashSet::new();
    for page in ordering {
        // println!("Current page: {}", &page);

        seen.insert(page);

        if let Some(dependents) = order_rules.get(&page) {
            // println!("Dependents: {:?}", &dependents);
            for dependent in dependents {
                if seen.contains(dependent) {
                    // println!("Dependent {} in seen!", &dependent);
                    is_valid = false;
                }
            }
        }
    }

    is_valid
}

fn fix_ordering(ordering: &Vec<i32>, order_rules: &HashMap<i32, HashSet<i32>>) -> Vec<i32> {
    println!("Fixing ordering: {:?}", &ordering);

    let mut seen: HashSet<i32> = HashSet::new();
    let mut new_ordering: Vec<i32> = vec![];
    for page in ordering {
        let mut insertion_point = new_ordering.len();
        if let Some(dependents) = order_rules.get(page) {
            for (i, e) in new_ordering.iter().enumerate() {
                if dependents.contains(&e) {
                    insertion_point = std::cmp::min(insertion_point, i);
                }
            }
        }
        new_ordering.insert(insertion_point, *page);
        seen.insert(*page);
    }

    println!("Fixed ordering: {:?}", new_ordering);
    assert!(is_valid_ordering(&new_ordering, &order_rules));

    new_ordering
}
