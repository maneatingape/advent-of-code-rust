//! # Report Repair
//!
//! The straightforward approach is to compare every possible pair of elements for part one and
//! every possible triple for part two. This would have `O(n²)` and `O(n³)` time complexity respectively.
//!
//! We can do better with `O(n)` complexity for part one and `O(n²)` for part two.
//!
//! For part one we use an implicit hash table in an array, since values are constrained to between
//! 0 and 2020 and each value is already perfectly hashed. For each entry we check the index
//! at its value. If this is marked then we have seen the reciprocal `2020 - value` before
//! so we have found the answer. Creating this array also performs a radix sort that will
//! be used in part two.
//!
//! Part two adds a second array, this time containing the product of any two numbers that
//! sum to that point. Since the problem only has one correct answer, it does not matter if
//! other array slots get written more than once. Because the list is already sorted, we can short
//! circuit iterations as soon as a sum is not possible.
use crate::util::parse::*;

type Input = (usize, [bool; 2020]);

pub fn parse(input: &str) -> Input {
    let mut numbers = [false; 2020];
    let mut part_one = 0;

    // Part one is determined as a side effect of the parse. Assume the input has no duplicates.
    for number in input.iter_unsigned::<usize>() {
        if numbers[2020 - number] {
            part_one = number * (2020 - number);
        }
        numbers[number] = true;
    }

    // The parse performed a radix sort; numbers can now be used as an ordered sparse array.
    (part_one, numbers)
}

pub fn part1(input: &Input) -> usize {
    input.0
}

pub fn part2(input: &Input) -> usize {
    let (_, numbers) = input;

    // We know at least one of the three numbers is at least 2020/3.
    for i in 674..2020 {
        for j in 1..i {
            let k = 2020 - i - j;
            if numbers[i] && numbers[j] && numbers[k] {
                return i * j * k;
            }
        }
    }

    unreachable!()
}
