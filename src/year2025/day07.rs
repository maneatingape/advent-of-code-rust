//! # Laboratories
type Input = (u64, u64);

pub fn parse(input: &str) -> Input {
    let lines: Vec<_> = input.lines().map(str::as_bytes).collect();
    let width = lines[0].len();
    let start = lines[0].iter().position(|&b| b == b'S').unwrap();

    let mut splits = 0;
    let mut current = vec![0; width];
    let mut next = vec![0; width];

    current[start] = 1;

    for row in lines {
        for (i, &count) in current.iter().enumerate() {
            if count > 0 {
                if row[i] == b'^' {
                    splits += 1;

                    if i > 0 {
                        next[i - 1] += count;
                    }
                    if i < width - 1 {
                        next[i + 1] += count;
                    }
                } else {
                    next[i] += count;
                }
            }
        }

        (current, next) = (next, current);
        next.fill(0);
    }

    (splits, current.iter().sum())
}

pub fn part1(input: &Input) -> u64 {
    input.0
}

pub fn part2(input: &Input) -> u64 {
    input.1
}
