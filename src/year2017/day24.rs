//! # Electromagnetic Moat
//!
//! Both parts are calculated at the same time by recurively building all possible bridge
//! combinations. Two optimizations are used to speed things up ten times
//!
//! First ports that only appear in two components are merged. For example `2/17` and `17/3`
//! becomes a single component `2/3` with a weight of 39 and a length of 2. This shaves about 30%
//! off the time needed.
//!
//! The second optimization is far more critical and reduces the time needed by 85%. The
//! observation is that compenets with two ports the same, for example `7/7` are always optimal
//! to pick first, as they increase strength and length without changing the port number.
//!
//! If we can place such a component then there's no need to consider further components which
//! reduces the total number of combination to consider.
use crate::util::bitset::*;
use crate::util::iter::*;
use crate::util::parse::*;

struct Component {
    left: usize,
    right: usize,
    weight: usize,
    length: usize,
}

struct State {
    possible: [usize; 64],
    both: [usize; 64],
    weight: [usize; 64],
    length: [usize; 64],
    bridge: [usize; 64],
}

pub fn parse(input: &str) -> [usize; 64] {
    let mut components: Vec<_> = input
        .iter_unsigned()
        .chunk::<2>()
        .map(|[left, right]| Component { left, right, weight: left + right, length: 1 })
        .collect();

    // First optimization. If a port value appears in only 2 components (excluding zero)
    // then fuse the components together.
    let mut indices = Vec::new();

    for n in 1..64 {
        for (index, component) in components.iter().enumerate() {
            if component.left == n || component.right == n {
                indices.push(index);
            }
        }

        if indices.len() == 2 {
            let second = components.swap_remove(indices[1]);
            let first = components.swap_remove(indices[0]);

            let left = if first.left == n { first.right } else { first.left };
            let right = if second.left == n { second.right } else { second.left };
            let weight = first.weight + second.weight;
            let length = first.length + second.length;

            components.push(Component { left, right, weight, length });
        }

        indices.clear();
    }

    // Second optimization. Sort components with both ports the same before other components,
    // so that the loop when choosing componentns in `build` function can terminate early.
    components.sort_unstable_by(|a, b| {
        (a.left ^ a.right).cmp(&(b.left ^ b.right)).then(a.left.cmp(&b.left))
    });

    let mut state = State {
        possible: [0; 64],
        both: [0; 64],
        weight: [0; 64],
        length: [0; 64],
        bridge: [0; 64],
    };

    for (index, component) in components.iter().enumerate() {
        let mask = 1 << index;
        state.possible[component.left] |= mask;
        state.possible[component.right] |= mask;

        // Bitwise logic trick. `a ^ b ^ a = b` and `a ^ b ^ b = a` so given a single port and
        // the XOR of both we can work out the other port of a component.
        state.both[index] = component.left ^ component.right;
        state.weight[index] = component.weight;
        state.length[index] = component.length;
    }

    // Recursively build all possible bridges.
    build(&mut state, 0, 0, 0, 0);
    state.bridge
}

/// Strongest bridge.
pub fn part1(input: &[usize]) -> usize {
    *input.iter().max().unwrap()
}

/// Longest bridge.
pub fn part2(input: &[usize]) -> usize {
    *input.iter().rfind(|&&n| n > 0).unwrap()
}

fn build(state: &mut State, current: usize, used: usize, strength: usize, length: usize) {
    // Bitset of all unused components that have a matching port.
    let remaining = state.possible[current] & !used;

    // Extract the index of each component from the bitset.
    for index in remaining.biterator() {
        let next = current ^ state.both[index];
        let used = used | (1 << index);
        let strength = strength + state.weight[index];
        let length = length + state.length[index];

        if state.possible[next] & !used == 0 {
            // No more possible components to add to the bridge.
            state.bridge[length] = state.bridge[length].max(strength);
        } else {
            build(state, next, used, strength, length);
            // Critical optimization. If this is a component with two ports of the same values,
            // for example 5/5 or 7/7 then it's always optimal to add it to the bridge.
            // We don't need to consider further options.
            if current == next {
                break;
            }
        }
    }
}
