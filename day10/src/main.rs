use std::{fs::read_to_string, vec};

fn main() {
    let _file = read_to_string("input.txt").unwrap();
    let input = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    let input = _file;
    // println!("{}", input);
    let equations: Vec<(i64, Vec<i64>)> = input
        .lines()
        .map(|s| {
            let (first, rest) = s.split_once(':').unwrap();
            // println!("{}:{}", first, rest);
            (
                first.parse::<i64>().unwrap(),
                rest.trim_start()
                    .split(' ')
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect(),
            )
        })
        .collect();
    println!("{:?}", equations);

    let mut result = 0;
    for equation in equations {
        if is_valid_equation(&equation) {
            result += equation.0;
        }
    }

    println!("Result: {}", result);
}

#[derive(Debug, Clone)]
enum Symbol {
    Add,
    Mul,
    Con,
    Num(i64),
}

fn is_valid_equation((target, terms): &(i64, Vec<i64>)) -> bool {
    // println!("Target: {}, terms: {:?}", target, &terms);
    let sign_len_to_gen = terms.len() - 1;
    let mut signs: Vec<Vec<Symbol>> = vec![];
    signs.push(vec![Symbol::Add]);
    signs.push(vec![Symbol::Mul]);
    signs.push(vec![Symbol::Con]);

    for i in 1..sign_len_to_gen {
        // Collect new combinations in a single expression
        let new_lists: Vec<_> = signs
            .iter()
            .filter(|list| list.len() == i) // Filter lists of the desired length
            .flat_map(|list| {
                // Generate two new lists: one with Add, one with Mul
                [Symbol::Add, Symbol::Mul, Symbol::Con]
                    .into_iter()
                    .map(move |symbol| {
                        let mut new_list = list.clone();
                        new_list.push(symbol);
                        new_list
                    })
            })
            .collect();

        signs.extend(new_lists); // Append all new lists to signs
    }

    // println!("signs: {:?}", signs);
    for ops in signs.iter().filter(|list| list.len() == sign_len_to_gen) {
        let equation: Vec<Symbol> = terms
            .iter()
            .cloned()
            .enumerate()
            .flat_map(|(i, term)| {
                if i < ops.len() {
                    vec![Symbol::Num(term), ops[i].clone()]
                } else {
                    vec![Symbol::Num(term)]
                }
            })
            .collect();
        // println!("eq: {:?}", equation);

        let result: (i64, Option<Symbol>) =
            equation.iter().fold((0, None), |(res, op), cur| match cur {
                Symbol::Add => return (res, Some(Symbol::Add)),
                Symbol::Mul => return (res, Some(Symbol::Mul)),
                Symbol::Con => return (res, Some(Symbol::Con)),
                Symbol::Num(n) => match op {
                    Some(Symbol::Add) => return (res + n, None),
                    Some(Symbol::Mul) => return (res * n, None),
                    Some(Symbol::Con) => {
                        let s = res.to_string() + &n.to_string();
                        return (s.parse::<i64>().unwrap(), None);
                    }
                    None => return (res + n, None),
                    _ => panic!("eep"),
                },
            });

        if result.0 == *target {
            return true;
        }

        // println!("{:?}", result);
    }

    false
}
