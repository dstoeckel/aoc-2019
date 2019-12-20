fn evaluate_beam(instructions: Vec<isize>, i: isize, j: isize) -> isize {
    let tmp = [i, j];
    let mut io = intcode::BufIo::new(&tmp);
    intcode::evaluate_io(instructions, &mut io);

    io.get(0)
}

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("At least one command line argument is required.");

    let instructions = intcode::read_intcode_file(&path);

    let mut sum = 0;
    for i in 0..50 {
        for j in 0..50 {
            sum += evaluate_beam(instructions.clone(), j, i);
        }
    }
    println!("The mount of affected fields is {}", sum);

    let mut min_j = 0;
    let mut max_j = 0;
    for i in 10..1000 {
        let mut j = min_j;
        while evaluate_beam(instructions.clone(), j, i) == 0 {
            j += 1;
        }

        min_j = j;
        j = std::cmp::max(max_j, j);

        while evaluate_beam(instructions.clone(), j, i) == 1 {
            j += 1;
        }

        max_j = j;

        if max_j - min_j < 100 {
            continue;
        }

        let mut all_covered = true;
        for j in (max_j - 100)..max_j {
            if evaluate_beam(instructions.clone(), j, i + 99) == 0 {
                all_covered = false;
                break;
            }
        }

        if all_covered {
            println!(
                "The first row that fits Santa's ship is {} ({}, {}). Code: {}",
                i,
                min_j,
                max_j,
                (max_j - 100) * 10000 + i
            );
            break;
        }
    }
}
