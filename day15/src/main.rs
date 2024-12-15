use std::fs::read_to_string;

type Graph = Vec<Vec<char>>;

fn main() {
    let _file = read_to_string("input.txt").unwrap();
    let _test1 = r"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
    let _test2 = r"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    let input = _file;
    let mut section = 0;
    let mut g: Graph = vec![];
    let mut moves = vec![];
    for line in input.lines() {
        if line.is_empty() {
            section += 1;
            continue;
        }

        match section {
            0 => g.push(line.chars().collect()),
            1 => moves.extend(line.chars().collect::<Vec<_>>()),
            _ => panic!("Invalid section"),
        }
    }

    let print_graph = |g: &Graph| {
        for row in g {
            for c in row {
                print!("{}", c);
            }
            println!();
        }
    };

    print_graph(&g);
    let mut pos = *find_char(&g, '@').first().unwrap();
    println!("Starting pos: {:?}", pos);

    for m in moves {
        let dir = match m {
            '^' => (0, -1),
            'v' => (0, 1),
            '<' => (-1, 0),
            '>' => (1, 0),
            _ => panic!("Invalid move"),
        };
        println!("Move: {}, dir: {:?}", m, dir);
        if apply_move(&mut g, pos, dir) {
            pos = (pos.0 + dir.0, pos.1 + dir.1);
        }

        print_graph(&g);
    }

    let boxes = find_char(&g, 'O');
    let sum = boxes.iter().map(|(x, y)| 100 * y + x).sum::<i32>();
    println!("Sum: {}", sum);
}

fn find_char(g: &Graph, c: char) -> Vec<(i32, i32)> {
    let mut pos = vec![];
    for (y, row) in g.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == c {
                pos.push((x as i32, y as i32));
            }
        }
    }

    pos
}

fn in_graph(g: &Graph, x: i32, y: i32) -> bool {
    if y >= g.len() as i32 || y < 0 || x >= g[0].len() as i32 || x < 0 {
        return false;
    }

    true
}

fn apply_move(g: &mut Graph, pos: (i32, i32), dir: (i32, i32)) -> bool {
    let (x, y) = pos;
    let (nx, ny) = (pos.0 + dir.0, pos.1 + dir.1);
    if !in_graph(g, nx, ny) {
        println!("Out of graph");
        return false;
    }

    let c = g[y as usize][x as usize];
    let nc = g[ny as usize][nx as usize];
    if nc == '#' {
        println!("Hit wall");
        return false;
    }

    if nc == '.' {
        g[y as usize][x as usize] = '.';
        g[ny as usize][nx as usize] = c;
        println!("Moved to empty cell");
        return true;
    }

    if nc == 'O' {
        if !apply_move(g, (nx, ny), dir) {
            return false;
        } else {
            g[y as usize][x as usize] = '.';
            g[ny as usize][nx as usize] = c;
            println!("Pushed a block");
            return true;
        }
    }

    false
}
