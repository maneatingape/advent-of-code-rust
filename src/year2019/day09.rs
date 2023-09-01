//! # Sensor Boost
//!
//! This problem is essentially an unit test for the canonical full intcode computer
//! used heavily by other days.
use crate::util::parse::*;
use intcode::*;

pub mod intcode {
    use std::sync::mpsc::*;
    use std::thread;

    pub struct Computer {
        pc: usize,
        base: i64,
        code: Vec<i64>,
        input_rx: Receiver<i64>,
        output_tx: Sender<i64>,
    }

    impl Computer {
        /// Spawns an `IntCode` computer in a new thread, returning an input and output channel
        /// for communicating asynchronously with the computer via the opcodes 3 and 4.
        pub fn spawn(code: &[i64]) -> (Sender<i64>, Receiver<i64>) {
            let pc = 0;
            let base = 0;
            let code = code.to_vec();
            let (input_tx, input_rx) = channel();
            let (output_tx, output_rx) = channel();

            let mut computer = Computer { pc, base, code, input_rx, output_tx };
            thread::spawn(move || computer.run());

            (input_tx, output_rx)
        }

        /// Runs until a `99` opcode instruction is encountered.
        fn run(&mut self) {
            loop {
                match self.code[self.pc] % 100 {
                    // Add
                    1 => {
                        let value = self.read(1) + self.read(2);
                        self.write(3, value);
                        self.pc += 4;
                    }
                    // Multiply
                    2 => {
                        let value = self.read(1) * self.read(2);
                        self.write(3, value);
                        self.pc += 4;
                    }
                    // Read input channel
                    3 => {
                        let value = self.input_rx.recv().unwrap();
                        self.write(1, value);
                        self.pc += 2;
                    }
                    // Write output channel
                    4 => {
                        let value = self.read(1);
                        let _ = self.output_tx.send(value);
                        self.pc += 2;
                    }
                    // Jump if true
                    5 => {
                        let first = self.read(1);
                        let second = self.read(2);
                        self.pc = if first == 0 { self.pc + 3 } else { second as usize };
                    }
                    // Jump if false
                    6 => {
                        let first = self.read(1);
                        let second = self.read(2);
                        self.pc = if first == 0 { second as usize } else { self.pc + 3 };
                    }
                    // Less than
                    7 => {
                        let value = self.read(1) < self.read(2);
                        self.write(3, value as i64);
                        self.pc += 4;
                    }
                    // Equals
                    8 => {
                        let value = self.read(1) == self.read(2);
                        self.write(3, value as i64);
                        self.pc += 4;
                    }
                    // Adjust relative base
                    9 => {
                        let value = self.read(1);
                        self.base += value;
                        self.pc += 2;
                    }
                    _ => break,
                }
            }
        }

        /// Convenience wrapper for reading a value
        fn read(&mut self, offset: usize) -> i64 {
            let index = self.address(offset);
            self.code[index]
        }

        /// Convenience wrapper for writing a value
        fn write(&mut self, offset: usize, value: i64) {
            let index = self.address(offset);
            self.code[index] = value;
        }

        /// Calculates an address using one of the three possible address modes.
        /// If the address exceeds the size of the `code` vector then it is extended with 0 values.
        fn address(&mut self, offset: usize) -> usize {
            const FACTOR: [i64; 4] = [0, 100, 1000, 10000];
            let mode = self.code[self.pc] / FACTOR[offset];

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
}

pub fn parse(input: &str) -> Vec<i64> {
    input.iter_signed().collect()
}

pub fn part1(input: &[i64]) -> i64 {
    run(input, 1)
}

pub fn part2(input: &[i64]) -> i64 {
    run(input, 2)
}

fn run(input: &[i64], value: i64) -> i64 {
    let (tx, rx) = Computer::spawn(input);
    let _ = tx.send(value);
    rx.recv().unwrap()
}
