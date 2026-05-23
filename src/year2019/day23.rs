//! # Category Six
//!
//! Solves both part one and two simultaneously. A nice benefit of our intcode computer is that it
//! returns [`State::Input`] when the input queue is empty, making it easy to detect an
//! idle network.
//!
//! Analysis of the network of 50 programs shows that each program id has a fixed number of
//! input slots, with slots addressed by (packetx/constant)-1 with different constants per id,
//! where the program stores packety. Once all slots are filled, a given program will either
//! copy, add, multiply, or divide the input slots and then fan out the result to a hardcoded
//! list of other programs any time a changed input results in a changed output. The connections
//! between programs form a directed acyclic graph (DAG), and the first output to 255 (the part
//! one answer) occurs only after all nodes have filled all input slots. Thereafter, feeding
//! the 255 result to node 0 triggers a wave of updates through other nodes. The overall network
//! is computing a cubic function `f(x) = (x - C1)*((x - C1)² - 300000000) / 1000000000 + x`
//! where each input to node 0 is a new x value that eventually causes f(x) to be output to node
//! 255. Since the network stabilizes at the fixed point when `f(x) == x`, the goal boils down
//! to computing the fixed point C1.
//!
//! We can use this knowledge to speed up part two: instead of multiple rounds of computing f(x)
//! on the value produced in the previous round, and waiting for things to converge, we can instead
//! trigger the start the computation of f(1) on node zero, use that to learn the id that processes
//! the x² term along with the magic numbers needed to populate its slots, then grab the output
//! of that node. We are guaranteed by how the cubic function was set up that the output of that
//! second node is -3 times C1, the answer we want.
use super::intcode::*;
use crate::util::parse::*;

type Input = (i64, i64);

pub fn parse(input: &str) -> Input {
    let code: Vec<_> = input.iter_signed().collect();
    let mut todo = Vec::with_capacity(50);
    let mut network: Vec<_> = (0..50)
        .map(|address| {
            let mut computer = Computer::new(&code);
            computer.input(address);
            computer.input(-1);
            todo.push(address);
            computer
        })
        .collect();

    let mut sent = Vec::new();
    let mut nat_x = 0;
    let mut nat_y = 0;

    // Run the network until todo is empty, at which point part 1 is available as nat_y. This
    // also gives us nat_x, which is magic number needed to pass input to node 0.
    while let Some(index) = todo.pop() {
        // Run an individual computer until it blocks for more input.
        loop {
            match network[index as usize].run() {
                State::Output(value) => {
                    // Loop until we have accumulated a full packet of 3 values.
                    sent.push(value);
                    let [address, x, y] = sent[..] else {
                        continue;
                    };
                    sent.clear();

                    if address == 255 {
                        // Handle part one.
                        nat_x = x;
                        nat_y = y;
                    } else {
                        let destination = &mut network[address as usize];
                        destination.input(x);
                        destination.input(y);
                        todo.push(address);
                    }
                }
                // Input queue is empty.
                State::Input => {
                    break;
                }
                State::Halted => unreachable!(),
            }
        }
    }

    // Instead of feeding nat_y to node 0, feed 1 to force the computation of f(1). Node 0 has
    // six output triples: first to the node in charge of the x term, then two to the node for x²,
    // and finally three to node for x³. We only need the inputs to the x² node.
    let mut next_computer = 0;
    network[0].input(nat_x);
    network[0].input(1);
    for _ in 0..9 {
        match network[0].run() {
            State::Output(value) => {
                // Loop until we have accumulated 3 triples.
                sent.push(value);
                let [address, x, y] = sent[..] else {
                    continue;
                };
                sent.clear();

                let destination = &mut network[address as usize];
                destination.input(x);
                destination.input(y);
                next_computer = address as usize;
            }
            _ => unreachable!(),
        }
    }

    // Now run the x² node until its second output triple, which is -3 times the fixed point.
    let mut last_sent = 0;
    while let State::Output(value) = network[next_computer].run() {
        last_sent = value;
    }

    (nat_y, last_sent / -3)
}

pub fn part1(input: &Input) -> i64 {
    input.0
}

pub fn part2(input: &Input) -> i64 {
    input.1
}
