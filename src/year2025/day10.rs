//! # Factory
use crate::util::bitset::*;
use crate::util::parse::*;
use crate::util::thread::*;
use std::collections::BTreeSet;

type Machine = (usize, Vec<usize>, Vec<i32>);
type Input = (Vec<i32>, Vec<i32>);

/// Each machine can be processed independently, parallelizing the work over multiple threads.
pub fn parse(input: &str) -> Input {
    let items: Vec<_> = input.lines().collect();

    let result: Vec<Vec<_>> = spawn_parallel_iterator(&items, |iter| {
        iter.map(|line| {
            let machine = parse_machine(line);
            (configure_lights(&machine), configure_joltages(&machine))
        })
        .collect()
    });

    result.into_iter().flatten().unzip()
}

pub fn part1(input: &Input) -> i32 {
    let (presses, _) = input;
    presses.iter().sum()
}

pub fn part2(input: &Input) -> i32 {
    let (_, presses) = input;
    presses.iter().sum()
}

/// Convert light patterns and buttons to bitmasks to speed up part one.
fn parse_machine(line: &str) -> Machine {
    let tokens: Vec<_> = line.split_ascii_whitespace().collect();
    let last = tokens.len() - 1;

    let lights = tokens[0]
        .bytes()
        .skip(1)
        .enumerate()
        .fold(0, |light, (i, b)| light | (usize::from(b == b'#') << i));
    let buttons = tokens[1..last]
        .iter()
        .map(|token| token.iter_unsigned::<usize>().fold(0, |button, i| button | (1 << i)))
        .collect();
    let joltages = tokens[last].iter_signed::<i32>().collect();

    (lights, buttons, joltages)
}

/// Check all patterns with one set bit, then pattern with two sets bits and so on, until we find
/// a match.
fn configure_lights((lights, buttons, _): &Machine) -> i32 {
    let limit = 1 << buttons.len();
    let mut set = 0;

    loop {
        set += 1;
        let mut n = (1 << set) - 1;

        while n < limit {
            if *lights == n.biterator().fold(0, |acc, i| acc ^ buttons[i]) {
                return set;
            }
            n = next_same_bits(n);
        }
    }
}

/// Find the next highest integer with the same number of one bits as the previous integer,
/// for example 1011 => 1110.
fn next_same_bits(n: i32) -> i32 {
    let smallest = n & -n;
    let ripple = n + smallest;
    let ones = n ^ ripple;
    let next = (ones >> 2) / smallest;
    ripple | next
}

