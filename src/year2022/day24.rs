//! # Blizzard Basin
//!
//! Similar to the previous day we represent the position of elves and blizzards as bits in an
//! integer in order to efficiently compute the next minute. The grid is much wider than it is tall,
//! so we transpose it and store each column as bits in a `u64`, one bit per row. We further
//! optimize by memoizing the position of vertical blizzards as they repeat every `height` minutes.
pub struct Input {
    width: usize,
    height: usize,
    left: Vec<u64>,
    right: Vec<u64>,
    vertical: Vec<u64>,
}

pub fn parse(input: &str) -> Input {
    // Don't include the left and right walls.
    let raw: Vec<_> = input.lines().map(|line| &line.as_bytes()[1..line.len() - 1]).collect();
    let width = raw[0].len();
    let height = raw.len() - 2;

    // For each blizzard type set a `0` bit in the corresponding integer. Later on we can AND this
    // with elves to eliminate possible positions.
    let build = |kind| -> Vec<_> {
        let fold = |x| (1..=height).fold(0, |acc, y| (acc << 1) | u64::from(raw[y][x] != kind));
        (0..width).map(fold).collect()
    };

    // Horizontal blizzards repeat every `width` minutes. Storing two copies of the pattern turns
    // the rotation into a simple offset.
    let left = build(b'<').repeat(2);
    let right = build(b'>').repeat(2);

    // Vertical blizzards repeat every `height` minutes so precompute to save time later.
    let up = build(b'^');
    let down = build(b'v');
    let mut vertical = Vec::with_capacity(height * width);

    for time in 0..height {
        for i in 0..width {
            let up = (up[i] << time) | (up[i] >> (height - time));
            let down = (down[i] >> time) | (down[i] << (height - time));
            vertical.push(up & down);
        }
    }

    Input { width, height, left, right, vertical }
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
    let Input { width, height, left, right, vertical } = input;
    let mut time = start;
    let mut state = vec![0; width + 1];

    loop {
        time += 1;
        // Blizzards blowing left drag the pattern towards the start, those blowing right away
        // from it. Both offsets stay within the doubled arrays.
        let left = &left[time % width..];
        let right = &right[width - time % width..];
        let vertical = &vertical[width * (time % height)..];

        // We modify the state in-place as we process each column, so preserve the previous state
        // for subsequent calculations.
        let mut prev;
        let mut cur = 0;
        let mut next = state[0];

        for i in 0..*width {
            prev = cur;
            cur = next;
            next = state[i + 1];
            // The Elves frontier can spread out 1 in each orthogonal direction unless there
            // is a blizzard present.
            state[i] =
                (cur | (cur >> 1) | (cur << 1) | prev | next) & left[i] & right[i] & vertical[i];
        }

        // Depending on the direction elves can wait indefinitely in the start or end positions.
        if forward {
            // Start position.
            state[0] |= 1 << (height - 1);
            // If we reached the end then stop.
            if state[width - 1] & 1 != 0 {
                break time + 1;
            }
        } else {
            // End position.
            state[width - 1] |= 1;
            // If we've reached the start then stop.
            if state[0] & (1 << (height - 1)) != 0 {
                break time + 1;
            }
        }
    }
}
