//! # Signals and Noise
//!
//! The cardinality of uppercase letters is only 26 so we can use a fixed size array to
//! count the frequency of each character efficiently.
type Input = (String, String);

pub fn parse(input: &str) -> Input {
    let width = input.lines().next().unwrap().len();
    let stride = width + 1;
    let input = input.as_bytes();

    let to_index = |b: u8| (b - b'a') as usize;
    let to_char = |i: usize| ((i as u8) + b'a') as char;

    (0..width)
        .map(|offset| {
            let mut freq = [0; 26];
            input.iter().skip(offset).step_by(stride).for_each(|&b| freq[to_index(b)] += 1);

            let (max, _) =
                freq.iter().enumerate().filter(|(_, f)| **f > 0).max_by_key(|(_, f)| **f).unwrap();
            let (min, _) =
                freq.iter().enumerate().filter(|(_, f)| **f > 0).min_by_key(|(_, f)| **f).unwrap();

            (to_char(max), to_char(min))
        })
        .unzip()
}

pub fn part1(input: &Input) -> &str {
    &input.0
}

pub fn part2(input: &Input) -> &str {
    &input.1
}