/// Convert the buttons and joltages to equations, then use
/// [Gaussian Elimination](https://en.wikipedia.org/wiki/Gaussian_elimination) to reduce the
/// dimensioanlity of the problem to only the free variables.
fn configure_joltages((_, buttons, joltages): &Machine) -> i32 {
    let width = buttons.len();
    let height = joltages.len();
    let mut equations = vec![vec![0; width + 1]; height];
    let mut limit = vec![i32::MAX; width];
    let mut previous: BTreeSet<_> = (0..width).collect();
    let mut current: BTreeSet<_> = BTreeSet::new();

    // If a button can increment a joltage counter then it get a coefficent of 1, otherwise zero.
    // Using the first example machine and labelling the button a..f from left to right:
    //
    // * [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    // * d + f = 3
    // * b + f = 5
    // * c + d + e = 4
    // * a + b + d = 7
    for (row, &joltage) in joltages.iter().enumerate() {
        equations[row][width] = joltage;

        for col in 0..width {
            if buttons[col] & (1 << row) != 0 {
                equations[row][col] = 1;
                limit[col] = limit[col].min(joltage);
            }
        }
    }

    // Gaussian elimination to reduce the matrix to row echelon form. For example the previous
    // equations form a matrix:
    //
    // [ 0  0  0  1  0  1 | 3 ]
    // [ 0  1  0  0  0  1 | 4 ]
    // [ 0  0  1  1  1  0 | 5 ]
    // [ 1  0  0  1  0  0 | 7 ]
    while previous != current {
        previous = current;
        current = (0..width).collect();

        let mut pivot_row = 0;
        let mut pivot_col = 0;

        while pivot_row < height && pivot_col < width {
            let Some(found) = (pivot_row..height).find(|&row| {
                let coefficient = equations[row][pivot_col];
                coefficient != 0 && equations[row].iter().all(|c| c % coefficient == 0)
            }) else {
                pivot_col += 1;
                continue;
            };

            equations.swap(pivot_row, found);
            let coefficient = equations[pivot_row][pivot_col];
            equations[pivot_row].iter_mut().for_each(|c| *c /= coefficient);

            for row in 0..height {
                if row != pivot_row {
                    let coefficient = equations[row][pivot_col];
                    let [from, to] = equations.get_disjoint_mut([pivot_row, row]).unwrap();
                    from.iter().zip(to).for_each(|(f, t)| *t -= coefficient * f);
                }
            }

            current.remove(&pivot_col);
            pivot_row += 1;
            pivot_col += 1;
        }
    }

    if current.is_empty() {
        return (0..height).map(|row| equations[row][width]).sum();
    }

    // Brute force search over the free variables.
    let free = current.len();
    let fixed = width - current.len();
    let presses = (0..fixed).map(|row| equations[row][width]).sum::<i32>();
    let mut cost = vec![0; free];
    let mut coefficients = vec![vec![0; height]; free];
    let mut rhs = vec![vec![0; height]; free];
    let mut ordered_limit: Vec<_> = vec![0; free];

    for (to, &from) in current.iter().enumerate() {
        cost[to] = 1 - (0..fixed).map(|row| equations[row][from]).sum::<i32>();
        ordered_limit[to] = limit[from];

        for row in 0..height {
            coefficients[to][row] = equations[row][from];
        }
    }

    for row in 0..height {
        rhs[0][row] = equations[row][width];
    }

    recurse(&cost, &ordered_limit, &coefficients, &mut rhs, fixed, presses, 0).unwrap()
}

fn recurse(
    cost: &[i32],
    limit: &[i32],
    coefficients: &[Vec<i32>],
    rhs: &mut [Vec<i32>],
    fixed: usize,
    presses: i32,
    depth: usize,
) -> Option<i32> {
    let height = rhs[depth].len();

    if depth == coefficients.len() - 1 {
        // For the last free variables, we can use the remaining inequalities (and possibily and
        // equalities to find the answer immediately) without needed another search. This
        // reduces the dimensions of the search space by one.
        let mut lower = 0;
        let mut upper = limit[depth];

        // Check inequalites
        for (&coef, &rhs) in coefficients[depth].iter().zip(&rhs[depth]) {
            if rhs >= 0 {
                if coef > 0 {
                    upper = upper.min(rhs / coef);
                }
            } else if coef < 0 {
                let floor = (rhs + coef + 1) / coef;
                lower = lower.max(floor);
            } else {
                upper = -1;
            }
        }

        // Check equalities (if they exist)
        for row in fixed..height {
            let c = coefficients[depth][row];
            let r = rhs[depth][row];

            if c != 0 {
                if r % c == 0 {
                    upper = upper.min(r / c);
                    lower = lower.max(r / c);
                } else {
                    upper = -1;
                }
            }
        }

        let presses = presses + cost[depth] * if cost[depth] >= 0 { lower } else { upper };
        (lower <= upper).then_some(presses)
    } else {
        (0..=limit[depth])
            .filter_map(|x| {
                let next_presses = presses + x * cost[depth];

                for row in 0..height {
                    rhs[depth + 1][row] = rhs[depth][row] - x * coefficients[depth][row];
                }

                recurse(cost, limit, coefficients, rhs, fixed, next_presses, depth + 1)
            })
            .min()
    }
}
