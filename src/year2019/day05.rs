//! # Sunny with a Chance of Asteroids
use crate::util::parse::*;
use std::sync::mpsc::*;
use std::thread;

struct IntCode {
    pc: usize,
    code: Vec<i64>,
    input_rx: Receiver<i64>,
    output_tx: Sender<i64>,
}

impl IntCode {
    const FACTOR: [i64; 4] = [0, 100, 1000, 10000];

    fn spawn(code: &[i64]) -> (Sender<i64>, Receiver<i64>) {
        let pc = 0;
        let code = code.to_vec();
        let (input_tx, input_rx) = channel();
        let (output_tx, output_rx) = channel();

        let mut computer = IntCode { pc, code, input_rx, output_tx };
        thread::spawn(move || computer.run());

        (input_tx, output_rx)
    }

    fn run(&mut self) {
        loop {
            let op = self.code[self.pc];

            match op % 100 {
                1 => {
                    let value = self.read(1) + self.read(2);
                    self.write(3, value);
                    self.pc += 4;
                }
                2 => {
                    let value = self.read(1) * self.read(2);
                    self.write(3, value);
                    self.pc += 4;
                }
                3 => {
                    let value = self.input_rx.recv().unwrap();
                    self.write(1, value);
                    self.pc += 2;
                }
                4 => {
                    let value = self.read(1);
                    let _ = self.output_tx.send(value);
                    self.pc += 2;
                }
                5 => {
                    let first = self.read(1);
                    let second = self.read(2);
                    self.pc = if first == 0 { self.pc + 3 } else { second as usize };
                }
                6 => {
                    let first = self.read(1);
                    let second = self.read(2);
                    self.pc = if first == 0 { second as usize } else { self.pc + 3 };
                }
                7 => {
                    let value = self.read(1) < self.read(2);
                    self.write(3, value as i64);
                    self.pc += 4;
                }
                8 => {
                    let value = self.read(1) == self.read(2);
                    self.write(3, value as i64);
                    self.pc += 4;
                }
                _ => break,
            }
        }
    }

    fn read(&self, offset: usize) -> i64 {
        let mode = self.code[self.pc] / Self::FACTOR[offset];

        let index = match mode % 10 {
            0 => self.code[self.pc + offset] as usize,
            1 => self.pc + offset,
            _ => unreachable!(),
        };

        self.code[index]
    }

    fn write(&mut self, offset: usize, value: i64) {
        let mode = self.code[self.pc] / Self::FACTOR[offset];

        let index = match mode % 10 {
            0 => self.code[self.pc + offset],
            _ => unreachable!(),
        };

        self.code[index as usize] = value;
    }
}

pub fn parse(input: &str) -> Vec<i64> {
    input.iter_signed().collect()
}

pub fn part1(input: &[i64]) -> i64 {
    run(input, 1)
}

pub fn part2(input: &[i64]) -> i64 {
    run(input, 5)
}

/// Start `IntCode` computer in its own thread, sending a single initial value.
/// Receives multiple values from the output channel returning only the last one.
fn run(input: &[i64], value: i64) -> i64 {
    let (tx, rx) = IntCode::spawn(input);
    let _ = tx.send(value);

    let mut result = 0;
    while let Ok(output) = rx.recv() {
        result = output;
    }
    result
}
