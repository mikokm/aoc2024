use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

type Graph = Vec<Vec<char>>;

fn in_graph(g: &Graph, x: i32, y: i32) -> bool {
    if y >= g.len() as i32 || y < 0 || x >= g[0].len() as i32 || x < 0 {
        return false;
    }

    true
}

fn print_graph(g: &Graph) {
    for row in g {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

fn start_fill(g: &Graph, x: i32, y: i32, visited: &mut HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    println!("x: {}, y: {}", x, y);
    if !in_graph(g, x, y) {
        return HashSet::new();
    }

    if visited.contains(&(x, y)) {
        return HashSet::new();
    }

    visited.insert((x, y));

    let c = g[y as usize][x as usize];

    let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let neighbours: Vec<(i32, i32)> = directions
        .iter()
        .map(|(dx, dy)| (x + dx, y + dy))
        .filter(|(nx, ny)| in_graph(g, *nx, *ny))
        .collect();

    let mut area = HashSet::new();
    area.insert((x, y));

    for (nx, ny) in neighbours {
        println!("nx: {}, ny: {}", nx, ny);
        if g[ny as usize][nx as usize] == c {
            let sub_area = start_fill(g, nx, ny, visited);
            area.extend(sub_area);
        }
    }

    area
}

fn find_external_edges(g: &Graph, c: char, x: i32, y: i32) -> Vec<(i32, i32)> {
    let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut edges = vec![];
    for (dx, dy) in directions {
        let nx = x + dx;
        let ny = y + dy;
        if !in_graph(g, nx, ny) || g[ny as usize][nx as usize] != c {
            edges.push((nx, ny));
        }
    }

    edges
}

fn find_areas(g: &Graph) {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut areas: Vec<(char, HashSet<(i32, i32)>)> = vec![];

    for y in 0..g.len() {
        for x in 0..g[0].len() {
            let c = g[y][x];
            let area = start_fill(g, x as i32, y as i32, &mut visited);
            println!("c: {}, Area: {:?}", c, area);
            if area.len() > 0 {
                areas.push((c, area));
            }
        }
    }

    let mut cost = 0;
    let mut new_cost = 0;
    for (k, v) in areas {
        println!("Region: {:?}, {:?}", k, v);
        let circumference = v
            .iter()
            .map(|(x, y)| find_external_edges(g, k, *x, *y).len())
            .sum::<usize>();
        println!("area: {}, circumference: {}", v.len(), circumference);
        cost += circumference * v.len();

        let external_edges = v
            .iter()
            .flat_map(|(x, y)| find_external_edges(g, k, *x, *y))
            .collect::<Vec<_>>();

        // group external edges by x coordinate and figure out gap count
        let mut x_groups: HashMap<i32, Vec<i32>> = HashMap::new();
        for (x, y) in &external_edges {
            x_groups.entry(*x).or_insert(vec![]).push(*y);
        }

        let mut total_gaps = 0;

        for (x, ys) in &x_groups {
            println!("x: {}, ys: {:?}", x, ys);
            let mut sorted_ys = ys.clone();
            sorted_ys.sort();
            let mut gaps = 0;
            for i in 0..sorted_ys.len() - 1 {
                if sorted_ys[i + 1] - sorted_ys[i] > 1 {
                    gaps += 1;
                }
            }

            total_gaps += gaps;
            println!("x: {}, ys: {:?}, gaps: {}", x, sorted_ys, gaps);
        }
        // group external edges by y coordinate and figure out gap count
        let mut y_groups: HashMap<i32, Vec<i32>> = HashMap::new();
        for (x, y) in &external_edges {
            y_groups.entry(*y).or_insert(vec![]).push(*x);
        }

        for (y, xs) in &y_groups {
            let mut sorted_xs = xs.clone();
            sorted_xs.sort();
            let mut gaps = 0;
            for i in 0..sorted_xs.len() - 1 {
                if sorted_xs[i + 1] - sorted_xs[i] > 1 {
                    gaps += 1;
                }
            }

            total_gaps += gaps;
            println!("y: {}, xs: {:?}, gaps: {}", y, sorted_xs, gaps);
        }

        println!(
            "External edges: {:?}, count: {}, total gaps: {}",
            external_edges,
            external_edges.len(),
            total_gaps
        );

        // find edges that belong to a same side
    }

    println!("Cost: {}", cost);
    println!("New cost: {}", new_cost);
}

fn main() {
    let _file = read_to_string("input.txt").unwrap();
    let _test1 = r"AAAA
BBCD
BBCC
EEEC";
    let _test2 = r"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
    let _test3 = r"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
    let input = _test1;
    let graph: Graph = input.lines().map(|s| s.chars().collect()).collect();
    print_graph(&graph);
    find_areas(&graph);
}
