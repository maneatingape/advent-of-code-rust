//! # Laboratories
type Input = (u64, u64);

pub fn parse(input: &str) -> Input {
    let lines: Vec<_> = input.lines().map(str::as_bytes).collect();
    let width = lines[0].len();
    let center = width / 2;

    let mut splits = 0;
    let mut timelines = vec![0; width];
    timelines[center] = 1;

    for (y, row) in lines.iter().skip(2).step_by(2).enumerate() {
        for x in ((center - y)..(center + y + 1)).step_by(2) {
            let count = timelines[x];

            if count > 0 && row[x] == b'^' {
                splits += 1;
                timelines[x] = 0;
                timelines[x - 1] += count;
                timelines[x + 1] += count;
            }
        }
    }

    (splits, timelines.iter().sum())
}

pub fn part1(input: &Input) -> u64 {
    input.0
}

pub fn part2(input: &Input) -> u64 {
    input.1
}
