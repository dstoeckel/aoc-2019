fn print_output(map: &[isize]) {
    let output = String::from_utf8(map.iter().map(|x| *x as u8).collect()).unwrap();

    println!("{}", output);
}

fn run(instructions: Vec<isize>, buffer: &str) {
    let input = buffer
        .as_bytes()
        .iter()
        .map(|&x| x as isize)
        .collect::<Vec<_>>();
    let mut io = intcode::BufIo::new(input.as_slice());

    intcode::evaluate_io(instructions, &mut io);

    // Check that all output is ASCII. If not, the robot was successful and we can report the
    // damage assessment.
    if io.output().iter().all(|&x| x <= 127) {
        print_output(io.output());
    } else {
        println!("Damage to ship {}", io.get(io.output().len() - 1));
    }
}

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("At least one command line argument is required.");

    let instructions = intcode::read_intcode_file(&path);

    println!("Part 1");
    let buffer_1 = "NOT C J
AND D J
NOT A T
OR T J
WALK\n";
    run(instructions.clone(), buffer_1);

    println!("Part 2");
    let buffer_2 = "NOT C J
AND D J
AND H J
NOT A T
OR T J
NOT B T
AND D T
OR T J
RUN\n";

    run(instructions, buffer_2);
}
