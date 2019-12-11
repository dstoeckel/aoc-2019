use intcode::*;
use std::io::Read;

#[test]
fn test_next_permutation() {
    let mut data = [0, 1, 2];
    assert_eq!(true, next_permutation(&mut data));
    assert_eq!([0, 2, 1], data);
    assert_eq!(true, next_permutation(&mut data));
    assert_eq!([1, 0, 2], data);
    assert_eq!(true, next_permutation(&mut data));
    assert_eq!([1, 2, 0], data);
    assert_eq!(true, next_permutation(&mut data));
    assert_eq!([2, 0, 1], data);
    assert_eq!(true, next_permutation(&mut data));
    assert_eq!([2, 1, 0], data);
    assert_eq!(false, next_permutation(&mut data));
    assert_eq!([0, 1, 2], data);
}

#[test]
fn test_compute_amplification1() {
    let instructions = vec![
        3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
    ];
    let setting = [4, 3, 2, 1, 0];

    assert_eq!(43210, compute_amplification(&instructions, &setting))
}

#[test]
fn test_compute_amplification2() {
    let instructions = vec![
        3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99,
        0, 0,
    ];
    let setting = [0, 1, 2, 3, 4];

    assert_eq!(54321, compute_amplification(&instructions, &setting))
}

#[test]
fn test_compute_amplification3() {
    let instructions = vec![
        3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1, 33,
        31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
    ];
    let setting = [1, 0, 4, 3, 2];

    assert_eq!(65210, compute_amplification(&instructions, &setting))
}

#[test]
fn test_compute_feedback_amplification1() {
    let instructions = vec![
        3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1, 28,
        1005, 28, 6, 99, 0, 0, 5,
    ];
    let setting = [9, 8, 7, 6, 5];

    assert_eq!(
        139629729,
        compute_feedback_amplification(&instructions, &setting)
    );
}

#[test]
fn test_compute_feedback_amplification2() {
    let instructions = vec![
        3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54, -5,
        54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4, 53,
        1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
    ];
    let setting = [9, 7, 8, 5, 6];
    assert_eq!(
        18216,
        compute_feedback_amplification(&instructions, &setting)
    );
}

fn next_permutation(perm: &mut [isize]) -> bool {
    if perm.len() < 2 {
        return false;
    }

    let mut i = perm.len() - 1;

    loop {
        i -= 1;
        if perm[i] < perm[i + 1] {
            let mut i2 = perm.len() - 1;
            while perm[i] >= perm[i2] {
                i2 -= 1;
            }

            perm.swap(i, i2);
            let last = perm.len();
            perm[(i + 1)..last].reverse();
            return true;
        }
        if i == 0 {
            perm.reverse();
            return false;
        }
    }
}

fn compute_amplification(instructions: &Vec<isize>, input: &[isize]) -> isize {
    let mut output = 0;
    for boost in input {
        let data = [*boost, output];
        let mut io = BufIo::new(&data);
        evaluate_io(instructions.clone(), &mut io);
        output = io.get(0);
    }

    output
}

fn initialize(instructions: &Vec<isize>, input: isize) -> Intcode {
    let mut icode = Intcode::new(instructions.clone());

    assert!(icode.step(0) == State::Input);
    assert!(icode.step(input) == State::Input);

    icode
}

fn feed_input(icode: &mut Intcode, input: isize) -> isize {
    let output = match icode.step(input) {
        State::Output(o) => o,
        _ => panic!("Unexpected state"),
    };

    match icode.step(0) {
        State::Input | State::Terminated => output,
        _ => panic!("Unexpected State"),
    }
}

fn compute_feedback_amplification(instructions: &Vec<isize>, input: &[isize]) -> isize {
    let mut amplifiers: Vec<_> = input.iter().map(|i| initialize(instructions, *i)).collect();

    let mut data = 0;
    let mut i = 0;
    while !amplifiers[i].is_terminated() {
        data = feed_input(&mut amplifiers[i], data);
        i = (i + 1) % amplifiers.len();
    }

    data
}

fn best_simple_amplifier_setting(instructions: &Vec<isize>) {
    let mut input: Vec<isize> = (0..5).collect();

    let mut best_val = compute_amplification(instructions, &input);
    let mut best = input.clone();

    while next_permutation(input.as_mut_slice()) {
        let val = compute_amplification(instructions, &input);
        if val > best_val {
            best_val = val;
            best = input.clone();
        }
    }

    println!("Configuration {:?} achieved best value {}", best, best_val);
}

fn best_feedback_amplifier_setting(instructions: &Vec<isize>) {
    let mut input: Vec<isize> = (5..10).collect();

    let mut best_val = compute_feedback_amplification(instructions, &input);
    let mut best = input.clone();

    while next_permutation(input.as_mut_slice()) {
        let val = compute_feedback_amplification(instructions, &input);
        if val > best_val {
            best_val = val;
            best = input.clone();
        }
    }

    println!(
        "Configuration {:?} achieved best amplification value {}",
        best, best_val
    );
}
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

    best_simple_amplifier_setting(&instructions);
    best_feedback_amplifier_setting(&instructions);
}
