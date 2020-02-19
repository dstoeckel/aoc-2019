fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("At least one command line argument is required.");

    let instructions = intcode::read_intcode_file(&path);
    intcode::evaluate_io(instructions, &mut intcode::AsciiIo::new());
}
