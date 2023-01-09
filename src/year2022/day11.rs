use crate::util::collection::*;
use crate::util::parse::*;

#[derive(Clone)]
pub struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: u64,
    yes: usize,
    no: usize,
}

#[derive(Clone, Copy)]
pub enum Operation {
    Square,
    Multiply(u64),
    Add(u64),
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

pub fn part1(input: &[Monkey]) -> usize {
    play(input, 20, |x| x / 3)
}

pub fn part2(input: &[Monkey]) -> usize {
    let product: u64 = input.iter().map(|m| m.test).product();
    play(input, 10000, |x| x % product)
}

fn play(input: &[Monkey], rounds: u32, adjust: impl Fn(u64) -> u64) -> usize {
    let mut monkeys: Vec<Monkey> = input.to_vec();
    let mut business: Vec<usize> = Vec::fill(monkeys.len(), 0);

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            let (pass, fail): (Vec<u64>, Vec<u64>) = monkey
                .items
                .iter()
                .map(|&x| adjust(worry(monkey.operation, x)))
                .partition(|&x| x % monkey.test == 0);
            let yes = monkey.yes;
            let no = monkey.no;

            business[i] += monkey.items.len();
            monkey.items.clear();

            monkeys[yes].items.extend(pass);
            monkeys[no].items.extend(fail);
        }
    }

    business.sort_unstable();
    business.iter().rev().take(2).product()
}

fn worry(operation: Operation, x: u64) -> u64 {
    match operation {
        Operation::Square => x * x,
        Operation::Multiply(y) => x * y,
        Operation::Add(y) => x + y,
    }
}
