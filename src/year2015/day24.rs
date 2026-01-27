//! # It Hangs in the Balance
//!
//! To simplify things assumes that the remaining items after the first best combination is found
//! can be split evenly.
//!
//! Runs a dynamic programming preliminary pass over the data to
//! compute a bitmask of which group sizes are reachable and a bitmask
//! of which item indices were the last encountered while reaching at
//! least one of those group sizes for a given sum.  This information
//! can then be used to prune away much of the depth-first search,
//! with memoization avoiding repeated computations.  Caching is done
//! via a 3D array; a typical input will have a part 1 sum around 500
//! with the smallest group having 6 or 7 of the 28 items, so the
//! cache is smaller than 1 megabyte, and part 2 ends up reusing many
//! entries already populated during part 1.  Thus, this solution
//! computes the results during `parse()`, at which point `part1()`
//! and `part2()` merely return the already-known results.
use crate::util::hash::*;
use crate::util::parse::*;

const MAX_SUM: usize = 600;
const ITEMS: usize = 28;

#[derive(Clone, Copy)]
struct Info {
    sizes: u8, // bitmask, bit 0 set for the empty group, not bothering with more than 7 items in a group
    items: u32, // bitmask, highest item indices that allowed group to reach sum
}

struct State {
    info: [Info; MAX_SUM], // Info learned during dynamic programming pre-pass
    packages: Vec<usize>,  // Package weights from input
    total: usize,          // Total weight of all packages
    memo: FastMap<(usize, usize, usize), u64>, // Memoization cache over (sum, maxindex, groupsize); with u64::MAX for no solution
}

pub struct Input {
    best3: u64, // Best quantum entanglement for one-third of total
    best4: u64, // Best quantum entanglement for one-fourth of total
}

pub fn parse(input: &str) -> Input {
    let packages: Vec<_> = input.iter_unsigned().collect();
    let mut state = State {
        info: [Info { sizes: 0, items: 0 }; MAX_SUM],
        packages: packages.clone(),
        total: packages.iter().sum::<usize>(),
        memo: FastMap::with_capacity(MAX_SUM * ITEMS),
    };
    state.info[0].sizes = 1 << 0;
    let mut cap = 0;

    // For each item, update which groups are reachable with that item, starting from largest sum.
    for (rank, size) in state.packages.iter().enumerate() {
        cap += size;
        let goal = if cap < state.total / 3 { cap } else { state.total / 3 };
        for i in (*size..goal + 1).rev() {
            if state.info[i - *size].sizes != 0 {
                state.info[i].sizes |= state.info[i - *size].sizes << 1;
                state.info[i].items |= 1 << rank;
            }
        }
    }

    Input { best3: solve(&mut state, 3), best4: solve(&mut state, 4) }
}

pub fn part1(input: &Input) -> u64 {
    input.best3
}

pub fn part2(input: &Input) -> u64 {
    input.best4
}

/// Convert the msb below limit back into a bit number, or ITEMS if no such bit remains in value.
fn msb_below(value: u32, limit: usize) -> usize {
    let remaining = value & ((1 << limit) - 1);
    if remaining == 0 {
        return ITEMS;
    }
    31 - remaining.leading_zeros() as usize
}

/// Kick off a depth first search.
fn solve(state: &mut State, groups: usize) -> u64 {
    let goal = state.total / groups;
    let sizebit = state.info[goal].sizes & (-(state.info[goal].sizes as i8) as u8);
    combinations(
        state,
        msb_below(sizebit.into(), ITEMS),
        msb_below(state.info[goal].items, ITEMS),
        goal,
    )
}

/// Depth first search over all possible package combinations, using
/// `state::info` to focus on only the best quantum entanglement that
/// satisfy the given group size and maximum item index that reach the
/// given sum, and `state::memo` as a cache shared between parts.
/// Returns `u64::MAX` if this branch is not viable.
fn combinations(state: &mut State, size: usize, maxindex: usize, sum: usize) -> u64 {
    // Check for bottom of the recursion
    if size == 0 && sum == 0 {
        return 1;
    }

    // Impossible to proceed if size is not supported at this sum
    if ((1 << size) & state.info[sum].sizes) == 0 {
        return u64::MAX;
    }

    // Impossible to proceed if no smaller item indices remain
    if maxindex == ITEMS {
        return u64::MAX;
    }

    // Check if answer still needs to be computed
    if !state.memo.contains_key(&(sum, maxindex, size)) {
        let nextsum = sum - state.packages[maxindex];
        let by_group =
            combinations(state, size - 1, msb_below(state.info[nextsum].items, maxindex), nextsum);
        let by_index = combinations(state, size, msb_below(state.info[sum].items, maxindex), sum);
        let mut entry = u64::MAX;
        if by_group < u64::MAX {
            entry = by_group * (state.packages[maxindex] as u64);
        }
        if by_index < entry {
            entry = by_index;
        }
        state.memo.insert((sum, maxindex, size), entry);
    }

    // Return cached answer
    state.memo[&(sum, maxindex, size)]
}
