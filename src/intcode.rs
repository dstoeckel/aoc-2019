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
    let mut io = BufIo::new(&[10]);
    assert_eq!(10, evaluate_io(instr, &mut io));
}

#[test]
fn test_decode() {
    assert_eq!([1, 0, 0, 0], decode_opcode(1));
    assert_eq!([99, 1, 0, 1], decode_opcode(10199))
}

#[test]
fn test_relative() {
    let instr = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
    let instr_ = instr.clone();
    let mut io = BufIo::new(&[]);
    evaluate_io(instr, &mut io);

    for i in 0..instr_.len() {
        assert_eq!(instr_[i], io.get(i));
    }
}

#[test]
fn test_large_num() {
    let instr = vec![1102,34915192,34915192,7,4,7,99,0];
    let mut io = BufIo::new(&[]);
    evaluate_io(instr, &mut io);
    assert_eq!(true, io.get(0) >= 1_000_000_000_000_000);
}

#[test]
fn test_large_num_2() {
    let instr = vec![104,1125899906842624,99];
    let mut io = BufIo::new(&[]);
    evaluate_io(instr, &mut io);
    assert_eq!(1125899906842624, io.get(0));
}

pub trait Io {
    fn input(&mut self) -> isize;
    fn output(&mut self, o: isize);
}

struct StdIo;

impl Io for StdIo {
    fn input(&mut self) -> isize {
        print!("Input: ");
        std::io::stdout().flush().unwrap();
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        str::parse::<isize>(&buffer.trim()).unwrap()
    }

    fn output(&mut self, o: isize) {
        println!("{}", o);
    }
}

pub struct ChannelIo {
    sender: std::sync::mpsc::Sender<isize>,
    receiver: std::sync::mpsc::Receiver<isize>,
    last_output: Option<isize>,
}

impl ChannelIo {
    pub fn new(
        sender: std::sync::mpsc::Sender<isize>,
        receiver: std::sync::mpsc::Receiver<isize>,
    ) -> ChannelIo {
        ChannelIo {
            sender,
            receiver,
            last_output: None,
        }
    }

    pub fn last(&self) -> Option<isize> {
        self.last_output
    }
}

impl Io for ChannelIo {
    fn input(&mut self) -> isize {
        self.receiver.recv().unwrap()
    }

    fn output(&mut self, o: isize) {
        self.last_output = Some(o);

        let _ = self.sender.send(o);
    }
}

pub struct BufIo<'a> {
    buf_in: &'a [isize],
    buf_out: Vec<isize>,
    cursor_in: usize,
}

impl<'a> BufIo<'a> {
    pub fn new(input: &'a [isize]) -> BufIo<'a> {
        BufIo {
            buf_in: input,
            buf_out: Vec::new(),
            cursor_in: 0,
        }
    }

    pub fn get(&self, i: usize) -> isize {
        self.buf_out[i]
    }

    pub fn len(&self) -> usize {
        self.buf_out.len()
    }
}

impl<'a> Io for BufIo<'a> {
    fn input(&mut self) -> isize {
        let result = self.buf_in[self.cursor_in];
        self.cursor_in += 1;
        result
    }

    fn output(&mut self, o: isize) {
        self.buf_out.push(o);
    }
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

fn load_argument(instructions: &Vec<isize>, pos: usize, mode: u8, base: isize) -> isize {
    let value = instructions[pos];

    match mode {
        0 => {
            if value < 0 {
                panic!("Encountered negative position!");
            }
            instructions[value as usize]
        }
        1 => value,
        2 => instructions[(base + value) as usize],
        _ => panic!("Unhandled parameter mode!"),
    }
}

fn store(instructions: &mut Vec<isize>, pos: usize, value: isize, mode: u8, base: isize) {
    let pos = instructions[pos];

    match mode {
        0 => {
            if pos < 0 {
                panic!("Encountered negative position!");
            }
            instructions[pos as usize] = value;
        }
        1 => panic!("Absolute mode not supported for store operation"),
        2 => {
            instructions[(base + pos) as usize] = value;
        }
        _ => panic!("Unhandled parameter mode!")
    }
}

fn load_ptr(instructions: &Vec<isize>, pos: usize, mode: u8, base: isize) -> usize {
    let ptr = load_argument(&instructions, pos, mode, base);

    if ptr < 0 {
        panic!("Invalid instruction pointer!")
    }
    ptr as usize
}

pub(crate) fn evaluate(instructions: Vec<isize>) -> isize {
    evaluate_io(instructions, &mut StdIo {})
}

pub(crate) fn evaluate_io<T: Io>(mut instructions: Vec<isize>, io: &mut T) -> isize {
    let mut i = 0;
    let mut base = 0;

    instructions.extend([0; 1000].iter());
    while i < instructions.len() {
        let opcode = decode_opcode(instructions[i]);

        let stride = match opcode[0] {
            1 => {
                let s1 = load_argument(&instructions, i + 1, opcode[1], base);
                let s2 = load_argument(&instructions, i + 2, opcode[2], base);
                store(&mut instructions, i + 3, s1 + s2, opcode[3], base);
                4
            }
            2 => {
                let s1 = load_argument(&instructions, i + 1, opcode[1], base);
                let s2 = load_argument(&instructions, i + 2, opcode[2], base);
                store(&mut instructions, i + 3, s1 * s2, opcode[3], base);
                4
            }
            3 => {
                store(&mut instructions, i + 1, io.input(), opcode[1], base);
                2
            }
            4 => {
                io.output(load_argument(&instructions, i + 1, opcode[1], base));
                2
            }
            5 => {
                if load_argument(&instructions, i + 1, opcode[1], base) != 0 {
                    i = load_ptr(&instructions, i + 2, opcode[2], base);
                    0
                } else {
                    3
                }
            }
            6 => {
                if load_argument(&instructions, i + 1, opcode[1], base) == 0 {
                    i = load_ptr(&instructions, i + 2, opcode[2], base);
                    0
                } else {
                    3
                }
            }
            7 => {
                let s1 = load_argument(&instructions, i + 1, opcode[1], base);
                let s2 = load_argument(&instructions, i + 2, opcode[2], base);

                store(&mut instructions, i + 3, (s1 < s2) as isize, opcode[3], base);
                4
            }
            8 => {
                let s1 = load_argument(&instructions, i + 1, opcode[1], base);
                let s2 = load_argument(&instructions, i + 2, opcode[2], base);

                store(&mut instructions, i + 3, (s1 == s2) as isize, opcode[3], base);
                4
            }
            9 => {
                base += load_argument(&instructions, i + 1, opcode[1], base);
                2
            }
            99 => break,
            o => panic!("Unhandled opcode {}", o),
        };

        i += stride;
    }

    instructions[0]
}
