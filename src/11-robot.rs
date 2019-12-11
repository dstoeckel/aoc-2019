use std::collections::HashSet;
use std::io::Read;

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("At least one command line argument is required.");

    let mut code = String::new();
    std::fs::File::open(path)
        .expect("Could not open input file")
        .read_to_string(&mut code)
        .expect("Error while reading from file.");

    let instructions = code
        .trim()
        .split(',')
        .map(|x| str::parse::<isize>(x).unwrap())
        .collect::<Vec<_>>();

    let mut white = HashSet::new();
    white.insert((0, 0));
    let mut painted = HashSet::new();

    let mut position = (0isize, 0isize);
    let mut direction = (0, -1);
    let mut interpreter = intcode::Intcode::new(instructions);

    let mut has_painted = false;
    let mut input = 0;

    loop {
        match (interpreter.step(input), has_painted) {
            (intcode::State::Input, _) => input = white.contains(&position) as isize,
            (intcode::State::Output(o), false) => {
                painted.insert(position);

                if o == 0 {
                    white.remove(&position);
                } else {
                    white.insert(position);
                }
                has_painted = true;
            }
            (intcode::State::Output(o), true) => {
                if o == 0 {
                    direction = (direction.1, -direction.0);
                } else if o == 1 {
                    direction = (-direction.1, direction.0);
                } else {
                    panic!("Unhandled direction change");
                }

                position = (position.0 + direction.0, position.1 + direction.1);
                has_painted = false;
            }
            (intcode::State::Terminated, _) => break,
        }
    }

    println!("Painted {} tiles", painted.len());

    let min_x = white.iter().map(|x| x.0).min().unwrap();
    let min_y = white.iter().map(|x| x.1).min().unwrap();

    let max_x = white.iter().map(|x| x.0).max().unwrap();
    let max_y = white.iter().map(|x| x.1).max().unwrap();

    let width = (max_x - min_x + 1) as usize;
    let height = (max_y - min_y + 1) as usize;

    let mut img = vec![' '; width * height];
    for (x, y) in white {
        img[(x - min_x) as usize + (y - min_y) as usize * width] = '█';
    }

    for y in 0..height {
        for x in 0..width {
            print!("{}", img[x + y * width]);
        }
        println!();
    }
}
