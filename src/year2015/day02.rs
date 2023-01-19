use crate::util::collection::*;
use crate::util::parse::*;

type Gift = [u32; 3];

pub fn parse(input: &str) -> Vec<Gift> {
    fn helper(tuple: (u32, u32, u32)) -> Gift {
        let (a, b, c) = tuple;
        let mut gift = [a, b, c];
        gift.sort_unstable();
        gift
    }
    input.to_unsigned_iter().tupled3().map(helper).collect()
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
