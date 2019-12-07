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
    evaluate_io(instructions, &mut StdIo {})
}

pub(crate) fn evaluate_io<T: Io>(mut instructions: Vec<isize>, io: &mut T) -> isize {
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
                store(&mut instructions, i + 1, io.input());
                2
            }
            4 => {
                io.output(load_argument(&instructions, i + 1, opcode[1]));
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
