use std::collections::HashMap;
use std::io::BufRead;

#[test]
fn test_methods_orbits() {
    let mut orbits = HashMap::new();
    orbits.insert("B".to_string(), "COM".to_string());
    orbits.insert("C".to_string(), "B".to_string());
    orbits.insert("D".to_string(), "C".to_string());
    orbits.insert("E".to_string(), "D".to_string());
    orbits.insert("F".to_string(), "E".to_string());
    orbits.insert("G".to_string(), "B".to_string());
    orbits.insert("H".to_string(), "G".to_string());
    orbits.insert("I".to_string(), "D".to_string());
    orbits.insert("J".to_string(), "E".to_string());
    orbits.insert("K".to_string(), "J".to_string());
    orbits.insert("L".to_string(), "K".to_string());

    let mut counts = HashMap::new();
    assert_eq!(42, count_all_orbits(&orbits, &mut counts));
    assert_eq!(4, compute_transfers("K", "I", &orbits, &mut counts));
}

fn count_orbits(
    body: &str,
    orbits: &HashMap<String, String>,
    counts: &mut HashMap<String, usize>,
) -> usize {
    if counts.contains_key(body) {
        return *counts.get(body).unwrap();
    }

    let result = if orbits.contains_key(body) {
        let next = orbits.get(body).unwrap();
        1 + count_orbits(&next, orbits, counts)
    } else {
        0
    };

    counts.insert(body.to_string(), result);

    result
}

fn count_all_orbits(
    orbits: &HashMap<String, String>,
    counts: &mut HashMap<String, usize>,
) -> usize {
    orbits
        .keys()
        .map(|body| count_orbits(body, orbits, counts))
        .sum()
}

fn compute_transfers<'a>(
    mut a: &'a str,
    mut b: &'a str,
    orbits: &'a HashMap<String, String>,
    counts: &HashMap<String, usize>,
) -> usize {
    let mut transfers = 0;
    let mut depth_a = counts.get(a).unwrap();
    let mut depth_b = counts.get(a).unwrap();

    while a != b {
        if depth_a <= depth_b {
            b = orbits.get(b).unwrap();
            depth_b = counts.get(b).unwrap();
        } else {
            a = orbits.get(a).unwrap();
            depth_a = counts.get(a).unwrap();
        }

        transfers += 1;
    }

    transfers
}

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("At least one command line argument is required.");

    let input = std::fs::File::open(path).expect("Could not open input file");
    let reader = std::io::BufReader::new(input);

    let orbits: HashMap<String, String> = reader
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let mut it = l.split(")");
            let a = it.next().unwrap().to_string();
            let b = it.next().unwrap().to_string();
            (b, a)
        })
        .collect();

    let mut num_orbits = HashMap::new();
    println!(
        "The total number of orbits is {}",
        count_all_orbits(&orbits, &mut num_orbits)
    );

    let a = orbits
        .get("YOU")
        .expect("This program expects that there is an entry called YOU.");
    let b = orbits
        .get("SAN")
        .expect("This program expects that there is an entry called SAN.");

    println!(
        "Number of needed transfers is {}",
        compute_transfers(a, b, &orbits, &num_orbits)
    );
}
