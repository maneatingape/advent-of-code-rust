//! # Signals and Noise
//!
//! The cardinality of uppercase letters is only 26 so we can use a fixed size array to
//! count the frequency of each character efficiently.
type Input = Vec<[u32; 26]>;

pub fn parse(input: &str) -> Input {
    let width = input.lines().next().unwrap().len();
    let mut freq = vec![[0; 26]; width];

    for (i, b) in input.bytes().filter(u8::is_ascii_lowercase).enumerate() {
        freq[i % width][(b - b'a') as usize] += 1;
    }

    freq
}

pub fn part1(input: &Input) -> String {
    find(input, |freq| {
        freq.iter().enumerate().filter(|(_, f)| **f > 0).max_by_key(|(_, f)| **f).unwrap()
    })
}

pub fn part2(input: &Input) -> String {
    find(input, |freq| {
        freq.iter().enumerate().filter(|(_, f)| **f > 0).min_by_key(|(_, f)| **f).unwrap()
    })
}

fn find(input: &Input, ec: impl Fn(&[u32; 26]) -> (usize, &u32)) -> String {
    input.iter().map(ec).map(|(index, _)| ((index as u8) + b'a') as char).collect()
}
