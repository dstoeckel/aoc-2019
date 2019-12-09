use std::io::Read;

mod intcode;

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

    let mut io = intcode::BufIo::new(&[1]);

    intcode::evaluate_io(instructions.clone(), &mut io);

    if io.len() > 1 {
        println!("ERROR: The following instructions do not work properly:");
        for i in 0..io.len() {
            println!("{}", io.get(i));
        }
    } else {
        println!("The BOOST keycode is {}", io.get(0));
    }

    let mut io = intcode::BufIo::new(&[2]);
    intcode::evaluate_io(instructions, &mut io);

    println!("The distress signal coordinates are {}", io.get(0));
}
