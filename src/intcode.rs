use std::io::Write;

#[test]
fn test_examples() {
    let instr = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];

    assert_eq!(3500, evaluate(instr));
}

#[test]
fn test_examples_2() {
    let instr = vec![1002, 4, 3, 4, 33];
    assert_eq!(1002, evaluate(instr));
}

#[test]
fn test_examples_3() {
    let instr = vec![1101, 100, -1, 4, 0];
    assert_eq!(1101, evaluate(instr));
}

#[test]
fn test_examples_4() {
    let instr = vec![3, 0, 4, 0, 99];
    let mut input = std::io::BufReader::new("10\n".as_bytes());
    assert_eq!(10, evaluate_impl(instr, &mut input));
}

#[test]
fn test_decode() {
    assert_eq!([1, 0, 0, 0], decode_opcode(1));
    assert_eq!([99, 1, 0, 1], decode_opcode(10199))
}

fn decode_opcode(mut op: isize) -> [u8; 4] {
    let mut result = [0u8; 4];

    let mut div = 10000;
    for i in 0..3 {
        let tmp = op / div;
        op -= tmp * div;
        div /= 10;
        result[3 - i] = tmp as u8;
    }

    result[0] = op as u8;

    result
}

fn load_argument(instructions: &Vec<isize>, pos: usize, mode: u8) -> isize {
    let value = instructions[pos];

    match mode {
        0 => {
            if value < 0 {
                panic!("Encountered negative position!");
            }
            instructions[value as usize]
        }
        1 => value,
        _ => panic!("Unhandled parameter mode!"),
    }
}

fn store(instructions: &mut Vec<isize>, pos: usize, value: isize) {
    let pos = instructions[pos];

    if pos < 0 {
        panic!("Encountered negative position!");
    }

    instructions[pos as usize] = value;
}

fn load_ptr(instructions: &Vec<isize>, pos: usize, mode: u8) -> usize {
    let ptr = load_argument(&instructions, pos, mode);

    if ptr < 0 {
        panic!("Invalid instruction pointer!")
    }
    ptr as usize
}

pub(crate) fn evaluate(instructions: Vec<isize>) -> isize {
    evaluate_impl(instructions, &mut std::io::BufReader::new(std::io::stdin()))
}

fn evaluate_impl<R: std::io::BufRead>(mut instructions: Vec<isize>, reader: &mut R) -> isize {
    let mut i = 0;
    while i < instructions.len() {
        let opcode = decode_opcode(instructions[i]);

        let stride = match opcode[0] {
            1 => {
                let s1 = load_argument(&instructions, i + 1, opcode[1]);
                let s2 = load_argument(&instructions, i + 2, opcode[2]);
                store(&mut instructions, i + 3, s1 + s2);
                4
            }
            2 => {
                let s1 = load_argument(&instructions, i + 1, opcode[1]);
                let s2 = load_argument(&instructions, i + 2, opcode[2]);
                store(&mut instructions, i + 3, s1 * s2);
                4
            }
            3 => {
                print!("Input: ");
                std::io::stdout().flush().unwrap();
                let mut buffer = String::new();
                reader.read_line(&mut buffer).unwrap();
                let input = str::parse::<isize>(&buffer.trim()).unwrap();

                store(&mut instructions, i + 1, input);
                2
            }
            4 => {
                println!("{}", load_argument(&instructions, i + 1, opcode[1]));
                2
            }
            5 => {
                if load_argument(&instructions, i + 1, opcode[1]) != 0 {
                    i = load_ptr(&instructions, i + 2, opcode[2]);
                    0
                } else {
                    3
                }
            }
            6 => {
                if load_argument(&instructions, i + 1, opcode[1]) == 0 {
                    i = load_ptr(&instructions, i + 2, opcode[2]);
                    0
                } else {
                    3
                }
            }
            7 => {
                let s1 = load_argument(&instructions, i + 1, opcode[1]);
                let s2 = load_argument(&instructions, i + 2, opcode[2]);

                store(&mut instructions, i + 3, (s1 < s2) as isize);
                4
            }
            8 => {
                let s1 = load_argument(&instructions, i + 1, opcode[1]);
                let s2 = load_argument(&instructions, i + 2, opcode[2]);

                store(&mut instructions, i + 3, (s1 == s2) as isize);
                4
            }
            99 => break,
            o => panic!("Unhandled opcode {}", o),
        };

        i += stride;
    }

    instructions[0]
}
