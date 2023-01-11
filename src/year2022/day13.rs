pub fn parse(input: &str) -> Vec<String> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.replace("10", "A"))
        .collect()
}

pub fn part1(input: &[String]) -> usize {
    input
        .chunks_exact(2)
        .enumerate()
        .map(|(i, chunk)| if compare(&chunk[0], &chunk[1]) { i + 1 } else { 0 })
        .sum()
}

pub fn part2(input: &[String]) -> u32 {
    let mut first = 1;
    let mut second = 2;

    for packet in input.iter() {
        if compare(packet, "[[2]]") {
            first += 1;
            second += 1;
        } else if compare(packet, "[[6]]") {
            second += 1;
        }
    }

    first * second
}

fn compare(left: &str, right: &str) -> bool {
    let mut left_iter = left.chars();
    let mut right_iter = right.chars();
    let mut left_extra: Vec<char> = Vec::new();
    let mut right_extra: Vec<char> = Vec::new();

    while let (Some(a), Some(b)) = (
        left_extra.pop().or_else(|| left_iter.next()),
        right_extra.pop().or_else(|| right_iter.next()),
    ) {
        match (a, b) {
            (a, b) if a == b => (),
            (']', _) => return true,
            (_, ']') => return false,
            ('[', b) => {
                right_extra.push(']');
                right_extra.push(b);
            }
            (a, '[') => {
                left_extra.push(']');
                left_extra.push(a);
            }
            (a, b) => return a < b,
        }
    }

    unreachable!();
}
