use std::io::Read;
use std::collections::HashSet;
use std::collections::VecDeque;

#[test]
fn test_bugs_example() {
    let layout = "....#
#..#.
#..##
..#..
#....";

    let area = Area::from_string(layout);

    let step_1 = "#..#.
####.
###.#
##.##
.##..";

    assert_eq!(area.step().to_string().trim(), step_1);
}

#[test]
fn test_bugs_set() {
let layout = "....#
#..#.
#..##
..#..
#....";

let layout_2 = ".#..#
#..#.
#..##
..#..
#....";

    let mut area = Area::from_string(layout);
    area.set(1, 0, Bug::Bug);

    assert_eq!(area, Area::from_string(layout_2));
    area.set(1, 0, Bug::NoBug);
    assert_eq!(area, Area::from_string(layout));
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Area(u32);

#[derive(Clone, Copy)]
enum Bug {
    NoBug,
    Bug,
}

impl Area {
    fn empty() -> Area {
        Area(0)
    }

    fn from_string(s: &str) -> Area {
        let mut data = 0;

        s.chars().filter(|&c| c != '\n').enumerate().for_each(|(i, c)| match c {
            '#' => data |= 1 << i,
            '.' => (),
            c => panic!("Unexpected input '{}'!", c),
        });

        Area(data)
    }

    fn get(self, x: usize, y: usize) -> Bug {
        if self.0 & (1 << (y*5 + x)) == 0 {
            Bug::NoBug
        } else {
            Bug::Bug
        }
    }

    fn get_u32(self, x: usize, y: usize) -> u32 {
        if self.0 & (1 << (y*5 + x)) == 0 {
            0
        } else {
            1
        }
    }

    fn get_recursive_u32(self, x_from: usize, y_from: usize, x: usize, y: usize, below: Area) -> u32 {
        if x == 2 && y == 2 {
            let mut sum = 0;
            if x_from < x {
                for y_ in 0..5 {
                    sum += below.get_u32(0, y_);
                }
            } else if y_from < y {
                for x_ in 0..5 {
                    sum += below.get_u32(x_, 0);
                }
            } else if x_from > x {
                for y_ in 0..5 {
                    sum += below.get_u32(4, y_);
                }
            } else if y_from > y {
                for x_ in 0..5 {
                    sum += below.get_u32(x_, 4);
                }
            } else {
                panic!("Cannot determine direction of access!");
            }

            sum
        } else {
            self.get_u32(x, y)
        }
    }

    fn set(&mut self, x: usize, y: usize, b: Bug) {
        match b {
            Bug::Bug   => self.0 |=   1 << (y*5 + x),
            Bug::NoBug => self.0 &= !(1 << (y*5 + x)),
        }
    }

    fn step(self) -> Area {
        let mut result = Area::empty();

        for x in 0..5 {
            for y in 0..5 {
                let mut sum = 0;

                if x > 0 {
                    sum += self.get_u32(x - 1, y);
                }

                if y > 0 {
                    sum += self.get_u32(x, y - 1);
                }

                if x < 4 {
                    sum += self.get_u32(x + 1, y);
                }

                if y < 4 {
                    sum += self.get_u32(x, y + 1);
                }

                let new_value = match self.get(x, y) {
                    Bug::NoBug if sum == 1 || sum == 2 => Bug::Bug,
                    Bug::Bug if sum == 1 => Bug::Bug,
                    _ => Bug::NoBug,
                };

                result.set(x, y, new_value);
            }
        }

        result
    }

    fn step_recursive(self, above: Area, below: Area) -> Area {
        let mut result = Area::empty();

        for x in 0..5 {
            for y in 0..5 {
                if x == 2 && y == 2 {
                    continue;
                }

                let mut sum = 0;

                if x == 0 {
                    sum += above.get_u32(1, 2);
                }

                if x == 4 {
                    sum += above.get_u32(3, 2);
                }

                if x > 0 {
                    sum += self.get_recursive_u32(x, y, x - 1, y, below);
                }

                if x < 4 {
                    sum += self.get_recursive_u32(x, y, x + 1, y, below);
                }

                if y == 0 {
                    sum += above.get_u32(2, 1);
                }

                if y == 4 {
                    sum += above.get_u32(2, 3);
                }

                if y > 0 {
                    sum += self.get_recursive_u32(x, y, x, y - 1, below);
                }

                if y < 4 {
                    sum += self.get_recursive_u32(x, y, x, y + 1, below);
                }

                let new_value = match self.get(x, y) {
                    Bug::NoBug if sum == 1 || sum == 2 => Bug::Bug,
                    Bug::Bug if sum == 1 => Bug::Bug,
                    _ => Bug::NoBug,
                };

                result.set(x, y, new_value);
            }
        }

        result
    }

    fn biodiversity(self) -> u32 {
        self.0
    }
}

impl std::fmt::Display for Area {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in 0..5 {
            for x in 0..5 {
                match self.get(x, y) {
                    Bug::Bug => write!(fmt, "#")?,
                    Bug::NoBug => write!(fmt, ".")?,
                }
            }
            write!(fmt, "\n")?;
        }

        Ok(())
    }
}

fn recursive(area: Area, n: usize) -> Vec<Area> {
    let mut levels = vec![Area::empty(); n + 3];
    levels[n / 2 + 1] = area;

    for _ in 0..n {
        let mut tmp = levels.clone();
        for j in 1..(levels.len() - 1) {
            tmp[j] = levels[j].step_recursive(levels[j - 1], levels[j + 1]);
        }

        std::mem::swap(&mut levels, &mut tmp);
    }

    levels
}

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("At least one command line argument is required.");

    let mut buffer = String::new();
    std::fs::File::open(path)
        .expect("Could not open input file")
        .read_to_string(&mut buffer)
        .expect("Error while reading from file.");

    let mut area = Area::from_string(&buffer);
    let mut states = HashSet::new();
    states.insert(area.0);

    loop {
        area = area.step();

        if states.contains(&area.0) {
            println!("Biodiversity of first replicating layout {}", area.biodiversity());
            break;
        }

        states.insert(area.0);
    }

    let result = recursive(Area::from_string(&buffer), 200);
    let total = result.iter().map(|x| x.0.count_ones()).sum::<u32>();
    println!("Number of living bugs: {}", total);
}
