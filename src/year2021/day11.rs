//! # Dumbo Octopus
//!
//! This puzzle resembles the [`Day 9`] flood fill a little. Since there are only 100 octopuses
//! a fixed size array is used both to track current energy levels and a second array to track
//! if an octopus has flashed this turn. Each time an octopus flashes it bumps its neighbors
//! energy levels, which can propagate recursively through the entire grid.
//!
//! [`Day 9`]: crate::year2021::day09

/// Pad the 10x10 grid by 1 on either side so that we can avoid boundary checks.
type Input = [u8; 144];

pub fn parse(input: &str) -> Input {
    let bytes: Vec<_> = input.lines().map(str::as_bytes).collect();
    let mut grid = [0; 144];

    for y in 0..10 {
        for x in 0..10 {
            grid[12 * (y + 1) + (x + 1)] = bytes[y][x] - b'0';
        }
    }

    grid
}

pub fn part1(input: &Input) -> usize {
    let (total, _) = simulate(input, |_, steps| steps < 100);
    total
}

pub fn part2(input: &Input) -> usize {
    let (_, steps) = simulate(input, |flashes, _| flashes < 100);
    steps
}

fn simulate(input: &Input, predicate: fn(usize, usize) -> bool) -> (usize, usize) {
    let mut grid = *input;
    let mut flashed = [true; 144];
    let mut todo = Vec::with_capacity(100);

    let mut flashes = 0;
    let mut steps = 0;
    let mut total = 0;

    while predicate(flashes, steps) {
        flashes = 0;

        // Bump each octopus' energy level by one. If it flashes then add to `todo` queue.
        for y in 0..10 {
            for x in 0..10 {
                let index = 12 * (y + 1) + (x + 1);

                if grid[index] < 9 {
                    grid[index] += 1;
                    flashed[index] = false;
                } else {
                    grid[index] = 0;
                    flashed[index] = true;
                    todo.push(index);
                }
            }
        }

        // Process each flash, possibly adding more to the queue.
        while let Some(index) = todo.pop() {
            flashes += 1;

            for next in [
                index + 1,
                index + 11,
                index + 12,
                index + 13,
                index - 1,
                index - 11,
                index - 12,
                index - 13,
            ] {
                if flashed[next] {
                    continue;
                }

                if grid[next] < 9 {
                    grid[next] += 1;
                } else {
                    grid[next] = 0;
                    flashed[next] = true;
                    todo.push(next);
                }
            }
        }

        steps += 1;
        total += flashes;
    }

    (total, steps)
}
