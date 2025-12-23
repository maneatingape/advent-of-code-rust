//! # Laboratories
//!
//! Examining the input shows that it consists of a triangular Christmas tree shape with both every
//! second line and second space blank. Two splitters will never occur immediately next to each
//! other. This structure speeds up and simplifies the solution, and we compute both parts together.
//!
//! The key insight to part two is that we only need the *total count* of paths, not each
//! separate path. This means that if 2 paths enter a tile from the left and another 2 from the
//! right, then we can simply sum the paths to 4. A dynamic programming approach counts the total
//! number of paths one row at a time.
//!
//! When a beam hits a splitter, the count underneath the splitter will be zero, and the number
//! of beams to either side is incremented by the count of the beams hitting the splitter.
type Input = (u64, u64);

pub fn parse(input: &str) -> Input {
    let lines: Vec<_> = input.lines().map(str::as_bytes).collect();
    let width = lines[0].len();
    let center = width / 2;

    let mut splits = 0;
    let mut timelines = vec![0; width];
    timelines[center] = 1;

    // Only process every second line and every second tile on each line,
    // starting in the center and growing in a triangle by 1 tile in each direction.
    for (y, row) in lines.iter().skip(2).step_by(2).enumerate() {
        for x in ((center - y)..(center + y + 1)).step_by(2) {
            let count = timelines[x];

            // Not all splitters are reachable, so check that there are beams from above.
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
