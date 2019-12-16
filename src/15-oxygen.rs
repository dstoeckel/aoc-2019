use intcode::State;
use std::collections::{HashMap, VecDeque};

static DIRECTIONS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

#[derive(PartialEq, Eq, Clone, Copy)]
enum Tile {
    Wall,
    Empty,
    Oxgenator,
}

fn print_map(robot_map: &HashMap<(isize, isize), Tile>) {
    let min_x = robot_map.keys().map(|x| x.0).min().unwrap();
    let min_y = robot_map.keys().map(|x| x.1).min().unwrap();
    let max_x = robot_map.keys().map(|x| x.0).max().unwrap() + 1;
    let max_y = robot_map.keys().map(|x| x.1).max().unwrap() + 1;

    println!("\x1b\x63");
    for j in min_x..max_x {
        for i in min_y..max_y {
            let c = if (i, j) == (0, 0) {
                'x'
            } else if robot_map.contains_key(&(j, i)) {
                let tile = robot_map.get(&(j, i)).unwrap();
                match tile {
                    Tile::Empty => ' ',
                    Tile::Wall => '█',
                    Tile::Oxgenator => 'o',
                }
            } else {
                '.'
            };

            print!("{}", c);
        }
        println!();
    }
}

fn candidate(
    robot_map: &HashMap<(isize, isize), Tile>,
    position: (isize, isize),
) -> Option<((isize, isize), isize)> {
    for i in 0..4 {
        let c = (position.0 + DIRECTIONS[i].0, position.1 + DIRECTIONS[i].1);
        if !robot_map.contains_key(&c) {
            return Some((c, i as isize + 1));
        }
    }

    None
}

fn invert(d: isize) -> isize {
    match d {
        1 => 2,
        2 => 1,
        3 => 4,
        4 => 3,
        _ => panic!("Invalid direction!"),
    }
}

fn bfs(
    robot_map: &HashMap<(isize, isize), Tile>,
    position: (isize, isize),
) -> HashMap<(isize, isize), usize> {
    let mut result = HashMap::new();
    let mut unvisited = VecDeque::new();

    result.insert(position, 0);
    unvisited.push_back(position);

    while let Some(node) = unvisited.pop_front() {
        let value = *result.get(&node).unwrap();

        for i in 0..4 {
            let c = (node.0 + DIRECTIONS[i].0, node.1 + DIRECTIONS[i].1);

            let tile = robot_map.get(&c);

            if tile == None || tile == Some(&Tile::Wall) {
                continue;
            }

            if !result.contains_key(&c) {
                result.insert(c, value + 1);
                unvisited.push_back(c);
            }
        }
    }

    result
}

fn explore(
    robot_map: &mut HashMap<(isize, isize), Tile>,
    mut position: (isize, isize),
    robot: &mut dyn FnMut(isize) -> isize,
) -> (isize, isize) {
    let mut path = Vec::new();
    let mut oxygenator = (0, 0);

    robot_map.insert(position, Tile::Empty);

    loop {
        if let Some(c) = candidate(robot_map, position) {
            let tile = match robot(c.1) {
                0 => Tile::Wall,
                1 => Tile::Empty,
                2 => {
                    oxygenator = c.0;
                    Tile::Oxgenator
                }
                _ => panic!("Unhandled tile type!"),
            };

            robot_map.insert(c.0, tile);

            if tile != Tile::Wall {
                path.push(invert(c.1));
                position = c.0;
            }
        } else if let Some(prev) = path.pop() {
            if robot(prev) == 0 {
                panic!("Backtracking to non-empty field")
            }
            let dir = DIRECTIONS[prev as usize - 1];
            position = (position.0 + dir.0, position.1 + dir.1);
        } else {
            return oxygenator;
        }
    }
}

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("At least one command line argument is required.");

    let instructions = intcode::read_intcode_file(&path);
    let mut interpreter = intcode::Intcode::new(instructions.clone());
    interpreter.step(-1);

    let mut robot = move |dir| {
        let mut output = 0;
        loop {
            match interpreter.step(dir) {
                State::Input => return output,
                State::Output(o) => output = o,
                State::Terminated => panic!("Roboter terminated!"),
            }
        }
    };

    let mut robot_map = HashMap::new();
    let oxygenator = explore(&mut robot_map, (0, 0), &mut robot);
    print_map(&robot_map);

    let from_start = bfs(&robot_map, (0, 0));
    println!(
        "Number of steps is {}",
        from_start.get(&oxygenator).unwrap()
    );
    let from_oxygenator = bfs(&robot_map, oxygenator);
    println!(
        "Number of minutes until fully oxygenized: {}",
        from_oxygenator.values().max().unwrap()
    );
}
