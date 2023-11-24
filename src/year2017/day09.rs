//! # Stream Processing
//!
//! Computes both parts in a single pass.
type Input = (u32, u32);

pub fn parse(input: &str) -> Input {
    let mut iter = input.bytes();
    let mut groups = 0;
    let mut depth = 1;
    let mut characters = 0;

    while let Some(b) = iter.next() {
        match b {
            b'<' => {
                // Inner loop for garbage.
                while let Some(b) = iter.next() {
                    match b {
                        b'!' => {
                            iter.next();
                        }
                        b'>' => break,
                        _ => characters += 1,
                    }
                }
            }
            b'{' => {
                groups += depth;
                depth += 1;
            }
            b'}' => {
                depth -= 1;
            }
            _ => (),
        }
    }

    (groups, characters)
}

pub fn part1(input: &Input) -> u32 {
    input.0
}

pub fn part2(input: &Input) -> u32 {
    input.1
}
