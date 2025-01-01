//! # Internet Protocol Version 7
//!
//! It's faster to treat the entire input as one big stream, using line breaks to increment
//! the count if an address is valid.
//!
//! For part two there are at most 26 * 26 = 676 possible ABA or BAB sequences so we can use
//! a fixed size array to keep track of which ones we've seen for the current address so far.
pub fn parse(input: &str) -> &[u8] {
    input.as_bytes()
}

pub fn part1(input: &[u8]) -> usize {
    let mut count = 0;
    let mut inside = false;
    let mut positive = false;
    let mut negative = false;

    for w in input.windows(4) {
        if w[0].is_ascii_lowercase() {
            if w[0] == w[3] && w[1] == w[2] && w[0] != w[1] {
                if inside {
                    negative = true;
                } else {
                    positive = true;
                }
            }
        } else if w[0] == b'[' {
            inside = true;
        } else if w[0] == b']' {
            inside = false;
        } else {
            // Next line
            if positive && !negative {
                count += 1;
            }
            positive = false;
            negative = false;
        }
    }

    if positive && !negative { count + 1 } else { count }
}

pub fn part2(input: &[u8]) -> usize {
    let mut count = 0;
    let mut version = 0;
    let mut inside = false;
    let mut positive = false;
    let mut aba = [usize::MAX; 676];
    let mut bab = [usize::MAX; 676];

    for w in input.windows(3) {
        if w[1].is_ascii_lowercase() {
            if w[0] == w[2] && w[0] != w[1] {
                let first = (w[0] - b'a') as usize;
                let second = (w[1] - b'a') as usize;

                if inside {
                    // Reverse the order of letters
                    let index = 26 * second + first;
                    bab[index] = version;
                    positive |= aba[index] == version;
                } else {
                    let index = 26 * first + second;
                    aba[index] = version;
                    positive |= bab[index] == version;
                }
            }
        } else if w[1] == b'[' {
            inside = true;
        } else if w[1] == b']' {
            inside = false;
        } else {
            // Next line
            if positive {
                count += 1;
            }
            version += 1;
            positive = false;
        }
    }

    if positive { count + 1 } else { count }
}
