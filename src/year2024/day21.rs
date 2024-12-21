//! # Keypad Conundrum
use crate::util::hash::*;
use crate::util::parse::*;
use crate::util::point::*;

type Cache = FastMap<(usize, usize), usize>;

pub fn parse(input: &str) -> &str {
    input
}

pub fn part1(input: &str) -> usize {
    chain(input, 3)
}

pub fn part2(input: &str) -> usize {
    chain(input, 26)
}

fn chain(input: &str, limit: usize) -> usize {
    let cache = &mut FastMap::with_capacity(500);
    input
        .lines()
        .map(str::as_bytes)
        .zip(input.iter_unsigned::<usize>())
        .map(|(code, numeric)| dfs(cache, code, 0, limit) * numeric)
        .sum()
}

fn dfs(cache: &mut Cache, slice: &[u8], depth: usize, limit: usize) -> usize {
    if depth == limit {
        return slice.len();
    }

    let key = (to_usize(slice), depth);
    if let Some(&previous) = cache.get(&key) {
        return previous;
    }

    let keypad = if depth == 0 { NUMERIC } else { DIRECTIONAL };
    let mut shortest = usize::MAX;

    for sequence in combinations(slice, &keypad) {
        let mut presses = 0;

        for chunk in sequence.split_inclusive(|&b| b == b'A') {
            presses += dfs(cache, chunk, depth + 1, limit);
        }

        shortest = shortest.min(presses);
    }

    cache.insert(key, shortest);
    shortest
}

fn combinations(current: &[u8], keypad: &Keypad) -> Vec<Vec<u8>> {
    let mut next = Vec::new();
    pad_dfs(&mut next, &mut Vec::with_capacity(16), keypad, current, 0, keypad.start);
    next
}

fn pad_dfs(
    combinations: &mut Vec<Vec<u8>>,
    path: &mut Vec<u8>,
    keypad: &Keypad,
    sequence: &[u8],
    depth: usize,
    from: Point,
) {
    // Success
    if depth == sequence.len() {
        combinations.push(path.clone());
        return;
    }

    // Failure
    if from == keypad.gap {
        return;
    }

    let to = keypad.lookup[sequence[depth] as usize];

    if from == to {
        // Push button.
        path.push(b'A');
        pad_dfs(combinations, path, keypad, sequence, depth + 1, from);
        path.pop();
    } else {
        // Move towards button.
        let mut step = |next: u8, direction: Point| {
            path.push(next);
            pad_dfs(combinations, path, keypad, sequence, depth, from + direction);
            path.pop();
        };

        if to.x < from.x {
            step(b'<', LEFT);
        }
        if to.x > from.x {
            step(b'>', RIGHT);
        }
        if to.y < from.y {
            step(b'^', UP);
        }
        if to.y > from.y {
            step(b'v', DOWN);
        }
    }
}

struct Keypad {
    start: Point,
    gap: Point,
    lookup: [Point; 128],
}

const NUMERIC: Keypad = {
    let start = Point::new(2, 3);
    let gap = Point::new(0, 3);
    let mut lookup = [ORIGIN; 128];

    lookup[b'7' as usize] = Point::new(0, 0);
    lookup[b'8' as usize] = Point::new(1, 0);
    lookup[b'9' as usize] = Point::new(2, 0);
    lookup[b'4' as usize] = Point::new(0, 1);
    lookup[b'5' as usize] = Point::new(1, 1);
    lookup[b'6' as usize] = Point::new(2, 1);
    lookup[b'1' as usize] = Point::new(0, 2);
    lookup[b'2' as usize] = Point::new(1, 2);
    lookup[b'3' as usize] = Point::new(2, 2);
    lookup[b'0' as usize] = Point::new(1, 3);
    lookup[b'A' as usize] = Point::new(2, 3);

    Keypad { start, gap, lookup }
};

const DIRECTIONAL: Keypad = {
    let start = Point::new(2, 0);
    let gap = Point::new(0, 0);
    let mut lookup = [ORIGIN; 128];

    lookup[b'^' as usize] = Point::new(1, 0);
    lookup[b'A' as usize] = Point::new(2, 0);
    lookup[b'<' as usize] = Point::new(0, 1);
    lookup[b'v' as usize] = Point::new(1, 1);
    lookup[b'>' as usize] = Point::new(2, 1);

    Keypad { start, gap, lookup }
};

// Max slice length is 5 so value is unique.
fn to_usize(slice: &[u8]) -> usize {
    let mut array = [0; 8];
    array[0..slice.len()].copy_from_slice(slice);
    usize::from_ne_bytes(array)
}
