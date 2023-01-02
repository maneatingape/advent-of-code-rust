use crate::util::parse::to_vec_u32;

pub struct Gift(u32, u32, u32);

pub fn parse(input: &str) -> Vec<Gift> {
    fn helper(line: &str) -> Gift {
        let mut tokens: Vec<u32> = to_vec_u32(line);
        tokens.sort_unstable();
        match tokens[..] {
            [l, w, h] => Gift(l, w, h),
            _ => panic!("Unexpected input")
        }
    }
    input.lines().map(helper).collect()
}

pub fn part1(input: &[Gift]) -> u32 {
    fn helper(gift: &Gift) -> u32 {
        let Gift(l, w, h) = gift;
        2 * (l * w + w * h + h * l) + l * w
    }
    input.iter().map(helper).sum()
}

pub fn part2(input: &[Gift]) -> u32 {
    fn helper(gift: &Gift) -> u32 {
        let Gift(l, w, h) = gift;
        2 * (l + w) + (l * w * h)
    }
    input.iter().map(helper).sum()
}