use core::net;
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

#[derive(Debug, PartialEq)]
enum EdgeType {
    Internal,
    External,
}

fn find_edges(g: &Graph, c: char, x: i32, y: i32) -> Vec<(i32, i32, EdgeType)> {
    let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut edges = vec![];
    for (dx, dy) in directions {
        let nx = x + dx;
        let ny = y + dy;
        if !in_graph(g, nx, ny) {
            edges.push((nx, ny, EdgeType::External));
        } else if g[ny as usize][nx as usize] != c {
            edges.push((nx, ny, EdgeType::External));
        } else {
            edges.push((nx, ny, EdgeType::Internal));
        }
    }

    edges
}

fn count_external_corners(g: &Graph, c: char, x: i32, y: i32) -> usize {
    println!("Counting corners for: {}, x: {}, y: {}", c, x, y);
    let neighbour_masks = [
        vec![(-1, 0), (-1, 1), (0, 1)],   // top-left
        vec![(0, 1), (1, 1), (1, 0)],     // top-right
        vec![(1, 0), (1, -1), (0, -1)],   // bottom-right
        vec![(0, -1), (-1, -1), (-1, 0)], // bottom-left
    ];

    let mut corners = 0;

    for mask in neighbour_masks {
        let neighbours = mask
            .iter()
            .map(|(dx, dy)| (x + dx, y + dy))
            .filter(|(nx, ny)| !in_graph(g, *nx, *ny) || g[*ny as usize][*nx as usize] != c)
            .collect::<Vec<(i32, i32)>>();
        println!("neighbours: {:?}", neighbours);
        if neighbours.len() == 3 {
            corners += 1;
        }
    }
    corners
}

fn count_corners(g: &Graph, points: &HashSet<(i32, i32)>) -> usize {
    let mut corners = 0;
    for (x, y) in points {
        let c = g[*y as usize][*x as usize];
        corners += count_external_corners(g, c, *x, *y);
    }

    corners
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

        let mut corners = 4;
        for (x, y) in &v {
            let edges = find_edges(g, k, *x, *y);
            let (internal, external): (Vec<_>, Vec<_>) = edges
                .iter()
                .partition(|(_, _, edge_type)| edge_type == &EdgeType::Internal);

            println!(
                "Point: {:?}, Edges: {:?}, Corners: {:?}",
                (x, y),
                edges,
                corners
            );
        }

        new_cost += corners * v.len();
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
    /*
    EEEEE
    EXXXX
    EEEEE
    EXXXX
    EEEEE
    */
    let input = _test1;
    let graph: Graph = input.lines().map(|s| s.chars().collect()).collect();
    print_graph(&graph);
    find_areas(&graph);
}
