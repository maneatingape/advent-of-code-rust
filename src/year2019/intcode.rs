//! Implementation of the full Intcode computer specification.
use std::collections::VecDeque;

/// [SWAG](https://en.wikipedia.org/wiki/Scientific_wild-ass_guess)
/// It's possible that some inputs will need more space than this.
/// At least one input for day 17 is known to produce a grid size of 85x61 in this region.
const EXTRA: usize = 6_000;

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

macro_rules! generate_opcode_match {
    // Entry point: Kick off recursion with an empty accumulator token list.
    ($val:expr, $self:expr, [ $m1:ident, $m2:ident, $m3:ident ], [ $($ops:tt)* ]) => {
        generate_opcode_match!(@collect $val, $self, [ $m1, $m2, $m3 ], [ $($ops)* ], [])
    };

    // 2. Recursive step: Pass the identifiers down and use simple token unrolling.
    (@collect $val:expr, $self:expr, [ $m1:ident, $m2:ident, $m3:ident ], [ $op_id:expr => $body:expr, $($tail_ops:tt)* ], [ $($acc_arms:tt)* ]) => {
        generate_opcode_match!(
            @collect $val, $self, [ $m1, $m2, $m3 ], [ $($tail_ops)* ],
            [
                $($acc_arms)*

                // Unroll directly, with $m1, $m2, $m3 provided by caller to avoid breaking any
                // hygiene rules when using those names in $body.
                idx if idx == 2 + (($op_id - 1) * 27) + 0 => { #[allow(unused_variables)] let ($m1, $m2, $m3) = (0, 0, 0); $body },
                idx if idx == 2 + (($op_id - 1) * 27) + 1 => { #[allow(unused_variables)] let ($m1, $m2, $m3) = (1, 0, 0); $body },
                idx if idx == 2 + (($op_id - 1) * 27) + 2 => { #[allow(unused_variables)] let ($m1, $m2, $m3) = (2, 0, 0); $body },
                idx if idx == 2 + (($op_id - 1) * 27) + 3 => { #[allow(unused_variables)] let ($m1, $m2, $m3) = (0, 1, 0); $body },
                idx if idx == 2 + (($op_id - 1) * 27) + 4 => { #[allow(unused_variables)] let ($m1, $m2, $m3) = (1, 1, 0); $body },
                idx if idx == 2 + (($op_id - 1) * 27) + 5 => { #[allow(unused_variables)] let ($m1, $m2, $m3) = (2, 1, 0); $body },
                idx if idx == 2 + (($op_id - 1) * 27) + 6 => { #[allow(unused_variables)] let ($m1, $m2, $m3) = (0, 2, 0); $body },
                idx if idx == 2 + (($op_id - 1) * 27) + 7 => { #[allow(unused_variables)] let ($m1, $m2, $m3) = (1, 2, 0); $body },
                idx if idx == 2 + (($op_id - 1) * 27) + 8 => { #[allow(unused_variables)] let ($m1, $m2, $m3) = (2, 2, 0); $body },

                idx if idx == 2 + (($op_id - 1) * 27) + 9 => { #[allow(unused_variables)] let ($m1, $m2, $m3) = (0, 0, 1); $body },
                idx if idx == 2 + (($op_id - 1) * 27) + 10 => { #[allow(unused_variables)] let ($m1, $m2, $m3) = (1, 0, 1); $body },
                idx if idx == 2 + (($op_id - 1) * 27) + 11 => { #[allow(unused_variables)] let ($m1, $m2, $m3) = (2, 0, 1); $body },
                idx if idx == 2 + (($op_id - 1) * 27) + 12 => { #[allow(unused_variables)] let ($m1, $m2, $m3) = (0, 1, 1); $body },
                idx if idx == 2 + (($op_id - 1) * 27) + 13 => { #[allow(unused_variables)] let ($m1, $m2, $m3) = (1, 1, 1); $body },
                idx if idx == 2 + (($op_id - 1) * 27) + 14 => { #[allow(unused_variables)] let ($m1, $m2, $m3) = (2, 1, 1); $body },
                idx if idx == 2 + (($op_id - 1) * 27) + 15 => { #[allow(unused_variables)] let ($m1, $m2, $m3) = (0, 2, 1); $body },
                idx if idx == 2 + (($op_id - 1) * 27) + 16 => { #[allow(unused_variables)] let ($m1, $m2, $m3) = (1, 2, 1); $body },
                idx if idx == 2 + (($op_id - 1) * 27) + 17 => { #[allow(unused_variables)] let ($m1, $m2, $m3) = (2, 2, 1); $body },

                idx if idx == 2 + (($op_id - 1) * 27) + 18 => { #[allow(unused_variables)] let ($m1, $m2, $m3) = (0, 0, 2); $body },
                idx if idx == 2 + (($op_id - 1) * 27) + 19 => { #[allow(unused_variables)] let ($m1, $m2, $m3) = (1, 0, 2); $body },
                idx if idx == 2 + (($op_id - 1) * 27) + 20 => { #[allow(unused_variables)] let ($m1, $m2, $m3) = (2, 0, 2); $body },
                idx if idx == 2 + (($op_id - 1) * 27) + 21 => { #[allow(unused_variables)] let ($m1, $m2, $m3) = (0, 1, 2); $body },
                idx if idx == 2 + (($op_id - 1) * 27) + 22 => { #[allow(unused_variables)] let ($m1, $m2, $m3) = (1, 1, 2); $body },
                idx if idx == 2 + (($op_id - 1) * 27) + 23 => { #[allow(unused_variables)] let ($m1, $m2, $m3) = (2, 1, 2); $body },
                idx if idx == 2 + (($op_id - 1) * 27) + 24 => { #[allow(unused_variables)] let ($m1, $m2, $m3) = (0, 2, 2); $body },
                idx if idx == 2 + (($op_id - 1) * 27) + 25 => { #[allow(unused_variables)] let ($m1, $m2, $m3) = (1, 2, 2); $body },
                idx if idx == 2 + (($op_id - 1) * 27) + 26 => { #[allow(unused_variables)] let ($m1, $m2, $m3) = (2, 2, 2); $body },
            ]
        )
    };

    // Base case for recursion, supplying mapping for DECODE[99]=>1 for halt, and 0 for unreachable.
    (@collect $val:expr, $self:expr, [ $m1:ident, $m2:ident, $m3:ident ], [], [ $($acc_arms:tt)* ]) => {
        match $val {
            1 => break State::Halted,
            $($acc_arms)*
            _ => unreachable!(),
        }
    };
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
        /// Compact valid values down into a u8 for easier matching.
        static DECODE: [u8; 22210] = {
            let mut table = [0_u8; 22210];
            let mut i = 1;
            while i < 10 {
                let mut j = 0;
                while j < 3 {
                    let mut k = 0;
                    while k < 3 {
                        let mut l = 0;
                        while l < 3 {
                            let index = i + 100 * j + 1000 * k + 10000 * l;
                            table[index] = ((i - 1) * 27 + 2 + j + k * 3 + l * 9) as u8;
                            l += 1;
                        }
                        k += 1;
                    }
                    j += 1;
                }
                i += 1;
            }
            table[99] = 1;
            table
        };

        loop {
            let decode = DECODE[self.code[self.pc]];

            generate_opcode_match!(decode, self, [mode1, mode2, mode3], [
                // Add
                1 => {
                    let first = self.address(mode1, 1);
                    let second = self.address(mode2, 2);
                    let third = self.address(mode3, 3);
                    self.code[third] = self.code[first].wrapping_add(self.code[second]);
                    self.pc += 4;
                },
                // Multiply
                2 => {
                    let first = self.address(mode1, 1);
                    let second = self.address(mode2, 2);
                    let third = self.address(mode3, 3);
                    self.code[third] = self.code[first].wrapping_mul(self.code[second]);
                    self.pc += 4;
                },
                // Read input channel.
                3 => {
                    let Some(value) = self.input.pop_front() else {
                        break State::Input;
                    };
                    let first = self.address(mode1, 1);
                    self.code[first] = value;
                    self.pc += 2;
                },
                // Write output channel.
                4 => {
                    let first = self.address(mode1, 1);
                    let value = self.code[first];
                    self.pc += 2;
                    break State::Output(value as i64);
                },
                // Jump if true.
                5 => {
                    let first = self.address(mode1, 1);
                    let second = self.address(mode2, 2);
                    let value = self.code[first] == 0;
                    self.pc = if value { self.pc + 3 } else { self.code[second] };
                },
                // Jump if false.
                6 => {
                    let first = self.address(mode1, 1);
                    let second = self.address(mode2, 2);
                    let value = self.code[first] == 0;
                    self.pc = if value { self.code[second] } else { self.pc + 3 };
                },
                // Less than
                7 => {
                    let first = self.address(mode1, 1);
                    let second = self.address(mode2, 2);
                    let third = self.address(mode3, 3);
                    let value = (self.code[first] as i64) < (self.code[second] as i64);
                    self.code[third] = value as usize;
                    self.pc += 4;
                },
                // Equals
                8 => {
                    let first = self.address(mode1, 1);
                    let second = self.address(mode2, 2);
                    let third = self.address(mode3, 3);
                    let value = self.code[first] == self.code[second];
                    self.code[third] = value as usize;
                    self.pc += 4;
                },
                // Adjust relative base.
                9 => {
                    let first = self.address(mode1, 1);
                    self.base = self.base.wrapping_add(self.code[first]);
                    self.pc += 2;
                },
                // Op 99 was already mapped to match arm 1 by macro.
            ]);
        }
    }

    /// Calculates an address using one of the three possible address modes.
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
