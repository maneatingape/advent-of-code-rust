//! # Lobby
//!
//! The highest possible joltage is made by choosing the maximum value for the most
//! significant digit (leaving enough batteries over to make the rest of the bank),
//! then the maximum value for the second most significant digit and so on.
//!
//! One approach is to scan from left to right checking for the maximum. While the complexity is
//! technically `O(n)` we do have to scan the same digits multiple times (up to twelve in part two).
//!
//! Instead, starting with enough batteries to make the bank, we scan from right to left. If we
//! encounter a battery greater than or equal to the leading battery in the bank then we replace it.
//! The replaced battery is then checked against the next battery, "bubbling" up from right to left
//! just like the [infamous sorting algorithm](https://en.wikipedia.org/wiki/Bubble_sort).
//!
//! While the worst case complexity of bubble sort is `O(n²)`, in practice this approach is much
//! faster due to the randomized nature of the inputs.
use std::mem::replace;

pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part1(input: &[&str]) -> u64 {
    solve::<2>(input)
}

pub fn part2(input: &[&str]) -> u64 {
    solve::<12>(input)
}

fn solve<const N: usize>(input: &[&str]) -> u64 {
    let mut batteries = [0; N];

    input
        .iter()
        .map(|&bank| {
            // Start with enough batteries to make the bank, taken from the right of the input.
            let end = bank.len() - N;
            batteries.copy_from_slice(&bank.as_bytes()[end..]);

            // Scan from right to left, bubbling up any battery greater than the start of the bank.
            for mut next in bank[..end].bytes().rev() {
                for battery in &mut batteries {
                    if next < *battery {
                        break;
                    }
                    next = replace(battery, next);
                }
            }

            batteries.iter().fold(0, |joltage, &b| 10 * joltage + u64::from(b - b'0'))
        })
        .sum()
}
