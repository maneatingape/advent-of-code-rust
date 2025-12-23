//! # Factory
use crate::util::bitset::*;
use crate::util::math::*;
use crate::util::parse::*;
use std::array::from_fn;

const MAX_BUTTONS: usize = 14;
const MAX_JOLTAGES: usize = 11;

type Column = [i32; MAX_JOLTAGES];

pub struct Machine {
    lights: u32,
    buttons: Vec<u32>,
    joltages: Vec<i32>,
}

struct Subspace {
    rank: usize,
    nullity: usize,
    lcm: i32,
    rhs: Column,
    basis: Vec<Basis>,
}

#[derive(Clone, Copy)]
struct Basis {
    limit: i32,
    cost: i32,
    vs: Column,
}

pub fn parse(input: &str) -> Vec<Machine> {
    input.lines().map(parse_machine).collect()
}

pub fn part1(input: &[Machine]) -> u32 {
    let mut todo = Vec::with_capacity(1_000);
    input.iter().map(|machine| configure_lights(machine, &mut todo)).sum()
}

pub fn part2(input: &[Machine]) -> i32 {
    input.iter().map(configure_joltages).sum()
}

/// Convert light patterns and buttons to bitmasks to speed up part one.
fn parse_machine(line: &str) -> Machine {
    let tokens: Vec<_> = line.split_ascii_whitespace().collect();
    let last = tokens.len() - 1;

    let lights = tokens[0][1..]
        .bytes()
        .enumerate()
        .fold(0, |light, (i, b)| light | (u32::from(b == b'#') << i));
    let buttons = tokens[1..last]
        .iter()
        .map(|token| token.iter_unsigned().fold(0, |button, i: u32| button | (1 << i)))
        .collect();
    let joltages = tokens[last].iter_signed().collect();

    Machine { lights, buttons, joltages }
}

/// Check all patterns with one set bit, then patterns with two set bits, and so on,
/// returning early as soon as we find a match without checking all possible combinations.
fn configure_lights(machine: &Machine, todo: &mut Vec<(usize, u32, u32)>) -> u32 {
    todo.clear();
    todo.push((machine.buttons.len(), 0, 0));

    for index in 0.. {
        let (limit, pressed, pattern) = todo[index];

        for i in 0..limit {
            let next_pattern = pattern ^ machine.buttons[i];
            if next_pattern == machine.lights {
                return pressed + 1;
            }
            todo.push((i, pressed + 1, next_pattern));
        }
    }

    unreachable!()
}

/// Convert the buttons and joltages to simultaneous equations,
/// then use [Gaussian Elimination](https://en.wikipedia.org/wiki/Gaussian_elimination)
/// to reduce (the up to 13) dimensions of the full solution space to a (between 0 and 3)
/// dimensional subspace of only the free variables.
fn configure_joltages(machine: &Machine) -> i32 {
    let subspace @ Subspace { rank, nullity, lcm, rhs, .. } = gaussian_elimination(machine);
    let particular_solution = rhs[..rank].iter().sum();

    if nullity == 0 {
        particular_solution / lcm
    } else {
        let remaining = (1 << subspace.basis.len()) - 1;
        recurse(&subspace, rhs, remaining, particular_solution).unwrap()
    }
}

