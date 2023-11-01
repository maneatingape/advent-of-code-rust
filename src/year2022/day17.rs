//! # Pyroclastic Flow
//!
//! ## Part One
//!
//! For speed we encode each rock shape as binary bits so that we can use bitwise logic to check
//! for collisions. Each rock is encoded top to bottom and left to right. For example:
//!
//! ```none
//!    #     00010000    0x10
//!   ### => 00111000 => 0x38 => 0x00103810
//!    #     00010000    0x10
//! ```
//!
//! The bits are shifted 2 away from the left wall. Walls are also encoded in binary, overlapping
//! the left and right walls (no rock will ever collide first with a wall and its top row):
//!
//! ```none
//!   100000001
//!   100000001 => 0x01010101
//!   100000001
//! ```
//!
//! We store the accumulated tower efficiently as a vec of `u8` including the floor at index
//! zero as a special pattern of `11111111`.
//!
//! We use bitwise AND to check for collisions between the rock, the walls and the existing tower
//! in one operation.
//!
//! ## Part Two
//!
//! Since there's no reasonable way to analytically predict the height after some `n` rocks
//! and brute force would take too long we can assume that there must be a
//! [cycle](https://en.wikipedia.org/wiki/Cycle_(graph_theory)) in the output.
//!
//! We choose an arbitrary length and generate a sequence of that size then search
//! for repeating patterns. Once we find the length of the cycle then we can extrapolate for
//! any `n` greater than the start of the cycle.
use std::iter::{Copied, Cycle};
use std::slice::Iter;

/// Convenience alias to shorten type name.
type Wrapper<'a, T> = Cycle<Copied<Iter<'a, T>>>;

/// Encode pieces one row per byte, highest row in the most significant position.
const FLOOR: u8 = 0xff;
const WALLS: u32 = 0x01010101;
const ROCKS: [Rock; 5] = [
    Rock { size: 1, shape: 0x0000003c },
    Rock { size: 3, shape: 0x00103810 },
    Rock { size: 3, shape: 0x00080838 },
    Rock { size: 4, shape: 0x20202020 },
    Rock { size: 2, shape: 0x00003030 },
];

#[derive(Copy, Clone)]
struct Rock {
    size: usize,
    shape: u32,
}

struct State<'a> {
    rocks: Wrapper<'a, Rock>,
    jets: Wrapper<'a, u8>,
    tower: Vec<u8>,
    height: usize,
}

impl State<'_> {
    fn new(input: &[u8]) -> State<'_> {
        // Rocks and jets repeat endlessly.
        // 13,000 is the the maximum possible height that the tower could reach after 5000 rocks.
        let mut state = State {
            rocks: ROCKS.iter().copied().cycle(),
            jets: input.iter().copied().cycle(),
            tower: vec![0; 13_000],
            height: 0,
        };
        state.tower[0] = FLOOR;
        state
    }
}

/// Implement as an iterator for ergonomics.
impl Iterator for State<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let Rock { size, mut shape } = self.rocks.next().unwrap();
        let mut chunk = WALLS;
        // Start 3 rows above the current top of the tower.
        let mut index = self.height + 3;

        loop {
            let jet = self.jets.next().unwrap();
            let candidate = if jet == b'<' { shape.rotate_left(1) } else { shape.rotate_right(1) };
            // Check for a horizontal collision (this does not prevent downwards movement).
            if candidate & chunk == 0 {
                shape = candidate;
            };

            // The neat part of using bitwise AND to compare is that we can check all four
            // rows in a single operation, including both walls and the existing tower.
            chunk = (chunk << 8) | WALLS | (self.tower[index] as u32);

            if shape & chunk == 0 {
                // Keep falling
                index -= 1;
            } else {
                // Add the new piece to the tower.
                let bytes = shape.to_le_bytes();
                self.tower[index + 1] |= bytes[0];
                self.tower[index + 2] |= bytes[1];
                self.tower[index + 3] |= bytes[2];
                self.tower[index + 4] |= bytes[3];
                // Rock may have fallen far enough to not add any additional height.
                self.height = self.height.max(index + size);
                break Some(self.height);
            }
        }
    }
}

pub fn parse(input: &str) -> &[u8] {
    input.trim().as_bytes()
}

pub fn part1(input: &[u8]) -> usize {
    State::new(input).nth(2021).unwrap()
}

pub fn part2(input: &[u8]) -> usize {
    // We make two complete [SWAGs](https://en.wikipedia.org/wiki/Scientific_wild-ass_guess):
    // * 1000 row deltas are enough to form a unique prefix
    // * The tower pattern will repeat in a cycle in first 5000 rows.
    let guess = 1000;
    let height: Vec<_> = State::new(input).take(5 * guess).collect();
    // We compare based on the *delta* between rows instead of absolute heights.
    let deltas: Vec<_> = height
        .iter()
        .scan(0, |state, &height| {
            let delta = height - *state;
            *state = height;
            Some(delta)
        })
        .collect();

    // Simple brute force check, instead of a
    // [cycle detection](https://en.wikipedia.org/wiki/Cycle_detection) algorithm.
    let end = deltas.len() - guess;
    let needle = &deltas[end..];
    let start = deltas.windows(guess).position(|w| w == needle).unwrap();

    // Now that we know when the cycle repeats, we can work out the height for any arbitrary
    // number of rocks after that point.
    let cycle_height = height[end] - height[start];
    let cycle_width = end - start;
    let offset = 1_000_000_000_000 - 1 - start;
    let quotient = offset / cycle_width;
    let remainder = offset % cycle_width;
    (quotient * cycle_height) + height[start + remainder]
}
