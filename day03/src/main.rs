use regex::Regex;
use std::fs::read_to_string;

fn parse_line(s: &str, enabled: &mut bool) -> Vec<(i32, i32)> {
    println!("Input: {}", &s);
    let re = Regex::new(r"(?<func>mul|do|don't)\((?<arg>(\d*,\d*)*)\)").unwrap();
    let mut result = vec![];

    for captures in re.captures_iter(s) {
        // println!("Group: {:?}", captures);
        let func = captures.name("func");
        let arg = captures.name("arg");

        println!(
            "{} func: {}({})",
            if *enabled { "*" } else { " " },
            func.unwrap().as_str(),
            arg.unwrap().as_str()
        );

        match func.unwrap().as_str() {
            "do" => *enabled = true,
            "don't" => *enabled = false,
            "mul" if *enabled => {
                if let Some(arg) = arg {
                    // println!("mul({})", arg.as_str());
                    let args: Vec<i32> = arg
                        .as_str()
                        .split(",")
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect();
                    assert!(args.len() == 2);
                    result.push((args[0], args[1]));
                }
            }
            _ => (),
        }
    }

    result
}

fn main() {
    let file = read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = file.lines().collect();
    let mut total: i64 = 0;
    let mut enabled: bool = true;
    for line in lines {
        // let line = r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let res = parse_line(line, &mut enabled);
        for r in res {
            // println!("{:?}", &r);
            total += r.0 as i64 * r.1 as i64;
        }
    }
    println!("total: {}", total);
}