fn gaussian_elimination(machine: &Machine) -> Subspace {
    let Machine { buttons, joltages, .. } = machine;
    let width = buttons.len();
    let height = joltages.len();

    assert!(width < MAX_BUTTONS);
    assert!(height < MAX_BUTTONS);
    let mut equations = [[0; MAX_BUTTONS]; MAX_JOLTAGES];

    for row in 0..height {
        equations[row][width] = joltages[row];
    }

    for col in 0..width {
        let mut limit = i32::MAX;

        for row in buttons[col].biterator() {
            equations[row][col] = 1;
            limit = limit.min(joltages[row]);
        }

        equations[height][col] = limit;
    }

    let mut rank = 0;
    let mut last = width;

    while rank < height && rank < last {
        if let Some(found) = (rank..height)
            .filter(|&row| equations[row][rank] != 0)
            .min_by_key(|&row| equations[row][rank].abs())
        {
            equations.swap(rank, found);
            let mut pivot = equations[rank][rank];

            if pivot < 0 {
                pivot *= -1;
                equations[rank][rank..=width].iter_mut().for_each(|c| *c *= -1);
            }

            for row in 0..height {
                let coefficient = equations[row][rank];
                if row != rank && coefficient != 0 {
                    let lcm = coefficient.abs().lcm(pivot);
                    let x = lcm / coefficient.abs();
                    let y = lcm / pivot * coefficient.signum();

                    for col in 0..equations[row].len() {
                        equations[row][col] = x * equations[row][col] - y * equations[rank][col];
                    }
                }
            }

            rank += 1;
        } else {
            last -= 1;
            equations[..=height].iter_mut().for_each(|row| row.swap(rank, last));
        }
    }

    let lcm = (0..rank).fold(1, |lcm, pivot| lcm.lcm(equations[pivot][pivot]));

    for (pivot, equation) in equations[..rank].iter_mut().enumerate() {
        let q = lcm / equation[pivot];
        equation[rank..=width].iter_mut().for_each(|c| *c *= q);
    }

    let nullity = width - rank;
    let rhs = from_fn(|row| equations[row][width]);
    let basis: Vec<_> = (0..nullity)
        .map(|col| {
            let limit = equations[height][col + rank];
            let vs = from_fn(|row| equations[row][rank + col]);
            let cost = lcm - vs[..rank].iter().sum::<i32>();
            Basis { limit, cost, vs }
        })
        .collect();

    Subspace { rank, nullity, lcm, rhs, basis }
}

fn recurse(subspace: &Subspace, mut rhs: Column, remaining: u32, presses: i32) -> Option<i32> {
    let rank = subspace.rank;
    let mut temp = rhs;

    for i in remaining.biterator() {
        let free = &subspace.basis[i];
        for (row, &v) in free.vs[..rank].iter().enumerate() {
            if v < 0 {
                temp[row] -= v * free.limit;
            }
        }
    }

    let mut min_value = i32::MAX;
    let mut min_index = usize::MAX;
    let mut global_lower = 0;
    let mut global_upper = 0;

    for i in remaining.biterator() {
        let free = &subspace.basis[i];
        let mut lower = 0;
        let mut upper = free.limit;

        for (&v, &rhs) in free.vs[..rank].iter().zip(&temp) {
            if v > 0 {
                upper = upper.min(rhs / v);
            }
            if v < 0 {
                let rhs = rhs + v * free.limit;
                lower = lower.max((rhs + v + 1) / v);
            }
        }

        let size = upper - lower + 1;
        if size > 0 && size < min_value {
            min_value = size;
            min_index = i;
            global_lower = lower;
            global_upper = upper;
        }
    }

    if min_index == usize::MAX {
        return None;
    }

    let remaining = remaining ^ (1 << min_index);
    let lower = global_lower;
    let upper = global_upper;
    let Basis { cost, vs, .. } = &subspace.basis[min_index];
    let cost = *cost;
    let lcm = subspace.lcm;

    if remaining != 0 {
        rhs[..rank].iter_mut().zip(vs).for_each(|(rhs, v)| *rhs -= (lower - 1) * v);

        (lower..upper + 1)
            .filter_map(|n| {
                rhs[..rank].iter_mut().zip(vs).for_each(|(rhs, v)| *rhs -= v);
                recurse(subspace, rhs, remaining, presses + n * cost)
            })
            .min()
    } else if cost >= 0 {
        (lower..upper + 1).find_map(|n| {
            let total = (presses + n * cost) / lcm;
            rhs[..rank].iter().zip(vs).all(|(rhs, v)| (rhs - n * v) % lcm == 0).then_some(total)
        })
    } else {
        (lower..upper + 1).rev().find_map(|n| {
            let total = (presses + n * cost) / lcm;
            rhs[..rank].iter().zip(vs).all(|(rhs, v)| (rhs - n * v) % lcm == 0).then_some(total)
        })
    }
}
