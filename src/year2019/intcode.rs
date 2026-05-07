//! Implementation of the full Intcode computer specification.
use std::collections::VecDeque;

/// [SWAG](https://en.wikipedia.org/wiki/Scientific_wild-ass_guess)
/// It's possible that some inputs will need more space than this.
const EXTRA: usize = 3_000;

pub enum State {
    Input,
    Output(i64),
    Halted,
}

pub struct Computer {
    pc: usize,
    base: usize,
    code: Vec<usize>,
    input: VecDeque<usize>,
}

#[derive(Clone, Copy, Default)]
struct Decode {
    op: u8,
    mode1: u8,
    mode2: u8,
    mode3: u8,
}

impl Computer {
    pub fn new(input: &[i64]) -> Computer {
        let mut code = Vec::with_capacity(input.len() + EXTRA);
        code.extend(input.iter().map(|&i| i as usize));
        code.resize(code.len() + EXTRA, 0);

        Computer { pc: 0, base: 0, code, input: VecDeque::new() }
    }

    pub fn input(&mut self, value: i64) {
        self.input.push_back(value as usize);
    }

    pub fn input_ascii(&mut self, ascii: &str) {
        self.input.extend(ascii.bytes().map(|b| b as usize));
    }

    /// Resets state *except* for memory which may have been modified.
    pub fn reset(&mut self) {
        self.pc = 0;
        self.base = 0;
        self.input.clear();
    }

    /// Runs until either the program needs input, outputs a value or encounters the halt opcode.
    /// In the first two cases, the computer can be resumed by calling `run` again.
    pub fn run(&mut self) -> State {
        /// Preparing a decoder table at compile time avoids division operators at runtime.
        static DECODE: [Decode; 22210] = {
            let zero = Decode { op: 0, mode1: 0, mode2: 0, mode3: 0 };
            let mut table = [zero; 22210];
            let mut i = 1;
            while i < 10 {
                let mut j = 0;
                while j < 3 {
                    let mut k = 0;
                    while k < 3 {
                        let mut l = 0;
                        while l < 3 {
                            let index = i + 100 * j + 1000 * k + 10000 * l;
                            table[index].op = i as u8;
                            table[index].mode1 = j as u8;
                            table[index].mode2 = k as u8;
                            table[index].mode3 = l as u8;
                            l += 1;
                        }
                        k += 1;
                    }
                    j += 1;
                }
                i += 1;
            }
            table
        };

        loop {
            let decode = DECODE[self.code[self.pc]];

            match decode.op {
                // Add
                1 => {
                    let first = self.address(decode.mode1, 1);
                    let second = self.address(decode.mode2, 2);
                    let third = self.address(decode.mode3, 3);
                    self.code[third] = self.code[first].wrapping_add(self.code[second]);
                    self.pc += 4;
                }
                // Multiply
                2 => {
                    let first = self.address(decode.mode1, 1);
                    let second = self.address(decode.mode2, 2);
                    let third = self.address(decode.mode3, 3);
                    self.code[third] = self.code[first].wrapping_mul(self.code[second]);
                    self.pc += 4;
                }
                // Read input channel.
                3 => {
                    let Some(value) = self.input.pop_front() else {
                        break State::Input;
                    };
                    let first = self.address(decode.mode1, 1);
                    self.code[first] = value;
                    self.pc += 2;
                }
                // Write output channel.
                4 => {
                    let first = self.address(decode.mode1, 1);
                    let value = self.code[first];
                    self.pc += 2;
                    break State::Output(value as i64);
                }
                // Jump if true.
                5 => {
                    let first = self.address(decode.mode1, 1);
                    let second = self.address(decode.mode2, 2);
                    let value = self.code[first] == 0;
                    self.pc = if value { self.pc + 3 } else { self.code[second] };
                }
                // Jump if false.
                6 => {
                    let first = self.address(decode.mode1, 1);
                    let second = self.address(decode.mode2, 2);
                    let value = self.code[first] == 0;
                    self.pc = if value { self.code[second] } else { self.pc + 3 };
                }
                // Less than
                7 => {
                    let first = self.address(decode.mode1, 1);
                    let second = self.address(decode.mode2, 2);
                    let third = self.address(decode.mode3, 3);
                    let value = (self.code[first] as i64) < (self.code[second] as i64);
                    self.code[third] = value as usize;
                    self.pc += 4;
                }
                // Equals
                8 => {
                    let first = self.address(decode.mode1, 1);
                    let second = self.address(decode.mode2, 2);
                    let third = self.address(decode.mode3, 3);
                    let value = self.code[first] == self.code[second];
                    self.code[third] = value as usize;
                    self.pc += 4;
                }
                // Adjust relative base.
                9 => {
                    let first = self.address(decode.mode1, 1);
                    self.base = self.base.wrapping_add(self.code[first]);
                    self.pc += 2;
                }
                _ => break State::Halted,
            }
        }
    }

    /// Calculates an address using one of the three possible address modes.
    #[inline]
    fn address(&self, mode: u8, offset: usize) -> usize {
        let index = self.pc + offset;
        match mode {
            0 => self.code[index],
            1 => index,
            2 => self.base.wrapping_add(self.code[index]),
            _ => unreachable!(),
        }
    }
}
