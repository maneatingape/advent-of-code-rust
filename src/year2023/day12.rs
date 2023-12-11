use crate::util::hash::*;
use crate::util::parse::*;

type Input = Vec<(Vec<u8>, Vec<usize>)>;
type Cache = FastMap<(usize, usize), u64>;

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (prefix, suffix) = line.split_once(' ').unwrap();
            let first = prefix.as_bytes().to_vec();
            let second = suffix.iter_unsigned().collect();
            (first, second)
        })
        .collect()
}

pub fn part1(input: &Input) -> u64 {
    solve(input, 0)
}

pub fn part2(input: &Input) -> u64 {
    solve(input, 4)
}

pub fn solve(input: &Input, repeat: usize) -> u64 {
    let mut result = 0;
    let mut bytes = Vec::new();
    let mut nums = Vec::new();
    let mut cache = FastMap::new();

    for (first, second) in input {
        for _ in 0..repeat {
            bytes.extend_from_slice(first);
            bytes.push(b'?');
            nums.extend_from_slice(second);
        }

        bytes.extend_from_slice(first);
        bytes.push(b'.');
        nums.extend_from_slice(second);

        let mut sum = 0;
        let mut ps = vec![0; nums.len()];

        for i in (1..nums.len()).rev() {
            sum += nums[i] + 1;
            ps[i - 1] = sum;
        }

        result += helper(&bytes, &nums, &ps, &mut cache);

        bytes.clear();
        nums.clear();
        cache.clear();
    }

    result
}

fn helper(slice: &[u8], nums: &[usize], ps: &[usize], cache: &mut Cache) -> u64 {
    let key = (slice.len(), nums.len());
    if let Some(prev) = cache.get(&key) {
        return *prev;
    }

    if nums.is_empty() {
        let result = working(slice) as u64;
        cache.insert(key, result);
        return result;
    }

    let size = nums[0];
    let wiggle = slice.len() - ps[0] - size;
    let mut result = 0;

    for offset in 0..wiggle {
        if offset > 0 && slice[offset - 1] == b'#' {
            break;
        }
        if slice[offset + size] != b'#' && broken(&slice[offset..offset + size]) {
            result += helper(&slice[offset + size + 1..], &nums[1..], &ps[1..], cache);
        }
    }

    cache.insert(key, result);
    result
}

fn working(slice: &[u8]) -> bool {
    slice.iter().all(|&b| b == b'.' || b == b'?')
}

fn broken(slice: &[u8]) -> bool {
    slice.iter().all(|&b| b == b'#' || b == b'?')
}
