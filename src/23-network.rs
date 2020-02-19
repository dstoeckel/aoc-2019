use intcode::*;
use std::collections::VecDeque;

struct Node {
    id: isize,
    interpreter: intcode::Intcode,
    iqueue: VecDeque<(isize, isize)>,
    current_state: intcode::State,
}

impl Node {
    fn new(id: isize, instructions: Vec<isize>) -> Node {
        let mut interpreter = Intcode::new(instructions);

        if interpreter.step(-1) != State::Input {
            panic!("Expected input instruction!");
        }

        let current_state = interpreter.step(id);

        Node {
            id,
            interpreter,
            iqueue: VecDeque::new(),
            current_state,
        }
    }

    fn process_output(&mut self) -> Option<[isize; 3]> {
        if let State::Output(o) = self.current_state {
            let mut output = [0; 3];
            output[0] = o;
            output[1] = match self.interpreter.step(-1) {
                State::Output(o) => o,
                _ => panic!("Expected output for node {}", self.id),
            };

            output[2] = match self.interpreter.step(-1) {
                State::Output(o) => o,
                _ => panic!("Expected output for node {}", self.id),
            };

            self.current_state = self.interpreter.step(-1);
            Some(output)
        } else {
            None
        }
    }

    fn process_input(&mut self) -> bool {
        let mut iter = 0;
        let mut is_idle = true;
        while let State::Input = self.current_state {
            if self.iqueue.is_empty() {
                if iter == 0 {
                    self.current_state = self.interpreter.step(-1);
                }
                break;
            }
            is_idle = false;
            iter += 1;

            let input = self.iqueue.pop_front().unwrap();

            if self.interpreter.step(input.0) != State::Input {
                panic!("Expected input state for node {}", self.id);
            }

            self.current_state = self.interpreter.step(input.1);
        }

        is_idle
    }
}

fn run(network: &mut Vec<Node>) {
    let mut nat = None;
    let mut old = None;

    loop {
        let mut is_idle = true;
        for node in 0..50 {
            while let Some(o) = network[node].process_output() {
                is_idle = false;
                if o[0] == 255 {
                    println!("Package sent to 255. Y value is {}", o[2]);
                    nat = Some((o[1], o[2]));
                    break;
                }

                network[o[0] as usize].iqueue.push_back((o[1], o[2]));
            }

            if !network[node].process_input() {
                is_idle = false;
            }
        }

        if let Some(x) = nat {
            if is_idle {
                if nat == old {
                    println!(
                        "Delivering the same package to address 0 twice in a row {:?}",
                        x
                    );
                    return;
                }
                network[0].iqueue.push_back(x);
                old = nat;
            }
        }
    }
}

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("At least one command line argument is required.");

    let instructions = intcode::read_intcode_file(&path);
    let mut network = (0..50)
        .map(|i| Node::new(i, instructions.clone()))
        .collect();
    run(&mut network);
}
