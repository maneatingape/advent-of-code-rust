pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part1(input: &[&str]) -> u32 {
    fn helper(rucksack: &&str) -> u32 {
        let (a, b) = rucksack.split_at(rucksack.len() / 2);
        priority(mask(a) & mask(b))
    }
    input.iter().map(helper).sum()
}

pub fn part2(input: &[&str]) -> u32 {
    fn helper(rucksacks: &[&str]) -> u32 {
        priority(mask(rucksacks[0]) & mask(rucksacks[1]) & mask(rucksacks[2]))
    }
    input.chunks_exact(3).map(helper).sum()
}

fn mask(s: &str) -> u128 {
    s.chars().fold(0, |acc, c| acc | 1 << (c as u8))
}

fn priority(mask: u128) -> u32 {
    let zeroes = mask.trailing_zeros();
    match zeroes {
        65..=90 => zeroes - 38,
        97..=122 => zeroes - 96,
        _ => unreachable!()
    }
}
