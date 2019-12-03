use std::collections::HashMap;
use std::collections::HashSet;
use std::io::BufRead;

#[test]
fn test_example_1() {
    let w1 = coordinates("R75,D30,R83,U83,L12,D49,R71,U7,L72".to_owned());
    let w2 = coordinates("U62,R66,U55,R34,D71,R55,D58,R83".to_owned());

    let (_, _, d) = distance(&w1, &w2);

    assert_eq!(159, d);
}

#[test]
fn test_example_2() {
    let w1 = coordinates("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".to_owned());
    let w2 = coordinates("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_owned());

    let (_, _, d) = distance(&w1, &w2);

    assert_eq!(135, d);
}

#[test]
fn test_example_1_steps() {
    let w1 = coordinates("R75,D30,R83,U83,L12,D49,R71,U7,L72".to_owned());
    let w2 = coordinates("U62,R66,U55,R34,D71,R55,D58,R83".to_owned());

    let (_, _, d) = steps_until(&w1, &w2);

    assert_eq!(610, d);
}

#[test]
fn test_example_2_steps() {
    let w1 = coordinates("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".to_owned());
    let w2 = coordinates("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_owned());

    let (_, _, d) = steps_until(&w1, &w2);

    assert_eq!(410, d);
}

fn coordinates(s: String) -> Vec<(isize, isize)> {
    let mut x = 0;
    let mut y = 0;

    s.split(',')
        .flat_map(|instr| {
            let mut iter = instr.chars();

            let direction = iter.next();
            let count = str::parse::<isize>(iter.as_str())
                .expect("Expected a valid integer after direction specifier.");

            let d = match direction {
                Some('R') => (1, 0),
                Some('L') => (-1, 0),
                Some('U') => (0, 1),
                Some('D') => (0, -1),
                _ => panic!("End of string or unhandled control character."),
            };

            let mut result = Vec::with_capacity(count as usize);
            for _ in 0..count {
                x += d.0;
                y += d.1;
                result.push((x, y));
            }

            result
        })
        .collect()
}

fn distance(a: &Vec<(isize, isize)>, b: &Vec<(isize, isize)>) -> (isize, isize, isize) {
    let a = a.iter().collect::<HashSet<_>>();
    let b = b.iter().collect::<HashSet<_>>();

    let mut best = (isize::max_value(), isize::max_value(), isize::max_value());
    for candidate in a.intersection(&b).map(|(x, y)| (*x, *y, x.abs() + y.abs())) {
        if candidate.2 < best.2 {
            best = candidate;
        }
    }

    best
}

fn count_steps(
    wire: &Vec<(isize, isize)>,
    inter: &HashSet<(isize, isize)>,
    result: &mut HashMap<(isize, isize), isize>,
) {
    let mut steps = 1;
    for coord in wire {
        if inter.contains(coord) {
            *result.entry(*coord).or_insert(0) += steps;
        }

        steps += 1;
    }
}

fn steps_until(a: &Vec<(isize, isize)>, b: &Vec<(isize, isize)>) -> (isize, isize, isize) {
    let a_s = a.iter().copied().collect::<HashSet<_>>();
    let b_s = b.iter().copied().collect::<HashSet<_>>();

    let intersections = a_s.intersection(&b_s).copied().collect::<HashSet<_>>();

    let mut results = HashMap::new();
    count_steps(&a, &intersections, &mut results);
    count_steps(&b, &intersections, &mut results);

    let mut best = (isize::max_value(), isize::max_value(), isize::max_value());
    for (candidate, value) in results {
        if value < best.2 {
            best = (candidate.0, candidate.1, value)
        }
    }

    best
}

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("At least one command line argument is required.");

    let input = std::fs::File::open(path).expect("Could not open input file");

    let wires = std::io::BufReader::new(input)
        .lines()
        .map(|l| coordinates(l.unwrap()))
        .collect::<Vec<_>>();

    let best = distance(&wires[0], &wires[1]);
    println!(
        "Closest intersection is at {}, {} with distance {}",
        best.0, best.1, best.2
    );

    let best2 = steps_until(&wires[0], &wires[1]);
    println!(
        "Intersection with fewest stepsis at {}, {} with distance {}",
        best2.0, best2.1, best2.2
    );
}
