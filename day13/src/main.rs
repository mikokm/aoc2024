use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

#[derive(Debug)]
struct Machine {
    buttons: HashMap<char, (i32, i32)>,
    prize: (i32, i32),
}

fn main() {
    let _file = read_to_string("input.txt").unwrap();
    let _test1 = r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
    // let input = _test1;
    let input = _file;

    let input = input.lines().filter(|s| s.len() > 0).collect::<Vec<_>>();
    let mut machines = Vec::new();
    for m in input.chunks_exact(3) {
        println!("{:?}", m);
        let mut buttons = HashMap::new();
        let re = regex::Regex::new(r"Button ([A-Z]): X\+([0-9]+), Y\+([0-9]+)").unwrap();

        for line in [m[0], m[1]].iter() {
            let caps = re.captures(line).unwrap();
            let button = caps[1].chars().next().unwrap();
            let x = caps[2].parse::<i32>().unwrap();
            let y = caps[3].parse::<i32>().unwrap();
            buttons.insert(button, (x, y));
        }

        let re = regex::Regex::new(r"Prize: X=([0-9]+), Y=([0-9]+)").unwrap();
        let caps = re.captures(m[2]).unwrap();
        let prize_x = caps[1].parse::<i32>().unwrap();
        let prize_y = caps[2].parse::<i32>().unwrap();
        let prize = (prize_x, prize_y);

        let machine = Machine { buttons, prize };
        machines.push(machine);
    }

    println!("{:?}", machines);
    let mut total_cost = 0;
    for machine in machines {
        let (steps, cost) = solve(&machine);
        println!("Steps: {:?}, Cost: {:?}", steps, cost);
        total_cost += cost.unwrap_or(0);
    }

    println!("Total cost: {}", total_cost);
}

fn solve(machine: &Machine) -> (Option<i32>, Option<i32>) {
    println!("Solving {:?}", machine);
    let mut visited = HashSet::new();
    let mut queue = Vec::new();
    queue.push((0, 0, 0, 0, 0));

    let mut best_steps = None;
    let mut best_cost = None;
    while let Some((x, y, steps, a, b)) = queue.pop() {
        if visited.contains(&(x, y)) {
            continue;
        }

        if a > 100 || b > 100 {
            continue;
        }

        visited.insert((x, y));

        if (x, y) == machine.prize {
            println!("Found prize in {} steps", steps);
            let cost = a * 3 + b;
            best_cost = best_cost.map(|c: i32| c.min(cost)).or(Some(cost));
            best_steps = best_steps.map(|s: i32| s.min(steps)).or(Some(steps));
        }

        for (c, (dx, dy)) in machine.buttons.iter() {
            let nx = x + dx;
            let ny = y + dy;
            queue.push((
                nx,
                ny,
                steps + 1,
                a + if c == &'A' { 1 } else { 0 },
                b + if c == &'B' { 1 } else { 0 },
            ));
        }
    }

    (best_steps, best_cost)
}
