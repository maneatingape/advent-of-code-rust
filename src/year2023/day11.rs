//! # Cosmic Expansion
//!
//! We simplify the problem by treating each axis independently. Consider 4 galaxies on the same
//! axis at arbitrary non-decreasing values `a b c d`. The pairwise distances are:
//!
//! * `b - a`
//! * `c - b + c - a` => `2c - (a + b)`
//! * `d - c + d - b + d - a` => `3d - (a + b + c)`
//!
//! We can see that each pairwise distance can be expressed as the current coordinate multiplied by
//! the previous number of galaxies minus the [prefix sum](https://en.wikipedia.org/wiki/Prefix_sum)
//! of the coordinates of the previous galaxies.
//!
//! In the special case that two or more galaxies are at the same coordinate, for example `c == d`:
//!
//! * `c - b + c - a` => `2c - (a + b)`
//! * `d - c + d - b + d - a` => `3d - (a + b + c)` => `2c - (a + b)`
//! * Total: `2 * [2c - (a + b)]`
//!
//! This implies that we only need the *count* of the number of galaxies at each coordinate and
//! can multiply the total value by that count. This also finds gaps with no galaxies to
//! calculate the expanded coordinates.
pub struct Input {
    xs: [usize; 140],
    ys: [usize; 140],
}

pub fn parse(input: &str) -> Input {
    let mut xs = [0; 140];
    let mut ys = [0; 140];

    for (y, row) in input.lines().enumerate() {
        for (x, b) in row.bytes().enumerate() {
            if b == b'#' {
                xs[x] += 1;
                ys[y] += 1;
            }
        }
    }

    Input { xs, ys }
}

pub fn part1(input: &Input) -> usize {
    axis(&input.xs, 2) + axis(&input.ys, 2)
}

pub fn part2(input: &Input) -> usize {
    axis(&input.xs, 1_000_000) + axis(&input.ys, 1_000_000)
}

fn axis(counts: &[usize], empty_space: usize) -> usize {
    let mut result = 0;
    let mut prefix_sum = 0;
    let mut galaxies = 0;
    let mut coordinate = 0;

    for &count in counts {
        result += count * (galaxies * coordinate - prefix_sum);
        prefix_sum += count * coordinate;
        galaxies += count;
        coordinate += if count > 0 { 1 } else { empty_space };
    }

    result
}
