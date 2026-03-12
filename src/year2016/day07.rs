//! # Internet Protocol Version 7
//!
//! For part two there are at most 26 × 26 = 676 possible ABA or BAB sequences so we can use
//! a fixed-size array to keep track of which ones we've seen for the current address so far.
pub fn parse(input: &str) -> Vec<&[u8]> {
    input.lines().map(str::as_bytes).collect()
}

pub fn part1(input: &[&[u8]]) -> usize {
    input
        .iter()
        .filter(|&&line| {
            let mut has_abba = false;
            let mut inside_brackets = false;

            for [w, x, y, z] in line.array_windows() {
                if w.is_ascii_lowercase() {
                    if w == z && x == y && w != x {
                        if inside_brackets {
                            return false;
                        }
                        has_abba = true;
                    }
                } else {
                    inside_brackets = *w == b'[';
                }
            }

            has_abba
        })
        .count()
}

pub fn part2(input: &[&[u8]]) -> usize {
    let mut aba = [usize::MAX; 676];
    let mut bab = [usize::MAX; 676];

    input
        .iter()
        .enumerate()
        .filter(|&(version, &line)| {
            let mut inside_brackets = false;

            for [x, y, z] in line.array_windows() {
                if x.is_ascii_lowercase() {
                    if x == z && x != y && y.is_ascii_lowercase() {
                        let first = (x - b'a') as usize;
                        let second = (y - b'a') as usize;

                        if inside_brackets {
                            // Reverse the order of letters
                            let index = 26 * second + first;
                            bab[index] = version;
                            if aba[index] == version {
                                return true;
                            }
                        } else {
                            let index = 26 * first + second;
                            aba[index] = version;
                            if bab[index] == version {
                                return true;
                            }
                        }
                    }
                } else {
                    inside_brackets = *x == b'[';
                }
            }

            false
        })
        .count()
}
