pub struct Input {
    width: usize,
    height: usize,
    horizontal: Vec<u128>,
    vertical: Vec<u128>,
}

pub fn parse(input: &str) -> Input {
    let raw: Vec<&[u8]> = input
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            &bytes[1..(bytes.len() - 1)]
        })
        .collect();

    let width = raw[0].len();
    let height = raw.len() - 2;
    let build = |kind| {
        let fold = |row: &&[u8]| {
            row.iter()
                .fold(0, |acc, &b| (acc << 1) | if b == kind { 0 } else { 1 })
        };
        raw[1..=height].iter().map(fold).collect()
    };
    let left: Vec<u128> = build(b'<');
    let right: Vec<u128> = build(b'>');
    let up: Vec<u128> = build(b'^');
    let down: Vec<u128> = build(b'v');

    let mut horizontal = Vec::with_capacity(width * height);
    for time in 0..width {
        for i in 0..height {
            let left = left[i] << time | left[i] >> (width - time);
            let right = right[i] >> time | right[i] << (width - time);
            horizontal.push(left & right);
        }
    }

    let mut vertical = Vec::with_capacity(height * height);
    for time in 0..height {
        for i in 0..height {
            let up = up[(i + time) % height];
            let down = down[(height + i - time % height) % height];
            vertical.push(up & down);
        }
    }

    Input { width, height, horizontal, vertical }
}

pub fn part1(input: &Input) -> usize {
    expedition(input, 0, true)
}

pub fn part2(input: &Input) -> usize {
    let first = expedition(input, 0, true);
    let second = expedition(input, first, false);
    expedition(input, second, true)
}

fn expedition(input: &Input, start: usize, forward: bool) -> usize {
    let Input { width, height, horizontal, vertical } = input;
    let mut time = start;
    let mut state: Vec<u128> = vec![0; height + 1];

    loop {
        time += 1;
        let mut prev;
        let mut cur = 0;
        let mut next = state[0];

        for i in 0..*height {
            prev = cur;
            cur = next;
            next = state[i + 1];
            state[i] = (cur | cur >> 1 | cur << 1 | prev | next)
                & horizontal[height * (time % width) + i]
                & vertical[height * (time % height) + i]
        }

        if forward {
            state[0] |= 1 << (width - 1);
            if state[height - 1] & 1 != 0 {
                break time + 1;
            }
        } else {
            state[height - 1] |= 1;
            if state[0] & 1 << (width - 1) != 0 {
                break time + 1;
            }
        }
    }
}
