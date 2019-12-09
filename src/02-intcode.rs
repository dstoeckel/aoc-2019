use std::io::Read;

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

    for noun in 0..99 {
        for verb in 0..99 {
            let mut memory = instructions.clone();

            memory[1] = noun;
            memory[2] = verb;

            if intcode::evaluate(memory) == 19690720 {
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
