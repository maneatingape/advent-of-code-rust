use crate::util::parse::to_tuple_4;

type Pairs = (u32, u32, u32, u32);

pub fn parse(input: &str) -> Vec<Pairs> {
    input.lines().map(to_tuple_4::<u32>).collect()
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
