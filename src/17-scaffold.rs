fn print_map(map: &Vec<isize>) {
    let output = String::from_utf8(map.iter().map(|x| *x as u8).collect()).unwrap();

    println!("{}", output);
}

fn dimensions(map: &Vec<isize>) -> (usize, usize) {
    let width = map.iter().position(|&x| x == 10).unwrap();
    let height = map.len() / (width + 1);

    (width, height)
}

fn compute_intersections(map: &Vec<isize>) -> Vec<(usize, usize)> {
    let (width, height) = dimensions(map);

    let mut intersections = Vec::new();
    for i in 1..(height - 1) {
        for j in 1..(width - 1) {
            if map[i * (width + 1) + j] == 35 {
                if map[(i + 1) * (width + 1) + j] == 35
                    && map[(i - 1) * (width + 1) + j] == 35
                    && map[i * (width + 1) + j + 1] == 35
                    && map[i * (width + 1) + j - 1] == 35
                {
                    intersections.push((i, j));
                }
            }
        }
    }

    intersections
}

fn trace_line(
    map: &Vec<isize>,
    i: usize,
    j: usize,
    width: usize,
    height: usize,
    dir_i: isize,
    dir_j: isize,
) -> (usize, usize, usize) {
    let mut i = i as isize;
    let mut j = j as isize;
    let mut steps = 0;
    loop {
        if i + dir_i < 0 || i + dir_i >= height as isize {
            break;
        }

        if j + dir_j < 0 || j + dir_j >= width as isize {
            break;
        }

        if map[(i + dir_i) as usize * (width + 1) + (j + dir_j) as usize] != 35 {
            break;
        }

        i += dir_i;
        j += dir_j;

        steps += 1;
    }

    (i as usize, j as usize, steps)
}

fn trace(map: &Vec<isize>) -> Vec<isize> {
    let (width, height) = dimensions(map);

    let robot_pos = map
        .iter()
        .position(|x| match x {
            60 | 62 | 94 | 118 => true,
            _ => false,
        })
        .unwrap();
    let mut robot_dir = map[robot_pos];
    let mut i = robot_pos / (width + 1);
    let mut j = robot_pos % (width + 1);

    let mut program = Vec::new();
    loop {
        // 60 = <, 62 = >, 94 = ^, 118 = v, 35 = #, R=82, L=76
        let trace = if robot_dir == 94 {
            if map[i * (width + 1) + j + 1] == 35 {
                program.push(82);
                robot_dir = 62;
                trace_line(map, i, j, width, height, 0, 1)
            } else if j > 0 && map[i * (width + 1) + j - 1] == 35 {
                program.push(76);
                robot_dir = 60;
                trace_line(map, i, j, width, height, 0, -1)
            } else {
                break;
            }
        } else if robot_dir == 118 {
            if map[i * (width + 1) + j + 1] == 35 {
                program.push(76);
                robot_dir = 62;
                trace_line(map, i, j, width, height, 0, 1)
            } else if j > 0 && map[i * (width + 1) + j - 1] == 35 {
                program.push(82);
                robot_dir = 60;
                trace_line(map, i, j, width, height, 0, -1)
            } else {
                break;
            }
        } else if robot_dir == 60 {
            if i > 0 && map[(i - 1) * (width + 1) + j] == 35 {
                program.push(82);
                robot_dir = 94;
                trace_line(map, i, j, width, height, -1, 0)
            } else if i < height - 1 && map[(i + 1) * (width + 1) + j] == 35 {
                program.push(76);
                robot_dir = 118;
                trace_line(map, i, j, width, height, 1, 0)
            } else {
                break;
            }
        } else {
            if i > 0 && map[(i - 1) * (width + 1) + j] == 35 {
                program.push(76);
                robot_dir = 94;
                trace_line(map, i, j, width, height, -1, 0)
            } else if i < height - 1 && map[(i + 1) * (width + 1) + j] == 35 {
                program.push(82);
                robot_dir = 118;
                trace_line(map, i, j, width, height, 1, 0)
            } else {
                break;
            }
        };

        i = trace.0;
        j = trace.1;
        program.extend(
            format!(",{},", trace.2)
                .as_bytes()
                .iter()
                .map(|&x| x as isize),
        );
    }

    program
}

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("At least one command line argument is required.");

    let mut instructions = intcode::read_intcode_file(&path);

    let mut io = intcode::BufIo::new(&[]);
    intcode::evaluate_io(instructions.clone(), &mut io);
    print_map(io.output());

    let intersections = compute_intersections(io.output());

    let alignment = intersections.iter().map(|&(i, j)| i * j).sum::<usize>();
    println!("The aligment is {}", alignment);
    let path = String::from_utf8(trace(io.output()).iter().map(|&x| x as u8).collect()).unwrap();
    println!("The path is {}", path);

    //              11111111112
    //     12345678901234567890
    // A = R,4,R,12,R,10,L,12
    // B = L,12,R,4,R,12
    // C = L,12,L,8,R,10,
    //     A,B,B,C,C,A,B,B,C,A

    let buffer = "A,B,B,C,C,A,B,B,C,A
R,4,R,12,R,10,L,12
L,12,R,4,R,12
L,12,L,8,R,10
n\n";

    let input = buffer
        .as_bytes()
        .iter()
        .map(|&x| x as isize)
        .collect::<Vec<_>>();
    let mut io = intcode::BufIo::new(input.as_slice());

    instructions[0] = 2;

    intcode::evaluate_io(instructions, &mut io);
    println!("Dust collected {}", io.output()[io.output().len() - 1]);
}
