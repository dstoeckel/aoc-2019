use std::io::Read;

use std::collections::{HashMap, HashSet, VecDeque};

fn test_example_large() {
    let data = "                   A               
                   A               
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P               ";

    let map = parse_map(&data);
    assert_eq!(58, bfs(&map));
}

#[test]
fn test_example_large_recursive() {
    let data = "             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     ";
    let map = parse_map(&data);
    assert_eq!(396, bfs(&map));
}
struct Array2d<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T> Array2d<T> {
    fn new(width: usize, height: usize, data: Vec<T>) -> Self {
        Array2d {
            width,
            height,
            data,
        }
    }
}

impl<T> std::ops::Index<(usize, usize)> for Array2d<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        if x >= self.width || y >= self.height {
            panic!("Access out of bounds");
        }

        &self.data[y * self.width + x]
    }
}

impl<T> std::ops::IndexMut<(usize, usize)> for Array2d<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        if x >= self.width || y >= self.height {
            panic!("Access out of bounds");
        }

        &mut self.data[y * self.width + x]
    }
}

#[derive(Clone)]
enum Tile {
    Empty,
    Portal(String),
    Wall,
}

fn is_portal(tmp: &Array2d<char>, x: usize, y: usize) -> Option<String> {
    if x > 0 && tmp[(x - 1, y)].is_uppercase() {
        let mut s = String::new();
        s.push(tmp[(x - 2, y)]);
        s.push(tmp[(x - 1, y)]);
        Some(s)
    } else if x < tmp.width && tmp[(x + 1, y)].is_uppercase() {
        let mut s = String::new();
        s.push(tmp[(x + 1, y)]);
        s.push(tmp[(x + 2, y)]);
        Some(s)
    } else if y > 0 && tmp[(x, y - 1)].is_uppercase() {
        let mut s = String::new();
        s.push(tmp[(x, y - 2)]);
        s.push(tmp[(x, y - 1)]);
        Some(s)
    } else if y < tmp.height && tmp[(x, y + 1)].is_uppercase() {
        let mut s = String::new();
        s.push(tmp[(x, y + 1)]);
        s.push(tmp[(x, y + 2)]);
        Some(s)
    } else {
        None
    }
}

fn parse_map(input: &str) -> Array2d<Tile> {
    let width = input.chars().position(|c| c == '\n').unwrap();

    let data: Vec<_> = input.chars().filter(|&c| c != '\n').collect();

    println!("{} {}", width, data.len());

    let tmp = Array2d::new(width, data.len() / width, data);

    let mut min_x = usize::max_value();
    let mut max_x = 0;
    let mut min_y = usize::max_value();
    let mut max_y = 0;

    for x in 0..tmp.width {
        for y in 0..tmp.height {
            match tmp[(x, y)] {
                '.' | '#' => {
                    min_x = std::cmp::min(x, min_x);
                    max_x = std::cmp::max(x, max_x);
                    min_y = std::cmp::min(y, min_y);
                    max_y = std::cmp::max(y, max_y);
                }
                _ => continue,
            }
        }
    }
    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;

    let mut map = Array2d::new(width, height, vec![Tile::Wall; width * height]);
    for i in 0..map.height {
        for j in 0..map.width {
            map[(j, i)] = match tmp[(j + min_x, i + min_y)] {
                '.' => {
                    if let Some(name) = is_portal(&tmp, j + min_x, i + min_y) {
                        Tile::Portal(name)
                    } else {
                        Tile::Empty
                    }
                }
                _ => Tile::Wall,
            }
        }
    }

    map
}

fn find_start(map: &Array2d<Tile>) -> (usize, usize) {
    for x in 0..map.width {
        for y in 0..map.height {
            match map[(x, y)] {
                Tile::Portal(ref s) if s == "AA" => return (x, y),
                _ => continue,
            }
        }
    }

    panic!("Could not find start!");
}

#[derive(PartialEq, Eq, Hash)]
struct BfsData {
    steps: usize,
    level: usize,
    x: usize,
    y: usize,
}

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn find_partners(map: &Array2d<Tile>) -> HashMap<String, Vec<(usize, usize)>> {
    let mut result = HashMap::new();

    for x in 0..map.width {
        for y in 0..map.height {
            match map[(x, y)] {
                Tile::Portal(ref s) if s != "AA" && s != "ZZ" => result
                    .entry(s.clone())
                    .or_insert_with(|| Vec::new())
                    .push((x, y)),
                _ => continue,
            }
        }
    }

    result
}

fn bfs(map: &Array2d<Tile>) -> usize {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    let start = find_start(map);
    let partners = find_partners(map);

    queue.push_back(BfsData {
        steps: 0,
        level: 0,
        x: start.0,
        y: start.1,
    });

    while let Some(n) = queue.pop_front() {
        if visited.contains(&(n.x, n.y, n.level)) {
            continue;
        }

        match map[(n.x, n.y)] {
            Tile::Portal(ref s) if s == "ZZ" => {
                if n.level == 0 {
                    return n.steps;
                }
            }
            Tile::Portal(ref s) if s != "AA" => {
                // If an outer portal
                let partner = partners
                    .get(s)
                    .unwrap()
                    .iter()
                    .find(|&&p| p != (n.x, n.y))
                    .unwrap();
                if n.x == 0 || n.y == 0 || n.x + 1 == map.width || n.y + 1 == map.height {
                    if n.level > 0 {
                        queue.push_back(BfsData {
                            steps: n.steps + 1,
                            level: n.level - 1,
                            x: partner.0,
                            y: partner.1,
                        });
                    }
                } else {
                    queue.push_back(BfsData {
                        steps: n.steps + 1,
                        level: n.level + 1,
                        x: partner.0,
                        y: partner.1,
                    });
                }
            }
            _ => (),
        }

        for d in DIRECTIONS.iter() {
            let x = n.x as isize + d.0;
            let y = n.y as isize + d.1;

            if x < 0 || x >= map.width as isize {
                continue;
            }

            if y < 0 || y >= map.height as isize {
                continue;
            }

            let x = x as usize;
            let y = y as usize;

            match map[(x, y)] {
                Tile::Empty | Tile::Portal(_) => queue.push_back(BfsData {
                    steps: n.steps + 1,
                    level: n.level,
                    x,
                    y,
                }),
                Tile::Wall => continue,
            }
        }
        visited.insert((n.x, n.y, n.level));
    }

    panic!("Could not find path!")
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

    let map = parse_map(&map);

    println!("Shortest path is {}", bfs(&map));
}
