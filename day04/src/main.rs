use std::{fs::read_to_string, vec};

type Graph = Vec<Vec<char>>;

fn find_char(g: &Graph, needle: char) -> Vec<(usize, usize)> {
    let mut xs = vec![];

    for (y, row) in g.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == needle {
                xs.push((x, y));
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

fn find_xmas(
    g: &Graph,
    x: usize,
    y: usize,
    stack: Vec<char>,
    mut positions: Vec<(usize, usize)>,
    direction: [i32; 2],
    directions: [[i32; 2]; 8],
) -> i32 {
    assert!(!stack.is_empty());
    let (c, rest) = stack.split_at(1);
    // println!(
    //     "x: {}, y: {}, g[y][x] = {}, c: {:?}, rest: {:?}, dir: {:?}",
    //     x, y, g[y][x], c, rest, direction
    // );

    if g[y][x] != c[0] {
        return 0;
    }

    positions.push((x, y));

    if rest.is_empty() {
        println!("Found xmas! {:?}, direction: {:?}", &positions, direction);
        return 1;
    }

    let mut total: i32 = 0;

    let can_go = |x: i32, y: i32| -> bool {
        return in_graph(g, x, y) && g[y as usize][x as usize] == rest[0];
    };

    for [dx, dy] in [direction] {
        let [x, y] = [x as i32 + dx, y as i32 + dy];

        if !can_go(x, y) {
            continue;
        }

        total += find_xmas(
            &g,
            x as usize,
            y as usize,
            rest.to_vec(),
            positions.clone(),
            direction,
            directions,
        );
    }

    total
}

fn find_mas(g: &Graph, x: usize, y: usize) -> bool {
    let corners: [[i32; 2]; 4] = [[-1, -1], [1, -1], [1, 1], [-1, 1]];
    let mut neighbours = vec![];

    for [dx, dy] in corners {
        let [x, y] = [x as i32 + dx, y as i32 + dy];
        if !in_graph(&g, x, y) {
            continue;
            // return false;
        }

        neighbours.push(g[y as usize][x as usize]);
    }
    // println!("A ({},{}): neighbours {:?}", x, y, neighbours);

    let masks = [
        ['M', 'S', 'S', 'M'],
        ['M', 'M', 'S', 'S'],
        ['S', 'S', 'M', 'M'],
        ['S', 'M', 'M', 'S'],
    ];

    for mask in masks {
        if mask == *neighbours {
            return true;
        }
    }

    false
}

fn main() {
    let file = read_to_string("input.txt").unwrap();
    let input = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    let input = file;
    let graph: Graph = input.lines().map(|s| s.chars().collect()).collect();
    println!("{:?}", &graph);

    let xs = find_char(&graph, 'X');
    let mut result = 0;

    let directions: [[i32; 2]; 8] = [
        [-1, -1],
        [-1, 0],
        [-1, 1],
        [0, 1],
        // [0, 0],
        [0, -1],
        [1, -1],
        [1, 0],
        [1, 1],
    ];

    for pos in xs {
        for direction in directions {
            result += find_xmas(
                &graph,
                pos.0,
                pos.1,
                "XMAS".chars().collect(),
                vec![],
                direction,
                directions,
            );
        }
    }
    println!("Result: {}", result);

    let a_pos = find_char(&graph, 'A');
    println!("Found {} A's", a_pos.len());
    let mut total_mas = 0;
    for pos in a_pos {
        if find_mas(&graph, pos.0, pos.1) {
            total_mas += 1;
        }
    }

    println!("total x-mas: {}", total_mas);
}
