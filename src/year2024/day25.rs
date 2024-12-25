//! # Code Chronicle
pub fn parse(input: &str) -> &str {
    input
}

pub fn part1(input: &str) -> u32 {
    let mut iter = input.bytes();
    let mut locks = Vec::with_capacity(250);
    let mut keys = Vec::with_capacity(250);
    let mut result = 0;

    loop {
        let mut bits = 0;

        for _ in 0..41 {
            let b = iter.next().unwrap();
            bits = (bits << 1) | (b & 1) as u64;
        }

        bits &= 0b011111_011111_011111_011111_011111_011111_011111;

        if bits & 0b011111_000000_000000_000000_000000_000000_000000 != 0 {
            locks.push(bits);
        } else {
            keys.push(bits);
        }

        iter.next();
        if iter.next().is_none() {
            break;
        }
    }

    for lock in &locks {
        for key in &keys {
            if lock & key == 0 {
                result += 1;
            }
        }
    }

    result
}

pub fn part2(_input: &str) -> &'static str {
    "n/a"
}
