//! # Monkey Market
//!
//! Solves both parts simultaneously, parallelizing the work over multiple threads since
//! each secret number is independent. The process of generating the next secret number is a
//! [linear feedback shift register](https://en.wikipedia.org/wiki/Linear-feedback_shift_register).
//! with a cycle of 2²⁴.
//!
//! Interestingly, this means that with some clever math it's possible to generate the `n`th number
//! from any starting secret number with only 24 calculations. Unfortunately this doesn't help for
//! part two since we need to check every possible price change. However to speed things up we can
//! make several optimizations:
//!
//! * First the sequence of 4 prices is converted from -9..9 to a base 19 index of 0..19.
//! * Whether a monkey has seen a sequence before and the total bananas for each sequence are
//!   stored in an array. This is much faster than a `HashMap`. Using base 19 gives much better
//!   cache locality needing only 130321 elements, for example compared to shifting each new cost
//!   by 5 bits and storing in an array of 2²⁰ = 1048675 elements. Multiplication on modern
//!   processors is cheap (and several instructions can issue at once) but random memory access
//!   is expensive.
//!
//! A SIMD variant processes 8 hashes at a time, taking about 60% of the time of the scalar version.
//! The bottleneck is that disjoint indices must be written in sequence reducing the amount of work
//! that can be parallelized.
use crate::util::parse::*;
use crate::util::thread::*;

type Input = (u64, u16);
type Result = (u64, Vec<u16>);

pub fn parse(input: &str) -> Input {
    #[cfg(not(feature = "simd"))]
    let result = scalar::parallel(input);
    #[cfg(feature = "simd")]
    let result = simd::parallel(input);

    // Merge results from different threads.
    let mut part_one = 0;
    let mut part_two = vec![0; 130321];

    for (first, second) in result {
        part_one += first;
        part_two.iter_mut().zip(second).for_each(|(a, b)| *a += b);
    }

    (part_one, part_two.into_iter().max().unwrap())
}

pub fn part1(input: &Input) -> u64 {
    input.0
}

pub fn part2(input: &Input) -> u16 {
    input.1
}

#[cfg(not(feature = "simd"))]
mod scalar {
    use super::*;

    // Use as many cores as possible to parallelize the remaining search.
    pub(super) fn parallel(input: &str) -> Vec<Result> {
        let numbers: Vec<_> = input.iter_unsigned().collect();
        spawn_parallel_iterator(&numbers, worker)
    }

    fn worker(iter: ParIter<'_, u32>) -> Result {
        let mut part_one = 0;
        let mut part_two = vec![0; 130321];
        let mut seen = vec![u16::MAX; 130321];

        for (id, &number) in iter.enumerate() {
            let id = id as u16;

            let zeroth = number;
            let first = hash(zeroth);
            let second = hash(first);
            let third = hash(second);

            let mut a;
            let mut b = to_index(zeroth, first);
            let mut c = to_index(first, second);
            let mut d = to_index(second, third);

            let mut number = third;
            let mut previous = third % 10;

            for _ in 3..2000 {
                number = hash(number);
                let price = number % 10;

                // Compute index into the array.
                (a, b, c, d) = (b, c, d, to_index(previous, price));
                let index = (6859 * a + 361 * b + 19 * c + d) as usize;
                previous = price;

                // Only sell the first time we see a sequence.
                // By storing the id in the array we don't need to zero every iteration which is faster.
                if seen[index] != id {
                    part_two[index] += price as u16;
                    seen[index] = id;
                }
            }

            part_one += number as u64;
        }

        (part_one, part_two)
    }

    /// Compute next secret number using a
    /// [Xorshift LFSR](https://en.wikipedia.org/wiki/Linear-feedback_shift_register#Xorshift_LFSRs).
    fn hash(mut n: u32) -> u32 {
        n = (n ^ (n << 6)) & 0xffffff;
        n = (n ^ (n >> 5)) & 0xffffff;
        (n ^ (n << 11)) & 0xffffff
    }

    /// Convert -9..9 to 0..18.
    fn to_index(previous: u32, current: u32) -> u32 {
        9 + current % 10 - previous % 10
    }
}

#[cfg(feature = "simd")]
mod simd {
    use super::*;
    use std::simd::Simd;
    use std::simd::num::SimdUint as _;

    type Vector = Simd<u32, 8>;

    pub(super) fn parallel(input: &str) -> Vec<Result> {
        let mut numbers: Vec<_> = input.iter_unsigned().collect();

        // Add zero elements so that size is a multiple of 8.
        // Zero always hashes to zero and does not contribute to score.
        numbers.resize(numbers.len().next_multiple_of(8), 0);
        let chunks: Vec<_> = numbers.chunks_exact(8).collect();

        spawn_parallel_iterator(&chunks, worker)
    }

    /// Similar to scalar version but using SIMD vectors instead.
    /// 8 lanes is the sweet spot for performance as the bottleneck is the scalar loop writing
    /// to disjoint indices after each step.
    fn worker(iter: ParIter<'_, &[u32]>) -> Result {
        let ten = Simd::splat(10);
        let x = Simd::splat(6859);
        let y = Simd::splat(361);
        let z = Simd::splat(19);

        let mut part_one = 0;
        let mut part_two = vec![0; 130321];

        for slice in iter {
            // Each lane uses a different bit to track if a sequence has been seen before.
            let mut seen = vec![u8::MAX; 130321];

            let zeroth = Simd::from_slice(slice);
            let first = hash(zeroth);
            let second = hash(first);
            let third = hash(second);

            let mut a;
            let mut b = to_index(zeroth, first);
            let mut c = to_index(first, second);
            let mut d = to_index(second, third);

            let mut number = third;
            let mut previous = third % ten;

            for _ in 3..2000 {
                number = hash(number);
                let prices = number % ten;

                // Compute index into the array.
                (a, b, c, d) = (b, c, d, to_index(previous, prices));
                let indices = x * a + y * b + z * c + d;
                previous = prices;

                // Only sell the first time we see a sequence.
                let indices = indices.to_array();
                let prices = prices.to_array();

                for i in 0..8 {
                    let index = indices[i] as usize;

                    // Avoid branching to improve speed, instead multiply by either 0 or 1,
                    // depending if sequence has been seen before or not.
                    let bit = (seen[index] >> i) & 1;
                    seen[index] &= !(1 << i);

                    part_two[index] += prices[i] as u16 * bit as u16;
                }
            }

            part_one += number.reduce_sum() as u64;
        }

        (part_one, part_two)
    }

    /// SIMD vector arguments are passed in memory so inline functions to avoid slow transfers
    /// to and from memory.
    #[inline]
    fn hash(mut n: Vector) -> Vector {
        let mask = Simd::splat(0xffffff);
        n = (n ^ (n << 6)) & mask;
        n = (n ^ (n >> 5)) & mask;
        (n ^ (n << 11)) & mask
    }

    #[inline]
    fn to_index(previous: Vector, current: Vector) -> Vector {
        let nine = Simd::splat(9);
        let ten = Simd::splat(10);
        nine + (current % ten) - (previous % ten)
    }
}
