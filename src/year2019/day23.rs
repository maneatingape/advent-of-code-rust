//! # Category Six
//!
//! Solves both part one and two simultaneously. A nice benefit of our intcode computer is that it
//! returns [`State::Input`] when the input queue is empty, making it easy to detect an
//! idle network.
use super::intcode::*;
use crate::util::parse::*;

type Input = (i64, i64);

pub fn parse(input: &str) -> Input {
    let code: Vec<_> = input.iter_signed().collect();
    let mut network: Vec<_> = (0..50)
        .map(|address| {
            let mut computer = Computer::new(&code);
            computer.input(address);
            computer
        })
        .collect();

    let mut sent = Vec::new();
    let mut nat_x = 0;
    let mut nat_y = 0;
    let mut first_y = None;
    let mut idle_y = None;

    loop {
        let mut index = 0;
        let mut empty = 0;

        while index < 50 {
            let computer = &mut network[index];

            match computer.run() {
                State::Output(value) => {
                    // Loop until we have accumulated a full packet of 3 values.
                    sent.push(value);
                    let [address, x, y] = sent[..] else {
                        continue;
                    };
                    sent.clear();

                    if address == 255 {
                        // Handle part one.
                        if first_y.is_none() {
                            first_y = Some(y);
                        }
                        nat_x = x;
                        nat_y = y;
                    } else {
                        let destination = &mut network[address as usize];
                        destination.input(x);
                        destination.input(y);
                    }
                }
                // Input queue is empty.
                State::Input => {
                    empty += 1;
                    computer.input(-1);
                }
                State::Halted => unreachable!(),
            }

            index += 1;
        }

        if empty == 50 {
            if idle_y == Some(nat_y) {
                break;
            }
            idle_y = Some(nat_y);

            let destination = &mut network[0];
            destination.input(nat_x);
            destination.input(nat_y);
        }
    }

    (first_y.unwrap(), idle_y.unwrap())
}

pub fn part1(input: &Input) -> i64 {
    input.0
}

pub fn part2(input: &Input) -> i64 {
    input.1
}
