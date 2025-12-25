//! # Doesn't He Have Intern-Elves For This?
//!
//! [Regular expressions](https://en.wikipedia.org/wiki/Regular_expression) are a good fit for this
//! problem. However in the interest of speed we'll take a different approach for both parts.
//!
//! ## Part One
//! Each string consists only of lowercase ASCII characters so the cardinality is 26. We can
//! test for vowels and invalid characters more quickly by converting each character into a bitmask
//! that fits into a `i32`. For example `a` becomes `1`, b becomes `10` and so on.
//!
//! To check if a character is a vowel we logically `AND` against `100000100000100010001` which is
//! "aeiou" converted to a bitmask. Similarly to check for the invalid sequence "ab" we `AND`
//! against a mask that has `b` set and notice the previous character is always one less, so we
//! can left shift to reuse the same mask.
//!
//! ## Part Two
//! We can check for non-overlapping pairs in `O(n)` complexity by storing the last seen index of
//! each pair in the string. If the difference is more than one, then we know that the pairs are
//! non-overlapping.
//!
//! Instead of using a `HashMap` we rely on the fact there at most 26² possible combinations
//! in order to use a fixed size array as an implicit data structure. Using zero as a special
//! starting value gives 27² or 729 possibilities. To avoid having to clear the array for each
//! string, we bump the index by 1000 (any value larger than the length of the string would do).
//! This means that if the difference is greater than the current position in the string we can be
//! sure that we haven't encountered this pair in this particular string before.
pub fn parse(input: &str) -> Vec<&[u8]> {
    input.lines().map(str::as_bytes).collect()
}

pub fn part1(input: &[&[u8]]) -> usize {
    // Bitmask for vowels (a, e, i, o, u)
    const VOWEL_MASK: u32 = 0x0104111;
    // Bitmask for forbidden pairs
    const FORBIDDEN_MASK: u32 = 0x101000a;

    let nice = |line: &&&[u8]| {
        let mut vowels = 0;
        let mut pairs = 0;
        let mut previous = 0;

        for &c in line.iter() {
            let current = 1 << (c - b'a');

            if FORBIDDEN_MASK & current & (previous << 1) != 0 {
                return false;
            }
            if VOWEL_MASK & current != 0 {
                vowels += 1;
            }
            if previous == current {
                pairs += 1;
            }

            previous = current;
        }

        vowels >= 3 && pairs >= 1
    };

    input.iter().filter(nice).count()
}

pub fn part2(input: &[&[u8]]) -> usize {
    let mut pairs = [0; 729];

    let nice = |(base, line): &(usize, &&[u8])| {
        let mut first = 0;
        let mut second = 0;

        let mut two_pair = false;
        let mut split_pair = false;

        for (offset, c) in line.iter().enumerate() {
            let third = (c - b'a' + 1) as usize;
            let index = 27 * second + third;

            let position = base * 1000 + offset;
            let delta = position - pairs[index];

            if delta > offset {
                // This is the first time we've seen the pair for this string.
                pairs[index] = position;
            } else if delta > 1 {
                // No overlapping means that the distance must be at least two.
                two_pair = true;
            }
            if first == third {
                split_pair = true;
            }

            first = second;
            second = third;
        }

        two_pair && split_pair
    };

    input.iter().enumerate().filter(nice).count()
}
