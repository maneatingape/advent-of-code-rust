//! # Knights of the Dinner Table
//!
//! This problem is very similar to [`Day 9`] and we solve it in almost exactly the same way by
//! computing an adjacency matrix of happiness then running [Held-Karp] to find the longest
//! cycle. If part one were the only problem at hand, the answer would be possible by iterating
//! over 127 sets and then selecting among seven candidates to close the loop back to whichever
//! abritrary point we pinned as the start.
//!
//! However, we are more interested in solving part two at the same time. Do this by noticing that
//! when you insert yourself between two diners, you set the value of their mutual link to zero.
//! This is effectively the same as inserting a ninth node into the algorithm, which we pin as the
//! start node before iterating over 255 sets. Meanwhile, the results for part one can still be
//! found from the table, if we also have an easy way to determine which diner was used to start
//! the path represented by any given g(set,k). We can then manually close the loop of 8 diners
//! by using all 8 g(255,k) plus the distance from k to the start node of that path, while the
//! loop of 9 diners uses g(255,k) with no additional distance.
//!
//! [`Day 9`]: crate::year2015::day09
//! [Held-Karp]: https://en.wikipedia.org/wiki/Held%E2%80%93Karp_algorithm
use crate::util::bitset::*;
use crate::util::hash::*;
use crate::util::iter::*;
use crate::util::parse::*;

type Input = (i16, i16);

pub fn parse(input: &str) -> Input {
    // Assign each diner an index on a first come first served basis.
    let tokens: Vec<_> = input.split([' ', '.', '\n']).chunk::<12>().collect();
    let mut indices = FastMap::new();

    for &[from, .., to, _] in &tokens {
        let size = indices.len();
        indices.entry(from).or_insert(size);

        let size = indices.len();
        indices.entry(to).or_insert(size);
    }

    // Calculate the happiness values. Note that the values are not reciprocal a => b != b => a.
    let stride = indices.len();
    let mut happiness = vec![0_i16; stride * stride];

    for &[from, _, gain_lose, value, .., to, _] in &tokens {
        let start = indices[from];
        let end = indices[to];
        let sign = if gain_lose == "gain" { 1 } else { -1 };
        let value: i16 = value.signed();

        // Add the values together to make the mutual link reciprocal.
        happiness[stride * start + end] += sign * value;
        happiness[stride * end + start] += sign * value;
    }

    // Solve both parts simultaneously.
    // Initialize a shared table for both parts: 2ⁿ sets with n distances per set. Default 0 matches
    // g({k},k) for all singleton sets of zero distance from yourself, but tracking k as the start
    // of the path. The initial value of other sets does not matter.
    let zero = (0_i16, 0_u8);
    let mut table = vec![zero; stride * (1 << stride)];
    for k in 0..stride {
        table[(1 << k) * stride + k].1 = k as u8;
    }

    // Visit each non-empty set in order, with no work to do for singleton sets.
    for set in 3_usize..(1 << stride) {
        if set & !(set - 1) == set {
            continue;
        }

        // For a given set, compute each g(set,k) for all k in the set.
        for k in set.biterator() {
            let subset = set ^ (1 << k);
            let mut longest = i16::MIN;
            let mut start = u8::MAX;

            // For a given destination k, find which other bit m gives the best path from the
            // subset to m, and then m to k. All table[subset] references were filled in prior
            // iterations of the outer loop or the singleton base cases.
            for m in subset.biterator() {
                let prior = table[subset * stride + m];
                let distance = prior.0 + happiness[m * stride + k];
                if distance > longest {
                    longest = distance;
                    start = prior.1;
                }
            }
            table[set * stride + k] = (longest, start);
        }
    }

    // With the sets now built, we have stride candidates for each answer.
    // Part one requires completing the cycle back to the stashed first element.
    // Part two can be directly read off the table.
    let mut part_one = i16::MIN;
    let mut part_two = i16::MIN;
    for (k, &prior) in table[table.len() - stride..].iter().enumerate() {
        part_one = part_one.max(prior.0 + happiness[prior.1 as usize * stride + k]);
        part_two = part_two.max(prior.0);
    }

    (part_one, part_two)
}

pub fn part1(input: &Input) -> i16 {
    input.0
}

pub fn part2(input: &Input) -> i16 {
    input.1
}
