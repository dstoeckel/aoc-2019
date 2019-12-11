use std::io::BufRead;

#[test]
fn test_compute_all_visibility() {
    let data = ".#..#\n.....\n#####\n....#\n...##".as_bytes();

    let map = AsteroidMap::parse(std::io::BufReader::new(data));
    let visibility = compute_all_visible(&map);

    assert_eq!(
        visibility,
        vec![0, 7, 0, 0, 7, 0, 0, 0, 0, 0, 6, 7, 7, 7, 5, 0, 0, 0, 0, 7, 0, 0, 0, 8, 7]
    )
}

#[test]
fn test_vaporize() {
    let data = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##"
        .as_bytes();

    let mut map = AsteroidMap::parse(std::io::BufReader::new(data));
    let vaporized = vaporize(&mut map, 11, 13, 200);

    assert_eq!((8, 2), vaporized);
}

#[derive(PartialEq)]
enum MapEntry {
    Asteroid,
    Empty,
}

impl From<char> for MapEntry {
    fn from(c: char) -> MapEntry {
        match c {
            '#' => MapEntry::Asteroid,
            '.' => MapEntry::Empty,
            _ => panic!("Unhandled map entry."),
        }
    }
}

struct AsteroidMap {
    data: Vec<MapEntry>,
    width: usize,
    height: usize,
}

impl AsteroidMap {
    fn parse<R: BufRead>(reader: R) -> AsteroidMap {
        let lines: Vec<Vec<MapEntry>> = reader
            .lines()
            .map(|l| {
                l.unwrap()
                    .trim()
                    .chars()
                    .map(char::into)
                    .collect::<Vec<MapEntry>>()
            })
            .collect();

        let height = lines.len();
        let width = lines.iter().nth(0).unwrap().len();

        AsteroidMap {
            data: lines.into_iter().flat_map(|l| l).collect(),
            width,
            height,
        }
    }

    fn is_asteroid(&self, i: usize, j: usize) -> bool {
        if i >= self.width || j >= self.height {
            false
        } else {
            self.data[j * self.width + i] == MapEntry::Asteroid
        }
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if a < b {
        gcd(b, a);
    }

    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

fn compute_number_visible(map: &AsteroidMap, i: usize, j: usize) -> usize {
    let i = i as isize;
    let j = j as isize;
    let w = map.width as isize;
    let h = map.height as isize;

    let left = -i;
    let right = w - i;
    let top = -j;
    let bottom = h - j;

    let mut sum = 0;
    for k in left..right {
        for l in top..bottom {
            if k == 0 && l == 0 {
                continue;
            }

            if gcd(k.abs() as usize, l.abs() as usize) > 1 {
                continue;
            }

            let (mut pos_i, mut pos_j) = (i + k, j + l);

            while pos_i >= 0 && pos_j >= 0 && pos_i <= w && pos_j <= h {
                if map.is_asteroid(pos_i as usize, pos_j as usize) {
                    sum += 1;
                    break;
                }

                pos_i += k;
                pos_j += l;
            }
        }
    }

    sum
}

fn compute_visible(map: &AsteroidMap, i: usize, j: usize) -> Vec<(usize, usize)> {
    let i = i as isize;
    let j = j as isize;
    let w = map.width as isize;
    let h = map.height as isize;

    let left = -i;
    let right = w - i;
    let top = -j;
    let bottom = h - j;

    let mut result = Vec::new();
    for k in left..right {
        for l in top..bottom {
            if k == 0 && l == 0 {
                continue;
            }

            if gcd(k.abs() as usize, l.abs() as usize) > 1 {
                continue;
            }

            let (mut pos_i, mut pos_j) = (i + k, j + l);

            while pos_i >= 0 && pos_j >= 0 && pos_i <= w && pos_j <= h {
                if map.is_asteroid(pos_i as usize, pos_j as usize) {
                    result.push((pos_i as usize, pos_j as usize));
                    break;
                }

                pos_i += k;
                pos_j += l;
            }
        }
    }

    result
}

fn compute_all_visible(map: &AsteroidMap) -> Vec<usize> {
    let mut result = vec![0; map.width * map.height];
    for i in 0..map.height {
        for j in 0..map.width {
            if map.is_asteroid(i, j) {
                result[j * map.width + i] = compute_number_visible(&map, i, j);
            }
        }
    }

    result
}

#[derive(PartialEq, PartialOrd)]
struct SortableF64(f64);

impl std::cmp::Eq for SortableF64 {}

impl std::cmp::Ord for SortableF64 {
    fn cmp(&self, other: &SortableF64) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn vaporize(map: &mut AsteroidMap, i: usize, j: usize, mut n: usize) -> (usize, usize) {
    if n == 0 {
        panic!("Positive n required");
    }

    if map
        .data
        .iter()
        .filter(|x| **x == MapEntry::Asteroid)
        .count()
        < n
    {
        panic!("Not enough asteroids!");
    }

    loop {
        let mut visible = compute_visible(&map, i, j);
        visible.sort_by_key(|(x, y)| {
            let i_ = *x as isize - i as isize;
            let j_ = *y as isize - j as isize;

            let angle = (i_ as f64).atan2(j_ as f64);
            SortableF64(-angle)
        });

        if visible.len() == 0 {
            panic!("Could not find visible asteroids");
        }

        if visible.len() >= n {
            return visible[n - 1];
        }

        n -= visible.len();
        for a in visible {
            map.data[a.1 * map.width + a.0] = MapEntry::Empty;
        }
    }
}

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("At least one command line argument is required.");

    let input = std::fs::File::open(path).expect("Could not open input file");
    let reader = std::io::BufReader::new(input);

    let mut map = AsteroidMap::parse(reader);

    let visibility = compute_all_visible(&map);
    let best = visibility.iter().enumerate().max_by_key(|x| x.1).unwrap();

    let pos_i = best.0 % map.width;
    let pos_j = best.0 / map.width;
    println!(
        "Best position is ({}, {}) with {} visible asteroids.",
        pos_i, pos_j, best.1
    );

    let num200 = vaporize(&mut map, pos_i, pos_j, 200);
    println!("200th vaporized asteroid is {}", num200.0 * 100 + num200.1);
}
