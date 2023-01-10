use crate::util::collection::*;
use crate::util::parse::*;

pub struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: u64,
    yes: usize,
    no: usize,
}

pub enum Operation {
    Square,
    Multiply(u64),
    Add(u64),
}

impl Operation {
    fn worry(&self, x: u64) -> u64 {
        match self {
            Operation::Square => x * x,
            Operation::Multiply(y) => x * y,
            Operation::Add(y) => x + y,
        }
    }
}

pub fn parse(input: &str) -> Vec<Monkey> {
    fn helper(chunk: &[&str]) -> Monkey {
        let items = to_vec::<u64>(chunk[1]);
        let tokens: Vec<&str> = chunk[2].split(' ').rev().take(2).collect();
        let operation = match tokens[..] {
            ["old", _] => Operation::Square,
            [y, "*"] => Operation::Multiply(to(y)),
            [y, "+"] => Operation::Add(to(y)),
            _ => unreachable!(),
        };
        let test = to_tuple_1::<u64>(chunk[3]);
        let yes = to_tuple_1::<usize>(chunk[4]);
        let no = to_tuple_1::<usize>(chunk[5]);
        Monkey { items, operation, test, yes, no, }
    }
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(7)
        .map(helper)
        .collect()
}

pub fn part1(input: &[Monkey]) -> u64 {
    play(input, 20, |x| x / 3)
}

pub fn part2(input: &[Monkey]) -> u64 {
    let product: u64 = input.iter().map(|m| m.test).product();
    play(input, 10000, |x| x % product)
}

fn play(monkeys: &[Monkey], rounds: u32, adjust: impl Fn(u64) -> u64) -> u64 {
    let mut business: Vec<u64> = Vec::fill(monkeys.len(), 0);

    for start_index in 0..monkeys.len() {
        for start_item in monkeys[start_index].items.iter() {
            let mut index = start_index;
            let mut item = *start_item;
            let mut count = 0;

            while count < rounds {
                let Monkey { items: _, operation, test, yes, no } = &monkeys[index];
                business[index] += 1;
                item = adjust(operation.worry(item));
                let next = if item % test == 0 { *yes } else { *no };
                if next < index { count += 1; }
                index = next;
            }
        }
    }

    business.sort_unstable();
    business.iter().rev().take(2).product()
}
