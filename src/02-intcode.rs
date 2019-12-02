use std::io::Read;

#[test]
fn test_examples() {
    let instr = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];

    assert_eq!(3500, evaluate(instr));
}

fn evaluate(mut instructions: Vec<usize>) -> usize {
    let mut i = 0;
    while i < instructions.len() {
        let opcode = instructions[i];

        if opcode == 99 {
            break;
        }

        let s1 = instructions[instructions[i + 1]];
        let s2 = instructions[instructions[i + 2]];
        let t = instructions[i + 3];

        instructions[t] = match opcode {
            1 => s1 + s2,
            2 => s1 * s2,
            o => panic!("Unhandled opcode {}", o),
        };

        i += 4;
    }

    instructions[0]
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
        .map(|x| str::parse::<usize>(x).unwrap())
        .collect::<Vec<_>>();

    for noun in 0..99 {
        for verb in 0..99 {
            let mut memory = instructions.clone();

            memory[1] = noun;
            memory[2] = verb;

            if evaluate(memory) == 19690720 {
                println!(
                    "Found noun = {} and verb = {}. Combined output is {}",
                    noun,
                    verb,
                    100 * noun + verb
                );
                break;
            }
        }
    }
}
