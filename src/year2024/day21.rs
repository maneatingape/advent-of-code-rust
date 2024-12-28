//! # Keypad Conundrum
//!
//! Each key sequence always end in `A`. This means that we can consider each group of button
//! presses between `A`s independently using a recursive approach with memoization to efficiently
//! compute the minimum presses needed for any depth of chained robots.
use crate::util::hash::*;
use crate::util::parse::*;
use crate::util::point::*;
use std::iter::{once, repeat_n};

type Input = (Vec<(String, usize)>, Combinations);
type Combinations = FastMap<(char, char), Vec<String>>;
type Cache = FastMap<(char, char, usize), usize>;

/// Convert codes to pairs of the sequence itself with the numeric part.
/// The pad combinations are the same between both parts so only need to be computed once.
pub fn parse(input: &str) -> Input {
    let pairs = input.lines().map(String::from).zip(input.iter_unsigned()).collect();
    (pairs, pad_combinations())
}

pub fn part1(input: &Input) -> usize {
    chain(input, 3)
}

pub fn part2(input: &Input) -> usize {
    chain(input, 26)
}

fn chain(input: &Input, depth: usize) -> usize {
    let (pairs, combinations) = input;
    let cache = &mut FastMap::with_capacity(500);
    pairs.iter().map(|(code, numeric)| dfs(cache, combinations, code, depth) * numeric).sum()
}

fn dfs(cache: &mut Cache, combinations: &Combinations, code: &str, depth: usize) -> usize {
    // Number of presses for the last keypad is just the length of the sequence.
    if depth == 0 {
        return code.len();
    }

    // All keypads start with `A`, either the initial position of the keypad or the trailing `A`
    // from the previous sequence at this level.
    let mut previous = 'A';
    let mut result = 0;

    for current in code.chars() {
        // Check each pair of characters, memoizing results.
        let key = (previous, current, depth);

        result += cache.get(&key).copied().unwrap_or_else(|| {
            // Each transition has either 1 or 2 possibilities.
            // Pick the sequence that results in the minimum keypresses.
            let presses = combinations[&(previous, current)]
                .iter()
                .map(|next| dfs(cache, combinations, next, depth - 1))
                .min()
                .unwrap();
            cache.insert(key, presses);
            presses
        });

        previous = current;
    }

    result
}

/// Compute keypresses needed for all possible transitions for both numeric and directional
/// keypads. There are no distinct pairs shared between the keypads so they can use the same map
/// without conflict.
fn pad_combinations() -> Combinations {
    let numeric_gap = Point::new(0, 3);
    let numeric_keys = [
        ('7', Point::new(0, 0)),
        ('8', Point::new(1, 0)),
        ('9', Point::new(2, 0)),
        ('4', Point::new(0, 1)),
        ('5', Point::new(1, 1)),
        ('6', Point::new(2, 1)),
        ('1', Point::new(0, 2)),
        ('2', Point::new(1, 2)),
        ('3', Point::new(2, 2)),
        ('0', Point::new(1, 3)),
        ('A', Point::new(2, 3)),
    ];

    let directional_gap = Point::new(0, 0);
    let directional_keys = [
        ('^', Point::new(1, 0)),
        ('A', Point::new(2, 0)),
        ('<', Point::new(0, 1)),
        ('v', Point::new(1, 1)),
        ('>', Point::new(2, 1)),
    ];

    let mut combinations = FastMap::with_capacity(145);
    pad_routes(&mut combinations, &numeric_keys, numeric_gap);
    pad_routes(&mut combinations, &directional_keys, directional_gap);
    combinations
}

/// Each route between two keys has 2 possibilites, horizontal first or vertical first.
/// We skip any route that would cross the gap and also avoid adding the same route twice
/// when a key is in a straight line (e.g. directly above/below or left/right). For example:
///
/// * `7 => A` is only `>>vvv`.
/// * `1 => 5` is `^>` and `>^`.
///
/// We don't consider routes that change direction more than once as these are always longer,
/// for example `5 => A` ignores the path `v>v`.
fn pad_routes(combinations: &mut Combinations, pad: &[(char, Point)], gap: Point) {
    for &(first, from) in pad {
        for &(second, to) in pad {
            let horizontal = || {
                let element = if from.x < to.x { '>' } else { '<' };
                let count = from.x.abs_diff(to.x) as usize;
                repeat_n(element, count)
            };

            let vertical = || {
                let element = if from.y < to.y { 'v' } else { '^' };
                let count = from.y.abs_diff(to.y) as usize;
                repeat_n(element, count)
            };

            if Point::new(from.x, to.y) != gap {
                let path = vertical().chain(horizontal()).chain(once('A')).collect();
                combinations.entry((first, second)).or_default().push(path);
            }

            if from.x != to.x && from.y != to.y && Point::new(to.x, from.y) != gap {
                let path = horizontal().chain(vertical()).chain(once('A')).collect();
                combinations.entry((first, second)).or_default().push(path);
            }
        }
    }
}
