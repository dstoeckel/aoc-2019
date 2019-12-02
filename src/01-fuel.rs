use std::io::BufRead;

#[test]
fn test_examples() {
    assert_eq!(2, compute_fuel(12));
    assert_eq!(2, compute_fuel(14));
    assert_eq!(654, compute_fuel(1969));
    assert_eq!(33583, compute_fuel(100756));
}

#[test]
fn test_examples_recursive() {
    assert_eq!(2, compute_fuel_recursive(12));
    assert_eq!(2, compute_fuel_recursive(14));
    assert_eq!(966, compute_fuel_recursive(1969));
    assert_eq!(50346, compute_fuel_recursive(100756));
}

fn compute_fuel(mass: u64) -> u64 {
    let divided = mass / 3;
    if divided <= 2 {
        0
    } else {
        divided - 2
    }
}

fn compute_fuel_recursive(mass: u64) -> u64 {
    if mass == 0 {
        return 0;
    }

    let fuel = compute_fuel(mass);

    fuel + compute_fuel_recursive(fuel)
}

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("At least one command line argument is required.");

    let input = std::fs::File::open(path).expect("Could not open input file");
    let reader = std::io::BufReader::new(input);

    let sum = reader
        .lines()
        .map(|l| str::parse::<u64>(l.unwrap().as_ref()).unwrap())
        // Swap this for compute_fuel in case you want to get the answer for part 1.
        .map(compute_fuel_recursive)
        .fold(0, |acc, x| acc + x);

    println!("Required fuel is {}", sum);
}
