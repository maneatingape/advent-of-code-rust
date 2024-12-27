//! # Blizzard Basin
//!
//! Similar to the previous day we represent the position of elves and blizzards as bits in an
//! integer in order to efficiently compute the next minute.
//!
//! We further optimize by memoizing the position of blizzards as they repeat
//! every `width` minutes for horizontal and every `height` minutes for vertical.
pub struct Input {
    width: usize,
    height: usize,
    horizontal: Vec<u128>,
    vertical: Vec<u128>,
}

pub fn parse(input: &str) -> Input {
    // Don't include the left and right walls.
    let raw: Vec<_> = input
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            &bytes[1..(bytes.len() - 1)]
        })
        .collect();

    let width = raw[0].len();
    let height = raw.len() - 2;
    // For each blizzard type set a `0` bit in the corresponding integer. Later on we can AND this
    // with elves to eliminate possible positions.
    let build = |kind| -> Vec<_> {
        let fold = |row: &&[u8]| row.iter().fold(0, |acc, &b| (acc << 1) | (b != kind) as u128);
        raw[1..=height].iter().map(fold).collect()
    };
    // Process each row.
    let left = build(b'<');
    let right = build(b'>');
    let up = build(b'^');
    let down = build(b'v');

    // Blizzard patterns repeat every `width` minutes, so we can precompute all possible
    // horizontal patterns.
    let mut horizontal = Vec::with_capacity(width * height);
    for time in 0..width {
        for i in 0..height {
            let left = (left[i] << time) | (left[i] >> (width - time));
            let right = (right[i] >> time) | (right[i] << (width - time));
            horizontal.push(left & right);
        }
    }

    // Similarly vertical blizzards repeat every `height` minutes so precompute to save time later.
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
    let mut state = vec![0; height + 1];

    loop {
        time += 1;
        // We modify the state in-place as we process each row, so preserve the previous state
        // for subsequent calculations.
        let mut prev;
        let mut cur = 0;
        let mut next = state[0];

        for i in 0..*height {
            prev = cur;
            cur = next;
            next = state[i + 1];
            // The Elves frontier can spread out 1 in each orthogonal direction unless there
            // is a blizzard present.
            state[i] = (cur | (cur >> 1) | (cur << 1) | prev | next)
                & horizontal[height * (time % width) + i]
                & vertical[height * (time % height) + i];
        }

        // Depending on the direction elves can wait indefinitely in the start or end positions.
        if forward {
            // Start position.
            state[0] |= 1 << (width - 1);
            // If we reached the end then stop.
            if state[height - 1] & 1 != 0 {
                break time + 1;
            }
        } else {
            // End position.
            state[height - 1] |= 1;
            // If we've reached the start then stop.
            if state[0] & (1 << (width - 1)) != 0 {
                break time + 1;
            }
        }
    }
}
