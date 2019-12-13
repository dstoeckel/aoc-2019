use std::io::{BufRead, Read, Write};
use intcode::State;

fn count_block_tiles(instructions: Vec<isize>) {
    let mut io = intcode::BufIo::new(&[]);
    intcode::evaluate_io(instructions, &mut io);

    let mut i = 0;
    let mut count = 0;
    while i < io.len() {
        count += (io.get(i + 2) == 2) as usize;
        i += 3;
    }
    println!("Number of block tiles is {}", count);
}

fn update_field(field: &mut [char; 43 * 23], buf: &[isize; 3]) {
    let symbol = match buf[2] {
        0 => ' ',
        1 => '█',
        2 => '═',
        3 => '☐',
        4 => '●',
        _ => panic!("Unhandled tile type!"),
    };

    field[(buf[0] + buf[1]*43) as usize] = symbol;
}

fn print_game(field: &[char; 43 * 23], score: isize) {
    println!("\033[2J\033[;HScore: {}", score);

    let mut idx = 0;
    for _ in 0..23 {
        println!("{}", field[idx..(idx + 43)].iter().collect::<String>());
        idx += 43;
    }
}

fn cheat(instructions: &Vec<isize>) {
    let mut input_sequence = Vec::new();

    if let Ok(input_file) = std::fs::File::open("arcade_dump.txt") {
        for l in std::io::BufReader::new(input_file).lines() {
            input_sequence.push(str::parse(&l.unwrap()).unwrap());
        }
    }

    loop {
        let result = play_game(instructions.clone(), &mut input_sequence);

        {
            let mut output = std::fs::File::create("arcade_dump.txt").unwrap();
            for input in input_sequence.iter() {
                writeln!(output, "{}", input).unwrap();
            }
        }

        match result {
            None => {
                println!("You played {} moves.", input_sequence.len());
                let mut buffer = String::new();
                std::io::stdin().read_line(&mut buffer).unwrap();
                std::io::stdin().read_line(&mut buffer).unwrap();
                let time: usize = loop {
                    if let Ok(time) = str::parse(&buffer.trim()) {
                        break time
                    }
                };

                input_sequence.truncate(input_sequence.len() - time);
            },
            Some(score) => {
                println!("You beat the game with a score of {}", score);
                break;
            }
        }
    }
}

fn play_game(mut instructions: Vec<isize>, input_sequence: &mut Vec<isize>) -> Option<isize> {
    instructions[0] = 2;
    let mut interpreter = intcode::Intcode::new(instructions);

    let mut input = 0;
    let mut input_buffer = [0; 1];
    let mut output_buffer = [0; 3];

    let mut n_output = 0;
    let mut field = [' '; 43 * 23];
    let mut score = 0;

    let mut moves = 0;
    loop {
        match interpreter.step(input) {
            State::Input => {
                if moves < input_sequence.len() {
                    input = input_sequence[moves];
                } else {
                    print_game(&field, score);
                    loop {
                        std::io::stdin().read(&mut input_buffer).unwrap();
                        input = match input_buffer[0] {
                            97 => -1,
                            115 => 0,
                            100 => 1,
                            _ => continue,
                        };
                        input_sequence.push(input);
                        break;
                    }
                }

                moves += 1;
            }
            State::Output(o) => {
                output_buffer[n_output] = o;

                if n_output == 2 {
                    if output_buffer[0] == -1 && output_buffer[1] == 0 {
                        score = output_buffer[2];
                    } else {
                        update_field(&mut field, &output_buffer);
                    }
                    n_output = 0;
                } else {
                    n_output += 1;
                }
            }
            State::Terminated => {
                println!("{}", score);
                let result = if field.iter().filter(|c| **c == ' ').count() == 0 {
                    Some(score)
                } else {
                    None
                };

                return result;
            }
        }
    }
}

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("At least one command line argument is required.");

    let instructions = intcode::read_incode_file(&path);
    count_block_tiles(instructions.clone());
    cheat(&instructions);
}
