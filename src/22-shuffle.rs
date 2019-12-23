use std::io::BufRead;

#[test]
fn test_shuffle_quick_exponent() {
    assert_eq!(6i128.pow(23) % 7, quick_exponent(6, 23, 7))
}

#[test]
fn test_shuffle_solution_part1() {
    let reduced = (7101, -1726);
    assert_eq!(5169, execute(2019, reduced, 1, 10007));
    assert_eq!(7674, execute(2019, reduced, 2, 10007));
}

#[test]
fn test_shuffle_solution_part2() {
    let reduced = (2904709793044, 83085969022373);
    let dl = 119315717514047i128;
    assert_eq!(104129592782950, execute(2020, reduced, 1, dl));
    assert_eq!(48838925201242, execute(2020, reduced, 2, dl));
    assert_eq!(59784798628865, execute(2020, reduced, 3, dl));
    assert_eq!(73885750177030, execute(2020, reduced, 4, dl));
    assert_eq!(38390173073717, execute(2020, reduced, 5, dl));
}

#[test]
fn test_shuffle_solution_in_pos_part2() {
    let reduced = (2904709793044, 83085969022373);
    let dl = 119315717514047;
    let t = 101741582076661;
    assert_eq!(2020, in_position(104129592782950,reduced, 1, dl));
    assert_eq!(2020, in_position(48838925201242, reduced, 2, dl));
    assert_eq!(2020, in_position(59784798628865, reduced, 3, dl));
    assert_eq!(2020, in_position(73885750177030, reduced, 4, dl));
    assert_eq!(2020, in_position(38390173073717, reduced, 5, dl));
    assert_eq!(2020, in_position(44723323000907, reduced, t, dl));
}

enum Instruction {
    WithIncrement(i128),
    NewStack,
    Cut(i128),
}

fn parse_instruction(s: &str) -> Instruction {
    if s == "deal into new stack" {
        Instruction::NewStack
    } else if s.starts_with("cut ") {
        let i = s.split(' ').nth(1).unwrap().parse().unwrap();
        Instruction::Cut(i)
    } else if s.starts_with("deal with increment "){
        let i = s.split(' ').nth(3).unwrap().parse().unwrap();
        Instruction::WithIncrement(i)
    } else {
        panic!("Undandled instruction.")
    }
}

fn reduce(acc: (i128, i128), instr: &Instruction, deck_len: i128) -> (i128, i128) {
    match *instr {
        Instruction::Cut(n) => (acc.0, (acc.1 - n) % deck_len),
        Instruction::WithIncrement(n) => ((n * acc.0) % deck_len, (n * acc.1) % deck_len),
        Instruction::NewStack => (-acc.0, -(acc.1 + 1)),
    }
}

fn reduce_instructions(instr: &[Instruction], deck_len: i128) -> (i128, i128) {
    instr.iter().fold((1, 0), |acc, instr| reduce(acc, instr, deck_len))
}

fn quick_exponent(a: i128, e: i128, m: i128) -> i128 {
    if e == 0 {
        return 1;
    }

    let half = quick_exponent(a, e / 2, m);
    let mult = if e & 1 == 0 { 1 } else { a };

    mult * (half * half % m) % m
}

fn extended_euclid(mut a: i128, mut b: i128) -> (i128, i128, i128) {
    let (mut x0, mut x1, mut y0, mut y1) = (0, 1, 1, 0);

    while a > 0 {
        let q = b / a;
        let tmp = a;
        a = b % a;
        b = tmp;

        let tmp = y1;
        y1 = y0 - q * y1;
        y0 = tmp;

        let tmp = x1;
        x1 = x0 - q * x1;
        x0 = tmp;
    }

    (b, x0, y0)
}

// Executes the shuffeling algorithm n times.
// This uses the reduced representation to compute the result of a single execution as ax + b.
// Multiple exections are computed as a**nx + b \sum_i=0^{n-1} a^i. For this we use the Egyptian
// multiplication algorithm to compute the power of a and use the geometric series to compute the
// sum of coefficients. The needed multiplicative inverses are computed using the extended
// euclidean algorithm.
fn execute(x: i128, s: (i128, i128), times: i128, deck_len: i128) -> i128 {
    let a_n = quick_exponent(s.0, times, deck_len);

    let (_, inv, _) = extended_euclid(s.0 - 1, deck_len);
    let sum = (a_n - 1) * inv % deck_len;

    let res = (a_n * x + sum * s.1) % deck_len;
    if res < 0 { res + deck_len } else { res}
}

fn in_position(x: i128, s: (i128, i128), times: i128, deck_len: i128) -> i128 {
    let a_n = quick_exponent(s.0, times, deck_len);
    let (_, inv, _) = extended_euclid(s.0 - 1, deck_len);
    let sum = (a_n - 1) * inv % deck_len;

    let (_, pot_inv, _) = extended_euclid(a_n, deck_len);

    let res = ((x - sum * s.1) % deck_len) * pot_inv % deck_len;

    if res < 0 { res + deck_len } else {res}
}

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("At least one command line argument is required.");

    let input = std::fs::File::open(path).expect("Could not open input file");

    let instructions = std::io::BufReader::new(input)
        .lines()
        .map(|l| parse_instruction(&l.unwrap()))
        .collect::<Vec<_>>();

    let input = 2019i128;
    let short_deck_len = 10007;
    let reduced = reduce_instructions(&instructions, short_deck_len);
    println!("Part 1: Position of {} is {}", input, execute(input, reduced, 1, short_deck_len));

    let input = 2020i128;
    let long_deck_len = 119315717514047i128;
    let times = 101741582076661i128;
    let reduced = reduce_instructions(&instructions, long_deck_len);
    println!("Part 2: Position of {} is {}", input, execute(input, reduced, times, long_deck_len));
    println!("Part 2: Card in position {} is {}", input, in_position(input, reduced, times, long_deck_len));
}
