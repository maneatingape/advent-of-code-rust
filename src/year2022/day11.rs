use crate::util::parse::*;

#[derive(Clone)]
pub struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: u64,
    yes: usize,
    no: usize,
    count: usize,
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
            _ => unreachable!()
        };
        let [test] = to_array1::<u64>(chunk[3]);
        let [yes] = to_array1::<usize>(chunk[4]);
        let [no] = to_array1::<usize>(chunk[5]);
        let count = 0;
        Monkey { items, operation, test, yes, no, count }
    }
    input.lines().collect::<Vec<&str>>().chunks(7).map(helper).collect()
}

pub fn part1(input: &[Monkey]) -> usize {
    let helper = |operation: Operation, x: u64| {
        match operation {
            Operation::Square => (x * x) / 3,
            Operation::Multiply(y) => (x * y) / 3,
            Operation::Add(y) => (x + y) / 3,
        }
    };
    play(input, helper, 20)
}

pub fn part2(input: &[Monkey]) -> usize {
    let product = input.iter().map(|m| m.test).product::<u64>();
    let helper = move |operation: Operation, x: u64| {
        match operation {
            Operation::Square => (x * x) % product,
            Operation::Multiply(y) => (x * y) % product,
            Operation::Add(y) => (x + y) % product,
        }
    };
    play(input, helper, 10000)
}

fn play(input: &[Monkey], helper: impl Fn(Operation, u64) -> u64, rounds: u32) -> usize {
    let mut monkeys: Vec<Monkey> = input.iter().map(|x| x.clone()).collect();

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            let (pass, fail): (Vec<u64>, Vec<u64>) = monkey.items.iter()
                .map(|&x| helper(monkey.operation, x))
                .partition(|&x| x % monkey.test == 0);
            let yes = monkey.yes;
            let no = monkey.no;

            monkey.count += monkey.items.len();
            monkey.items.clear();

            monkeys[yes].items.extend(pass);
            monkeys[no].items.extend(fail);
        }
    }

    let mut totals: Vec<usize> = monkeys.iter().map(|m| m.count).collect();
    totals.sort_unstable();
    totals.iter().rev().take(2).product::<usize>()
}
