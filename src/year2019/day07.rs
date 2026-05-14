//! # Amplification Circuit
//!
//! The biggest savings to be had here are by realizing that each machine id runs an independent
//! linear equation on the input. For part 1, these equations are in the form Ax + B, where B can
//! be learned by feeding in the value 0, and A can be learned by feeding in the value 1 after
//! accounting for B. For part two, each program runs exactly 10 phases of either +1, +2, or *2 as
//! its linear equation for the phase; all ten phases can be learned by feeding in ten copies of
//! input 0. Running a single machine 15 times up front is a lot lighter weight than running five
//! parallel machines per permutation being tested.
//!
//! Beyond that, a brute force solution using [`permutations`] requires `5!`, or 120, computations. But
//! since all of the linear equations are increasing, we can instead take a dynamic programming
//! approach. Given a set of machines (say ids 5, 7, and 8), we can break it into the recursive
//! problem of determining the best score when picking one of those machines to be at one end of
//! the chain, combined with the best score achieved by the smaller subset (select the best out of
//! arranging 5 with the best result from 7 and 8, arranging 7 with the best result of 5 and 8, and
//! arranging 8 with the best result from 5 and 7). This can be built up in tabular form, and
//! results in `5 * 2⁵`, or 160, bit checks. But since half of those bits are zero, it is only
//! `5 * 2⁴`, or 80, actual computations, beating permutations.
//!
//! [`permutations`]: crate::util::slice
use super::intcode::*;
use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<i64> {
    input.iter_signed::<i64>().collect()
}

pub fn part1(input: &[i64]) -> i64 {
    // Run the computer 10 times to learn the Ax + B for each amplifier id.
    let mut a = [0; 5];
    let mut b = [0; 5];
    let mut computer = Computer::new(input);

    let mut output = |phase: usize, seed: i64| {
        // Send exactly 2 inputs and receive exactly 1 output per amplifier.
        computer.reset();
        computer.input(phase as i64);
        computer.input(seed);
        let State::Output(next) = computer.run() else { unreachable!() };
        next
    };

    for i in 0..5 {
        b[i] = output(i, 0);
        a[i] = output(i, 1) - b[i];
    }

    // Store the best score seen for each set of machines considered. Bits not set will be
    // appended later in the chain.
    let mut best = [0_i64; 32];

    for set in 1..best.len() {
        for amp in 0..5 {
            // No work to do if amp is not part of the current set.
            if set & (1 << amp) == 0 {
                continue;
            }

            // See if adding current amp to best value from remaining subset is viable.
            let prior = best[set ^ (1 << amp)];
            best[set] = best[set].max(a[amp] * prior + b[amp]);
        }
    }

    best[31]
}

pub fn part2(input: &[i64]) -> i64 {
    // Run the computer 5 times to learn the +1/+2/*2 for each phase of each amplifier id.
    let mut effects = [[0; 10]; 5];
    let mut computer = Computer::new(input);

    // To track the multi-phase feedback, also track the bitmask of ids per phase that perform
    // a multiply, as well as the weight of cumulative multiplies done by all later phases.
    let mut masks = [0; 10];
    let mut weights = [1; 10];

    for (id, machine) in effects.iter_mut().enumerate() {
        // Send the id, then exactly 10 loops of 1 input and 1 output.
        computer.reset();
        computer.input(5 + id as i64);
        for phase in 0..10 {
            computer.input(0);
            let State::Output(next) = computer.run() else { unreachable!() };
            machine[phase] = next;
            if next == 0 {
                masks[phase] |= 1 << id;
                if phase > 0 {
                    weights[phase - 1] *= 2;
                }
            }
        }
    }

    // Combine per-phase weights into cumulative for rest of chain.
    for i in (0..9).rev() {
        weights[i] *= weights[i + 1];
    }

    // Store the best score seen for each set of machines considered. Bits not set occur
    // earlier in the chain.
    let mut best = [0_i64; 32];

    for set in 1..best.len() {
        for amp in 0..5 {
            // No work to do if amp is not part of the current set.
            if set & (1 << amp) == 0 {
                continue;
            }

            // Start with the best value from the subset of other machines in the set.
            let mut val = best[set ^ (1 << amp)];

            // For each phase, a +1 or +2 at this phase can determine its overall contribution
            // to the final output based on all later multiplies (masks is used to find those in the
            // current phase, weights for those in all later phases).
            for phase in 0..10 {
                let current = (masks[phase] & set).count_ones();
                val += effects[amp][phase] * (weights[phase] << current);
            }
            best[set] = best[set].max(val);
        }
    }

    best[31]
}
