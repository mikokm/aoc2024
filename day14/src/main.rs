use std::fs::read_to_string;

#[derive(Debug)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

#[derive(Debug)]
struct Quadrant {
    top_left: (i32, i32),
    bottom_right: (i32, i32),
}

fn main() {
    let _file = read_to_string("input.txt").unwrap();
    let _test1 = r"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
    let _test2 = r"p=2,4 v=2,-3";
    let input = _file;

    // let floor_size = (11, 7);
    let floor_size = (101, 103);

    let input = input.lines().filter(|s| s.len() > 0).collect::<Vec<_>>();
    let mut robots = Vec::new();
    for line in input {
        let re = regex::Regex::new(r"p=([0-9-]+),([0-9-]+) v=([0-9-]+),([0-9-]+)").unwrap();
        let caps = re.captures(line).unwrap();
        let px = caps[1].parse::<i32>().unwrap();
        let py = caps[2].parse::<i32>().unwrap();
        let vx = caps[3].parse::<i32>().unwrap();
        let vy = caps[4].parse::<i32>().unwrap();
        let robot = Robot {
            position: (px, py),
            velocity: (vx, vy),
        };
        robots.push(robot);
    }

    let print_graph = |robots: &Vec<Robot>| {
        for y in 0..floor_size.1 {
            for x in 0..floor_size.0 {
                let robots_n = robots.iter().filter(|r| r.position == (x, y)).count();
                if robots_n > 0 {
                    print!("{}", robots_n.min(9));
                } else {
                    print!(".");
                }
            }
            println!();
        }
    };
    print_graph(&robots);

    for iteration in 1..=100 {
        println!("Iteration: {}", iteration);
        for robot in &mut robots {
            println!("Robot: {:?}", robot);
            let mut nx = robot.position.0 + robot.velocity.0;
            let mut ny = robot.position.1 + robot.velocity.1;

            if nx < 0 {
                nx = floor_size.0 + nx;
            } else if nx >= floor_size.0 {
                nx = nx - floor_size.0;
            }
            if ny < 0 {
                ny = floor_size.1 + ny;
            } else if ny >= floor_size.1 {
                ny = ny - floor_size.1;
            }
            robot.position = (nx, ny);
            println!("New position: {:?}", robot.position);
        }
        print_graph(&robots);
    }

    let quadrant_size = ((floor_size.0 / 2), (floor_size.1 / 2));
    println!("Quadrant size: {:?}", quadrant_size);
    let quadrants = vec![
        Quadrant {
            top_left: (0, 0),
            bottom_right: (quadrant_size.0, quadrant_size.1),
        },
        Quadrant {
            top_left: (quadrant_size.0 + 1, 0),
            bottom_right: (floor_size.0, quadrant_size.1),
        },
        Quadrant {
            top_left: (0, quadrant_size.1 + 1),
            bottom_right: (quadrant_size.0, floor_size.1),
        },
        Quadrant {
            top_left: (quadrant_size.0 + 1, quadrant_size.1 + 1),
            bottom_right: (floor_size.0, floor_size.1),
        },
    ];
    println!("{:?}", quadrants);

    println!("{:?}", robots);
    print_graph(&robots);

    let mut robots_in_quadrants = Vec::new();
    // count robots in all quadrants
    for quadrant in &quadrants {
        let robots_in_quadrant = robots
            .iter()
            .filter(|r| {
                let (x, y) = r.position;
                x >= quadrant.top_left.0
                    && x < quadrant.bottom_right.0
                    && y >= quadrant.top_left.1
                    && y < quadrant.bottom_right.1
            })
            .count();
        robots_in_quadrants.push(robots_in_quadrant);
    }

    println!("{:?}", robots_in_quadrants);
    let safety_factor = robots_in_quadrants.iter().fold(1, |acc, x| acc * x);
    println!("Safety factor: {}", safety_factor);
}
