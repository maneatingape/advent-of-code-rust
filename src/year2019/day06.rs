//! # Universal Orbit Map
//!
//! Each object name is 3 characters long, using the characters `A` to `Z` and `0` to `9`.
//! This is only 36 ^ 3 = 46656 possibilities, so we can use
//! [perfect hashing](https://en.wikipedia.org/wiki/Perfect_hash_function) to store contiguous
//! indices for each object, allowing us to lookup a perfect *minimal* hash for each object.
//!
//! This is twice as fast as using a [`FastMap`] to lookup the indices.
//!
//! [`FastMap`]: crate::util::hash

/// Convert 3 character object names to contiguous indices for faster lookup.
pub fn parse(input: &str) -> Vec<usize> {
    // Convert 'A'.."Z" and '0'..'9' to a number between 0 and 36.
    let digit = |b: u8| {
        if b.is_ascii_digit() { (b - b'0') as usize } else { (10 + b - b'A') as usize }
    };

    // Hash each 3 character object name.
    let perfect_hash = |object: &str| -> usize {
        let bytes = object.as_bytes();
        digit(bytes[0]) + 36 * digit(bytes[1]) + 1296 * digit(bytes[2])
    };

    // Pre-seed known indices for objects that we need to specifically lookup later.
    let mut indices = [0_u16; 36 * 36 * 36];
    indices[perfect_hash("COM")] = 1;
    indices[perfect_hash("SAN")] = 2;
    indices[perfect_hash("YOU")] = 3;
    let mut current = 4;

    // Assign sequential indices to each object the first time that we encounter it.
    // 0 is used as a special "empty" value.
    let mut lookup = |s: &str| {
        let hash = perfect_hash(s);
        if indices[hash] == 0 {
            let previous = current;
            indices[hash] = current;
            current += 1;
            previous as usize
        } else {
            indices[hash] as usize
        }
    };

    // Build parent-child relationships for each object. Add one extra for the unused 0 special
    // value and another as there is always one more object than input lines.
    let lines: Vec<_> = input.lines().collect();
    let mut parent = vec![0; lines.len() + 2];

    for line in lines {
        let left = lookup(&line[..3]);
        let right = lookup(&line[4..]);
        parent[right] = left;
    }

    parent
}

/// Recusively follow parent relationships all the way to the root COM object. Cache each object's
/// depth in order to avoid unecessary work.
pub fn part1(input: &[usize]) -> usize {
    fn orbits(parent: &[usize], cache: &mut [Option<usize>], index: usize) -> usize {
        if let Some(result) = cache[index] {
            result
        } else {
            let result = 1 + orbits(parent, cache, parent[index]);
            cache[index] = Some(result);
            result
        }
    }

    let cache = &mut vec![None; input.len()];
    cache[0] = Some(0);
    cache[1] = Some(0);
    (0..input.len()).map(|index| orbits(input, cache, index)).sum()
}

/// Trace Santa's path all the way to the root COM object keeping track of distance. Then
/// trace our path to the root. As soon as we encounter a non-zero distance then we've hit
/// the first common ancestor and can calculate the required transfers.
pub fn part2(input: &[usize]) -> u16 {
    let mut distance = vec![0_u16; input.len()];
    let mut index = 2; // SAN
    let mut count = 0;

    // COM = 1
    while index != 1 {
        distance[index] = count;
        index = input[index];
        count += 1;
    }

    index = 3; // YOU
    count = 0;

    while distance[index] == 0 {
        index = input[index];
        count += 1;
    }

    distance[index] + count - 2
}
