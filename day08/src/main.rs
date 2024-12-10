use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    vec,
};

type Graph = Vec<Vec<char>>;

fn find_antennas(g: &Graph) -> HashMap<char, Vec<(i32, i32)>> {
    let mut locs = HashMap::new();

    for (y, row) in g.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c != '.' {
                let pos = (x as i32, y as i32);
                locs.entry(*c)
                    .and_modify(|v: &mut Vec<(i32, i32)>| v.push(pos))
                    .or_insert_with(|| vec![pos]);
            }
        }
    }
    locs
}

fn in_graph(g: &Graph, x: i32, y: i32) -> bool {
    if y >= g.len() as i32 || y < 0 || x >= g[0].len() as i32 || x < 0 {
        return false;
    }

    true
}

fn find_antinodes(g: &Graph, antenna: (i32, i32), rest: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut antinodes = vec![];

    for other in &rest {
        println!("calculating {:?} - {:?}", antenna, other);
        let dir = (other.0 - antenna.0, other.1 - antenna.1);
        println!("dir: {:?}", dir);

        for i in 0..100 {
            let dir = (dir.0 * i, dir.1 * i);

            let nodes = [
                (other.0 + dir.0, other.1 + dir.1),
                (antenna.0 - dir.0, antenna.1 - dir.1),
            ];
            println!("antinodes: {:?}", &nodes);
            let nodes = nodes.into_iter().filter(|(x, y)| in_graph(&g, *x, *y));
            antinodes.extend(nodes);
        }
    }

    if let Some((start, rest)) = rest.split_first() {
        antinodes.append(&mut find_antinodes(g, *start, rest.to_vec()));
    }

    antinodes
}

fn solve(graph: &Graph) -> bool {
    let antennas = find_antennas(&graph);
    println!("{:?}", antennas);

    let mut unique_positions: HashSet<(i32, i32)> = HashSet::new();
    for (freq, locations) in antennas {
        println!("Finding antinodes for {} in {:?}", freq, locations);
        let (start, rest) = locations.split_first().unwrap();
        let antinodes = find_antinodes(graph, *start, rest.to_vec());
        println!("Found {} antinodes: {:?}", antinodes.len(), antinodes);

        unique_positions.extend(antinodes);
    }
    println!("Found {} unique antinodes", unique_positions.len());

    true
}

fn main() {
    let _file = read_to_string("input.txt").unwrap();
    let input = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    let input = _file;
    let graph: Graph = input.lines().map(|s| s.chars().collect()).collect();

    let print_graph = |g: &Graph| {
        for row in g {
            for c in row {
                print!("{}", c);
            }
            println!();
        }
    };

    print_graph(&graph);
    println!("{} x {}", graph[0].len(), graph.len());

    solve(&graph);
}
