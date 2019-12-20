use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::Read;

#[test]
fn test_keys_simple() {
    let map = "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";

    let map = parse_map(&map);
    assert_eq!(86, bfs_single(&map));
}

#[test]
fn test_keys_simple_2() {
    let map = "########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################";

    let map = parse_map(&map);
    assert_eq!(132, bfs_single(&map));
}

#[test]
fn test_keys_combinatorics() {
    let map = "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################";

    let map = parse_map(&map);
    assert_eq!(136, bfs_single(&map));
}

#[test]
fn test_keys_medium() {
    let map = "########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################";

    let map = parse_map(&map);
    assert_eq!(81, bfs_single(&map));
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Tile {
    Wall,
    Empty,
    Start,
    Door(usize),
    Key(usize),
}

struct Array2d<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T> Array2d<T> {
    fn new(width: usize, height: usize, data: Vec<T>) -> Array2d<T> {
        Array2d {
            width,
            height,
            data,
        }
    }
}

impl<T> std::ops::Index<(usize, usize)> for Array2d<T> {
    type Output = T;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        if i >= self.height || j >= self.width {
            panic!("Acces out of bounds");
        }

        &self.data[i * self.width + j]
    }
}

impl<T> std::ops::IndexMut<(usize, usize)> for Array2d<T> {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        if i >= self.height || j >= self.width {
            panic!("Acces out of bounds");
        }

        &mut self.data[i * self.width + j]
    }
}

struct Maze {
    map: Array2d<Tile>,
    idx_to_char: Vec<char>,
}

fn parse_map(input: &str) -> Maze {
    let width = input.chars().position(|c| c == '\n').unwrap();

    let mut char_to_idx = HashMap::new();

    let data: Vec<_> = input
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| match c {
            '.' => Tile::Empty,
            '#' => Tile::Wall,
            '@' => Tile::Start,
            c if c.is_lowercase() => {
                let n = char_to_idx.len();
                Tile::Key(*char_to_idx.entry(c).or_insert(n))
            }
            c if c.is_uppercase() => {
                let n = char_to_idx.len();
                Tile::Door(
                    *char_to_idx
                        .entry(c.to_lowercase().next().unwrap())
                        .or_insert(n),
                )
            }
            _ => panic!("Unhandled character!"),
        })
        .collect();

    let mut idx_to_char = vec![' '; char_to_idx.len()];

    for (k, v) in char_to_idx {
        idx_to_char[v] = k;
    }

    Maze {
        map: Array2d::new(width, data.len() / width, data),

        idx_to_char,
    }
}

const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

struct BfsData {
    pos: (usize, usize),
    keys: u64,
    steps: usize,
}

fn find_starts(map: &Array2d<Tile>) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    for i in 0..map.height {
        for j in 0..map.width {
            match map[(i, j)] {
                Tile::Start => result.push((i, j)),
                _ => continue,
            }
        }
    }

    result
}

fn bfs_single(maze: &Maze) -> usize {
    let mut unvisited = VecDeque::new();
    let mut visited = HashSet::new();
    let map = &maze.map;

    let start = find_starts(&map)[0];

    unvisited.push_back(BfsData {
        pos: start,
        keys: 0,
        steps: 0,
    });

    while let Some(next) = unvisited.pop_front() {
        if next.keys.count_ones() >= maze.idx_to_char.len() as u32 {
            return next.steps;
        }

        if visited.contains(&(next.pos, next.keys)) {
            continue;
        }

        for d in DIRECTIONS.iter() {
            let pos = (
                (next.pos.0 as isize + d.0) as usize,
                (next.pos.1 as isize + d.1) as usize,
            );

            match map[pos] {
                Tile::Key(k) => {
                    unvisited.push_back(BfsData {
                        pos,
                        keys: next.keys | 1u64 << k,
                        steps: next.steps + 1,
                    });
                }
                Tile::Wall => continue,
                Tile::Door(d) => {
                    if next.keys & (1u64 << d) == 0 {
                        continue;
                    }
                    unvisited.push_back(BfsData {
                        pos,
                        keys: next.keys,
                        steps: next.steps + 1,
                    })
                }
                Tile::Empty | Tile::Start => unvisited.push_back(BfsData {
                    pos,
                    keys: next.keys,
                    steps: next.steps + 1,
                }),
            }
        }
        visited.insert((next.pos, next.keys));
    }

    panic!("Could not find path!");
}

struct MultiBfsData {
    pos: [(usize, usize); 4],
    keys: u64,
    steps: usize,
    active: usize,
}

fn bfs_multi(maze: &Maze) -> usize {
    let mut unvisited = VecDeque::new();
    let mut visited = HashSet::new();
    let map = &maze.map;

    let starts = find_starts(&map);

    for bot in 0..4 {
        unvisited.push_back(MultiBfsData {
            pos: [starts[0], starts[1], starts[2], starts[3]],
            keys: 0,
            steps: 0,
            active: bot,
        });
    }

    while let Some(next) = unvisited.pop_front() {
        if next.keys.count_ones() >= maze.idx_to_char.len() as u32 {
            return next.steps;
        }

        if visited.contains(&(next.pos, next.keys, next.active)) {
            continue;
        }

        for d in DIRECTIONS.iter() {
            let mut pos = next.pos.clone();
            pos[next.active] = (
                (pos[next.active].0 as isize + d.0) as usize,
                (pos[next.active].1 as isize + d.1) as usize
            );

            match map[pos[next.active]] {
                Tile::Key(k) => {
                    for bot in 0..4 {
                        unvisited.push_back(MultiBfsData {
                            pos,
                            keys: next.keys | 1u64 << k,
                            steps: next.steps + 1,
                            active: bot,
                        });
                    }
                }
                Tile::Wall => continue,
                Tile::Door(d) => {
                    if next.keys & (1u64 << d) == 0 {
                        continue;
                    }
                    unvisited.push_back(MultiBfsData {
                        pos,
                        keys: next.keys,
                        steps: next.steps + 1,
                        active: next.active,
                    })
                }
                Tile::Empty | Tile::Start => unvisited.push_back(MultiBfsData {
                    pos,
                    keys: next.keys,
                    steps: next.steps + 1,
                    active: next.active,
                }),
            }
        }
        visited.insert((next.pos, next.keys, next.active));
    }

    panic!("Could not find path!");
}

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("At least one command line argument is required.");

    let mut map = String::new();
    std::fs::File::open(path)
        .expect("Could not open input file")
        .read_to_string(&mut map)
        .expect("Error while reading from file.");

    let mut maze = parse_map(&map);
    println!("Possible tour {:?}", bfs_single(&maze));

    // Modify the map
    let start = find_starts(&maze.map)[0];
    maze.map[start] = Tile::Wall;
    maze.map[(start.0 - 1, start.1)] = Tile::Wall;
    maze.map[(start.0 + 1, start.1)] = Tile::Wall;
    maze.map[(start.0, start.1 - 1)] = Tile::Wall;
    maze.map[(start.0, start.1 + 1)] = Tile::Wall;

    maze.map[(start.0 - 1, start.1 + 1)] = Tile::Start;
    maze.map[(start.0 + 1, start.1 - 1)] = Tile::Start;
    maze.map[(start.0 - 1, start.1 - 1)] = Tile::Start;
    maze.map[(start.0 + 1, start.1 + 1)] = Tile::Start;

    println!("Possible tour {:?}", bfs_multi(&maze));
}
