use crate::util::collection::*;
use crate::util::parse::*;

type Pairs = (u32, u32, u32, u32);

pub fn parse(input: &str) -> Vec<Pairs> {
    input.to_unsigned_iter().tupled4().collect()
}

pub fn part1(input: &[Pairs]) -> usize {
    fn helper(pairs: &&Pairs) -> bool {
        let (a, b, c, d) = pairs;
        (a >= c && b <= d) || (c >= a && d <= b)
    }
    input.iter().filter(helper).count()
}

pub fn part2(input: &[Pairs]) -> usize {
    fn helper(pairs: &&Pairs) -> bool {
        let (a, b, c, d) = pairs;
        a <= d && c <= b
    }
    input.iter().filter(helper).count()
}
