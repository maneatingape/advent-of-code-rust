use crate::util::parse::to_u32_3;

type Gift = [u32; 3];

pub fn parse(input: &str) -> Vec<Gift> {
    fn helper(line: &str) -> Gift {
        let mut tokens = to_u32_3(line);
        tokens.sort_unstable();
        tokens 
    }
    input.lines().map(helper).collect()
}

pub fn part1(input: &[Gift]) -> u32 {
    fn helper(gift: &Gift) -> u32 {
        let [l, w, h] = gift;
        2 * (l * w + w * h + h * l) + l * w
    }
    input.iter().map(helper).sum()
}

pub fn part2(input: &[Gift]) -> u32 {
    fn helper(gift: &Gift) -> u32 {
        let [l, w, h] = gift;
        2 * (l + w) + (l * w * h)
    }
    input.iter().map(helper).sum()
}