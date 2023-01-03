use crate::util::parse::to_vec_u32;

pub struct Pairs(u32, u32, u32, u32);

pub fn parse(input: &str) -> Vec<Pairs> {
    fn helper(line: &str) -> Pairs {
        let tokens: Vec<u32> = to_vec_u32(line);
        match tokens[..] {
            [a, b, c, d] => Pairs(a, b, c, d),
            _ => panic!("Unexpected input")
        }
    }
    input.lines().map(helper).collect()
}

pub fn part1(input: &[Pairs]) -> usize {
    fn helper(pairs: &&Pairs) -> bool {
        let Pairs(a, b, c, d) = pairs;
        (a >= c && b <= d) || (c >= a && d <= b)
    }
    input.iter().filter(helper).count()
}

pub fn part2(input: &[Pairs]) -> usize {
    fn helper(pairs: &&Pairs) -> bool {
        let Pairs(a, b, c, d) = pairs;
        a <= d && c <= b
    }
    input.iter().filter(helper).count()
}
