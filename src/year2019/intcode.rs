//! Implementation of the full Intcode computer specification.
use std::collections::VecDeque;

pub enum State {
    Input,
    Output(i64),
    Halted,
}

pub struct Computer {
    pc: usize,
    base: i64,
    code: Vec<i64>,
    input: VecDeque<i64>,
}

impl Computer {
    pub fn new(code: &[i64]) -> Computer {
        Computer { pc: 0, base: 0, code: code.to_vec(), input: VecDeque::new() }
    }

    pub fn input(&mut self, value: i64) {
        self.input.push_back(value);
    }

    pub fn input_ascii(&mut self, ascii: &str) {
        self.input.extend(ascii.bytes().map(|b| b as i64));
    }

    /// Runs until either the program needs input, outputs a value or encounters the halt opcode.
    /// In the first two cases, the computer can be resumed by calling `run` again.
    pub fn run(&mut self) -> State {
        loop {
            let code = self.code[self.pc];

            match code % 100 {
                // Add
                1 => {
                    let first = self.address(code / 100, 1);
                    let second = self.address(code / 1000, 2);
                    let third = self.address(code / 10000, 3);
                    self.code[third] = self.code[first] + self.code[second];
                    self.pc += 4;
                }
                // Multiply
                2 => {
                    let first = self.address(code / 100, 1);
                    let second = self.address(code / 1000, 2);
                    let third = self.address(code / 10000, 3);
                    self.code[third] = self.code[first] * self.code[second];
                    self.pc += 4;
                }
                // Read input channel
                3 => {
                    let Some(value) = self.input.pop_front() else {
                        break State::Input;
                    };
                    let first = self.address(code / 100, 1);
                    self.code[first] = value;
                    self.pc += 2;
                }
                // Write output channel
                4 => {
                    let first = self.address(code / 100, 1);
                    let value = self.code[first];
                    self.pc += 2;
                    break State::Output(value);
                }
                // Jump if true
                5 => {
                    let first = self.address(code / 100, 1);
                    let second = self.address(code / 1000, 2);
                    let value = self.code[first] == 0;
                    self.pc = if value { self.pc + 3 } else { self.code[second] as usize };
                }
                // Jump if false
                6 => {
                    let first = self.address(code / 100, 1);
                    let second = self.address(code / 1000, 2);
                    let value = self.code[first] == 0;
                    self.pc = if value { self.code[second] as usize } else { self.pc + 3 };
                }
                // Less than
                7 => {
                    let first = self.address(code / 100, 1);
                    let second = self.address(code / 1000, 2);
                    let third = self.address(code / 10000, 3);
                    let value = self.code[first] < self.code[second];
                    self.code[third] = value as i64;
                    self.pc += 4;
                }
                // Equals
                8 => {
                    let first = self.address(code / 100, 1);
                    let second = self.address(code / 1000, 2);
                    let third = self.address(code / 10000, 3);
                    let value = self.code[first] == self.code[second];
                    self.code[third] = value as i64;
                    self.pc += 4;
                }
                // Adjust relative base
                9 => {
                    let first = self.address(code / 100, 1);
                    self.base += self.code[first];
                    self.pc += 2;
                }
                _ => break State::Halted,
            }
        }
    }

    /// Calculates an address using one of the three possible address modes.
    /// If the address exceeds the size of the `code` vector then it is extended with 0 values.
    #[inline]
    fn address(&mut self, mode: i64, offset: usize) -> usize {
        let index = match mode % 10 {
            0 => self.code[self.pc + offset] as usize,
            1 => self.pc + offset,
            2 => (self.base + self.code[self.pc + offset]) as usize,
            _ => unreachable!(),
        };

        if index >= self.code.len() {
            self.code.resize(index + 1, 0);
        }

        index
    }
}
