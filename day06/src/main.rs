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

    let xs = find_char(&graph, '^');
    let directions = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut pos = xs[0];
    let mut dir_index = 0;
    let mut unique_positions = HashSet::from([pos]);

    loop {
        let (oob, last_pos, positions) = walk_forward(&mut graph, pos, directions[dir_index]);
        for position in &positions {
            unique_positions.insert(*position);
        }

        if oob {
            break;
        }

        pos = last_pos;
        dir_index = (dir_index + 1) % directions.len();
    }

    for (x, y) in &unique_positions {
        assert!(in_graph(&graph, *x, *y));
        graph[*y as usize][*x as usize] = 'X';
    }
    println!("Result: {}", unique_positions.len());
    print_graph(&graph);
}

fn walk_forward(
    g: &mut Graph,
    start_pos: (i32, i32),
    delta: (i32, i32),
) -> (bool, (i32, i32), Vec<(i32, i32)>) {
    println!("Walking dir = {:?}, start_pos = {:?}", &delta, &start_pos);
    let mut positions = Vec::new();
    let mut pos = start_pos;

    loop {
        let next = (pos.0 + delta.0, pos.1 + delta.1);
        println!("Next {:?}", &next);
        if !in_graph(&g, next.0, next.1) {
            println!("oob {:?}", &pos);
            return (true, pos, positions);
        }

        let c = g[next.1 as usize][next.0 as usize];
        println!("Visited {:?} ({})", &pos, c);

        if c == '#' {
            println!("Hit wall");
            return (false, pos, positions);
        }

        positions.push(next);
        pos = next;
    }

    unreachable!("what");
}
