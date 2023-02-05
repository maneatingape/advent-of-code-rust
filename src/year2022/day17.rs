use core::slice::Iter;
use std::iter::{Copied, Cycle};

const FLOOR: u8 = 0xff;
const WALLS: u32 = 0x01010101;
const ROCKS: [Rock; 5] = [
    Rock {
        size: 1,
        shape: 0x0000003c,
    },
    Rock {
        size: 3,
        shape: 0x00103810,
    },
    Rock {
        size: 3,
        shape: 0x00080838,
    },
    Rock {
        size: 4,
        shape: 0x20202020,
    },
    Rock {
        size: 2,
        shape: 0x00003030,
    },
];

#[derive(Copy, Clone)]
struct Rock {
    size: usize,
    shape: u32,
}

struct State<'a> {
    rocks: Cycle<Copied<Iter<'a, Rock>>>,
    jets: Cycle<Copied<Iter<'a, u8>>>,
    tower: Vec<u8>,
    height: usize,
}

impl State<'_> {
    fn new(input: &[u8]) -> State {
        let mut state = State {
            rocks: ROCKS.iter().copied().cycle(),
            jets: input.iter().copied().cycle(),
            tower: vec![0; 40_000],
            height: 0,
        };
        state.tower[0] = FLOOR;
        state
    }
}

impl<'a> Iterator for State<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let Rock { size, mut shape } = self.rocks.next().unwrap();
        let mut chunk = WALLS;
        let mut index = self.height + 3;

        loop {
            let jet = self.jets.next().unwrap();
            let candidate = if jet == b'<' {
                shape.rotate_left(1)
            } else {
                shape.rotate_right(1)
            };
            if candidate & chunk == 0 {
                shape = candidate;
            };

            chunk = (chunk << 8) | WALLS | (self.tower[index] as u32);
            if shape & chunk == 0 {
                index -= 1;
            } else {
                let bytes = shape.to_le_bytes();
                self.tower[index + 1] |= bytes[0];
                self.tower[index + 2] |= bytes[1];
                self.tower[index + 3] |= bytes[2];
                self.tower[index + 4] |= bytes[3];
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
    let guess = 1000;
    let height: Vec<usize> = State::new(input).take(5 * guess).collect();
    let deltas: Vec<usize> = height
        .iter()
        .scan(0, |state, &height| {
            let delta = height - *state;
            *state = height;
            Some(delta)
        })
        .collect();

    let end = deltas.len() - guess;
    let needle = &deltas[end..];
    let start = deltas.windows(guess).position(|w| w == needle).unwrap();

    let cycle_height = height[end] - height[start];
    let cycle_width = end - start;
    let offset = 1_000_000_000_000 - 1 - start;
    let quotient = offset / cycle_width;
    let remainder = offset % cycle_width;
    (quotient * cycle_height) + height[start + remainder]
}
