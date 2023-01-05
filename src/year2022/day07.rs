use crate::util::parse::to_u32;

pub fn parse(input: &str) -> Vec<u32> {
    let mut total = 0;
    let mut stack: Vec<u32> = vec![];
    let mut sizes: Vec<u32> = vec![];

    for line in input.lines().skip(1) {
        let tokens: Vec<&str> = line.split(" ").collect();
        match tokens[..] {
            ["$", "cd", ".."] => {
                sizes.push(total);
                total += stack.pop().unwrap();
            }
            ["$", "cd", _] => {
                stack.push(total);
                total = 0;
            }
            ["$", _] => (),
            ["dir", _] => (),
            [size, _] => {
                total += to_u32(size);
            }
            _ => ()
        }
    }

    while !stack.is_empty() {
        sizes.push(total);
        total += stack.pop().unwrap();
    }

    sizes.push(total);
    sizes
}

pub fn part1(input: &[u32]) -> u32 {
    input.iter().filter(|x| **x <= 100_000).sum()
}

pub fn part2(input: &[u32]) -> u32 {
    let max = input.iter().max().unwrap();
    let needed = 30_000_000 - (70_000_000 - max);
    *input.iter().filter(|x| **x >= needed).min().unwrap()
}
