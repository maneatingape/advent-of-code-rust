pub struct Input {
    width: usize,
    height: usize,
    up: Vec<u128>,
    down: Vec<u128>,
    left: Vec<u128>,
    right: Vec<u128>,
}

pub fn parse(input: &str) -> Input {
    let mut raw: Vec<&[u8]> = input
        .lines()
        .skip(1)
        .map(|line| {
            let bytes = line.as_bytes();
            &bytes[1..(bytes.len() - 1)]
        })
        .collect();
    raw.pop();

    fn helper(row: &[u8], blizzard: u8) -> u128 {
        row
            .iter()
            .enumerate()
            .fold(0, |acc, (i, &b)| {
                if b == blizzard {
                    acc
                } else {
                    acc | (1 << i)
                }
            })
    }

    let width = raw[0].len();
    let height = raw.len();
    let up: Vec<u128> = raw.iter().map(|row| helper(row, b'^')).collect();
    let down: Vec<u128> = raw.iter().map(|row| helper(row, b'v')).collect();
    let left: Vec<u128> = raw.iter().map(|row| helper(row, b'<')).collect();
    let right: Vec<u128> = raw.iter().map(|row| helper(row, b'>')).collect();

    Input { width, height, up, down, left, right }
}

pub fn part1(input: &Input) -> usize {
    expedition(input, 0, false)
}

pub fn part2(input: &Input) -> usize {
    let first = expedition(input, 0, false);
    let second = expedition(input, first, true);
    expedition(input, second, false)
}

fn expedition(input: &Input, start: usize, reverse: bool) -> usize {
    let Input { width, height, up, down, left, right } = input;
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

            state[i] =
                (cur | cur >> 1 | cur << 1 | prev | next)
                & ((left[i] >> (time % width)) | (left[i] << (width - time % width)))
                & ((right[i] << (time % width)) | (right[i] >> (width - time % width)))
                & up[(i + time) % height]
                & down[(height + i - time % *height) % *height];
        }

        if reverse {
            state[*height - 1] |= 1 << (width - 1);
            if state[0] & 1 != 0 {
                break time + 1;
            }
        } else {
            state[0] |= 1;
            if state[*height - 1] & (1 << (width - 1)) != 0 {
                break time + 1;
            }
        }
    }
}
