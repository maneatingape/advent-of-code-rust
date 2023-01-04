pub fn parse(input: &str) -> Vec<i32> {
    fn helper(c:char) -> i32 {
        match c {
            '(' => 1,
            ')' => -1,
            _ => unreachable!()
        }
    }
    input.trim().chars().map(helper).collect()
}

pub fn part1(input: &[i32]) -> i32 {
    input.iter().sum()
}

pub fn part2(input: &[i32]) -> usize {
    let mut floor = 0;

    for (i, x) in input.iter().enumerate() {
        floor += x;
        if floor < 0 { return i + 1 }
    }

    unreachable!()
}
