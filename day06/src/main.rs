use std::{collections::HashSet, fs::read_to_string, vec};

type Graph = Vec<Vec<char>>;

fn find_char(g: &Graph, needle: char) -> Vec<(i32, i32)> {
    let mut xs = vec![];

    for (y, row) in g.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == needle {
                xs.push((x as i32, y as i32));
            }
        }
    }
    xs
}

fn in_graph(g: &Graph, x: i32, y: i32) -> bool {
    if y >= g.len() as i32 || y < 0 || x >= g[0].len() as i32 || x < 0 {
        return false;
    }

    true
}

fn solve(graph: &mut Graph) -> (HashSet<(i32, i32)>, bool) {
    let xs = find_char(&graph, '^');
    let directions = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut pos = xs[0];
    let mut dir_index = 0;
    let mut unique_positions = HashSet::from([pos]);
    let mut previous_edges: HashSet<((i32, i32), (i32, i32))> = HashSet::new();

    loop {
        let (oob, last_pos, positions) = walk_forward(graph, pos, directions[dir_index]);
        for position in &positions {
            unique_positions.insert(*position);
        }

        if oob {
            return (unique_positions, false);
        }

        if !previous_edges.insert((pos, last_pos)) {
            println!("Cycle detected!");
            return (unique_positions, true);
        }

        pos = last_pos;
        dir_index = (dir_index + 1) % directions.len();
    }
}

fn main() {
    let _file = read_to_string("input.txt").unwrap();
    let input = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    let input = _file;
    let mut graph: Graph = input.lines().map(|s| s.chars().collect()).collect();

    let print_graph = |g: &Graph| {
        for row in g {
            for c in row {
                print!("{}", c);
            }
            println!();
        }
    };

    print_graph(&graph);
    let mut cycle_pos = vec![];

    for y in 0..=graph.len() - 1 {
        for x in 0..=graph[0].len() - 1 {
            if graph[y][x] == '#' || graph[y][x] == '^' {
                continue;
            }

            let mut new_graph = graph.clone();
            new_graph[y][x] = '#';
            let (_, cycles) = solve(&mut new_graph);
            // print_graph(&new_graph);
            // println!("{:?} : {}", (x, y), cycles);
            if cycles {
                cycle_pos.push((x, y));
            }
        }
    }

    println!("cycles: {:?}, len: {}", cycle_pos, cycle_pos.len());
    for (x, y) in cycle_pos {
        graph[y][x] = 'O';
    }

    print_graph(&graph);
}

fn walk_forward(
    g: &mut Graph,
    start_pos: (i32, i32),
    delta: (i32, i32),
) -> (bool, (i32, i32), Vec<(i32, i32)>) {
    // println!("Walking dir = {:?}, start_pos = {:?}", &delta, &start_pos);
    let mut positions = Vec::new();
    let mut pos = start_pos;

    loop {
        let next = (pos.0 + delta.0, pos.1 + delta.1);
        // println!("Next {:?}", &next);
        if !in_graph(&g, next.0, next.1) {
            // println!("oob {:?}", &pos);
            return (true, pos, positions);
        }

        let c = g[next.1 as usize][next.0 as usize];
        // println!("Visited {:?} ({})", &pos, c);

        if c == '#' {
            // println!("Hit wall");
            return (false, pos, positions);
        }

        positions.push(next);
        pos = next;
    }

    unreachable!("what");
}
