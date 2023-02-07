use crate::util::parse::*;

#[derive(Clone)]
pub struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: u64,
    yes: usize,
    no: usize,
}

#[derive(Copy, Clone)]
pub enum Operation {
    Square,
    Multiply(u64),
    Add(u64),
}

pub fn parse(input: &str) -> Vec<Monkey> {
    fn helper(chunk: &[&str]) -> Monkey {
        let items = chunk[1].to_unsigned_iter().collect();
        let tokens: Vec<&str> = chunk[2].split(' ').rev().take(2).collect();
        let operation = match tokens[..] {
            ["old", _] => Operation::Square,
            [y, "*"] => Operation::Multiply(from(y)),
            [y, "+"] => Operation::Add(from(y)),
            _ => unreachable!(),
        };
        let test = chunk[3].to_unsigned_iter().next().unwrap();
        let yes = chunk[4].to_unsigned_iter().next().unwrap();
        let no = chunk[5].to_unsigned_iter().next().unwrap();
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
    let mut monkeys = input.to_vec();
    let mut business = vec![0; monkeys.len()];

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            business[i] += monkeys[i].items.len();

            while let Some(item) = monkeys[i].items.pop() {
                let worry = match monkeys[i].operation {
                    Operation::Square => item * item,
                    Operation::Multiply(y) => item * y,
                    Operation::Add(y) => item + y,
                };
                let next = adjust(worry);
                let to = if next % monkeys[i].test == 0 { monkeys[i].yes } else { monkeys[i].no };
                monkeys[to].items.push(next);
            }
        }
    }

    business.sort_unstable();
    business.iter().rev().take(2).product()
}
