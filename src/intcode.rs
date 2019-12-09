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
    let instr = vec![
        109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
    ];
    let instr_ = instr.clone();
    let mut io = BufIo::new(&[]);
    evaluate_io(instr, &mut io);

    for i in 0..instr_.len() {
        assert_eq!(instr_[i], io.get(i));
    }
}

#[test]
fn test_large_num() {
    let instr = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
    let mut io = BufIo::new(&[]);
    evaluate_io(instr, &mut io);
    assert_eq!(true, io.get(0) >= 1_000_000_000_000_000);
}

#[test]
fn test_large_num_2() {
    let instr = vec![104, 1125899906842624, 99];
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

pub fn evaluate_io(instructions: Vec<isize>, io: &mut dyn Io) -> isize {
    let mut interpreter = Intcode::new(instructions);

    let mut input = 0;
    loop {
        match interpreter.step(input) {
            State::Input => input = io.input(),
            State::Output(o) => io.output(o),
            State::Terminated => break,
        }
    }
    interpreter.first_cell()
}

pub fn evaluate(instructions: Vec<isize>) -> isize {
    let mut io = StdIo {};
    evaluate_io(instructions, &mut io)
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

pub enum State {
    Terminated,
    Output(isize),
    Input,
}

pub struct Intcode {
    instructions: Vec<isize>,
    base: isize,
    iptr: usize,
    input_requested: bool,
}

impl Intcode {
    pub fn new(mut instructions: Vec<isize>) -> Intcode {
        instructions.extend([0; 1000].iter());
        Intcode {
            instructions,
            base: 0,
            iptr: 0,
            input_requested: false,
        }
    }

    pub fn first_cell(&self) -> isize {
        self.instructions[0]
    }

    fn decode_opcode(&self) -> [u8; 4] {
        decode_opcode(self.instructions[self.iptr])
    }

    fn load_argument(&self, pos: usize, mode: &[u8; 4]) -> isize {
        let value = self.instructions[self.iptr + pos];

        match mode[pos] {
            0 => {
                if value < 0 {
                    panic!("Encountered negative position!");
                }
                self.instructions[value as usize]
            }
            1 => value,
            2 => self.instructions[(self.base + value) as usize],
            _ => panic!("Unhandled parameter mode!"),
        }
    }

    fn store(&mut self, pos: usize, value: isize, opcode: &[u8; 4]) {
        let address = self.instructions[self.iptr + pos];

        match opcode[pos] {
            0 => {
                if address < 0 {
                    panic!("Encountered negative position!");
                }
                self.instructions[address as usize] = value;
            }
            1 => panic!("Absolute mode not supported for store operation"),
            2 => {
                self.instructions[(self.base + address) as usize] = value;
            }
            _ => panic!("Unhandled parameter mode!"),
        }
    }

    fn load_ptr(&self, pos: usize, opcode: &[u8; 4]) -> usize {
        let ptr = self.load_argument(pos, opcode);

        if ptr < 0 {
            panic!("Invalid instruction pointer!")
        }
        ptr as usize
    }

    pub fn step(&mut self, input: isize) -> State {
        while self.iptr < self.instructions.len() {
            let opcode = self.decode_opcode();

            let stride = match opcode[0] {
                1 => {
                    let s1 = self.load_argument(1, &opcode);
                    let s2 = self.load_argument(2, &opcode);
                    self.store(3, s1 + s2, &opcode);
                    4
                }
                2 => {
                    let s1 = self.load_argument(1, &opcode);
                    let s2 = self.load_argument(2, &opcode);
                    self.store(3, s1 * s2, &opcode);
                    4
                }
                3 => {
                    if self.input_requested {
                        self.store(1, input, &opcode);
                        self.input_requested = false;
                        2
                    } else {
                        self.input_requested = true;
                        return State::Input;
                    }
                }
                4 => {
                    let result = self.load_argument(1, &opcode);
                    self.iptr += 2;
                    return State::Output(result);
                }
                5 => {
                    if self.load_argument(1, &opcode) != 0 {
                        self.iptr = self.load_ptr(2, &opcode);
                        0
                    } else {
                        3
                    }
                }
                6 => {
                    if self.load_argument(1, &opcode) == 0 {
                        self.iptr = self.load_ptr(2, &opcode);
                        0
                    } else {
                        3
                    }
                }
                7 => {
                    let s1 = self.load_argument(1, &opcode);
                    let s2 = self.load_argument(2, &opcode);

                    self.store(3, (s1 < s2) as isize, &opcode);
                    4
                }
                8 => {
                    let s1 = self.load_argument(1, &opcode);
                    let s2 = self.load_argument(2, &opcode);

                    self.store(3, (s1 == s2) as isize, &opcode);
                    4
                }
                9 => {
                    self.base += self.load_argument(1, &opcode);
                    2
                }
                99 => return State::Terminated,
                o => panic!("Unhandled opcode {}", o),
            };

            self.iptr += stride;
        }

        panic!("Reached end of memory!");
    }
}
