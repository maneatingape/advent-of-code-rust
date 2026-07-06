//! # Lanternfish
//!
//! The key observation is that all fish of the same age behave the same, so we only
//! need to store the *total* of each fish per day, rather than each fish individually.
//!
//! Another optimization trick is rather than modifying the array by removing the fish at day 0,
//! then shifting each fish total down by 1, we can simply increment what we consider the
//! head of the array modulo 9 to achieve the same effect in place.
//!
//! What's more, the computation of how many fish result after N generations for a single starting fish
//! can be determined up front.  With a table containing only `9*257` u64 entries (about 18k
//! memory) computed at compile time, we can determine the two answers in constant runtime.

type Input = [u64; 9];

const DAYS: usize = 256;
const TIMERS: usize = 9;
static IMPACT: [[u64; TIMERS]; DAYS + 1] = {
    let mut table = [[0; TIMERS]; DAYS + 1];

    let mut timer = 0;
    while timer < TIMERS {
        table[0][timer] = 1;
        timer += 1;
    }

    let mut day = 0;
    while day < DAYS {
        let mut timer = 0;
        while timer < TIMERS {
            table[day + 1][(timer + 1) % TIMERS] = table[day][timer];
            timer += 1;
        }
        table[day + 1][0] += table[day][6];
        day += 1;
    }

    table
};

pub fn parse(input: &str) -> Input {
    let mut fish = [0_u64; 9];
    for ch in input.as_bytes().iter().step_by(2) {
        fish[(ch & 0xf) as usize] += 1;
    }
    fish
}

pub fn part1(input: &Input) -> u64 {
    input.iter().zip(&IMPACT[80]).map(|(fish, impact)| fish * impact).sum()
}

pub fn part2(input: &Input) -> u64 {
    input.iter().zip(&IMPACT[256]).map(|(fish, impact)| fish * impact).sum()
}
