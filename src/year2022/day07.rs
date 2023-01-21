use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<u32> {
    let mut cd = false;
    let mut total = 0;
    let mut stack: Vec<u32> = vec![];
    let mut sizes: Vec<u32> = vec![];

    for token in input.split_ascii_whitespace() {
        if cd {
            if token == ".." {
                sizes.push(total);
                total += stack.pop().unwrap();
            } else {
                stack.push(total);
                total = 0;
            }
            cd = false;
        } else if token == "cd" {
            cd = true
        }
        else if token.as_bytes()[0].is_ascii_digit() {
            total += from::<u32>(token)
        }
    }

    while !stack.is_empty() {
        sizes.push(total);
        total += stack.pop().unwrap();
    }

    sizes
}

pub fn part1(input: &[u32]) -> u32 {
    input.iter().filter(|&&x| x <= 100_000).sum()
}

pub fn part2(input: &[u32]) -> u32 {
    let root = input.last().unwrap();
    let needed = 30_000_000 - (70_000_000 - root);
    *input.iter().filter(|&&x| x >= needed).min().unwrap()
}
