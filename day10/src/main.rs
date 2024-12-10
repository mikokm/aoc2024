use std::{collections::HashSet, fs::read_to_string};

type Graph = Vec<Vec<u32>>;

fn main() {
    let _file = read_to_string("input.txt").unwrap();
    let input = r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
    let input = _file;

    // println!("{}", input);
    let graph: Graph = input
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    for row in &graph {
        println!("{:?}", row);
    }

    let zeroes = find_zeros(&graph);
    println!("Found {} zeroes {:?}", zeroes.len(), zeroes);

    let mut routes = 0;
    for zero in zeroes {
        let ends = find_routes(&graph, zero, 0);
        routes += ends.len();
    }
    println!("Routes: {:?}", routes);
}

fn find_zeros(g: &Graph) -> Vec<(i32, i32)> {
    let mut result = vec![];
    for (y, row) in g.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 0 {
                result.push((x as i32, y as i32));
            }
        }
    }

    result
}

fn is_in_graph(g: &Graph, x: i32, y: i32) -> bool {
    y >= 0 && g.len() > y as usize && x >= 0 && g[0].len() > x as usize
}

fn find_neighbour(g: &Graph, x: i32, y: i32, n: u32) -> Vec<(i32, i32)> {
    let directions = [(0, -1), (0, 1), (1, 0), (-1, 0)];
    let neighbours = directions.map(|(dx, dy)| (x + dx, y + dy)).to_vec();
    neighbours
        .iter()
        .filter(|(x, y)| is_in_graph(&g, *x, *y) && g[*y as usize][*x as usize] == n)
        .cloned()
        .collect()
}

fn find_routes(g: &Graph, start: (i32, i32), current_height: u32) -> HashSet<(i32, i32)> {
    assert!(is_in_graph(&g, start.0, start.1));

    let height = g[start.1 as usize][start.0 as usize];

    if height != current_height {
        return HashSet::new();
    }

    if current_height == 9 {
        return HashSet::from([start]);
    }

    let next_height = current_height + 1;
    let neighbours = find_neighbour(&g, start.0, start.1, next_height);

    let mut all_ends = HashSet::new();
    for neighbour in neighbours {
        let ends = find_routes(&g, neighbour, next_height);
        all_ends.extend(ends);
    }

    all_ends
}
