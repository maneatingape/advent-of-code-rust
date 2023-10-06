//! # Wizard Simulator 20XX
//!
//! [Dijkstra's algorithm](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm) is ideal
//! for solving this problem. A node in the graph is our current state and each edge is
//! represented by casting a spell to get to a new state.
//!
//! The key to optimizing is to cache previously seen states. As we receive states in strictly
//! increasing order of mana spent if we see a state again then it cannot possibly be optimal
//! and we can discard.
use crate::util::hash::*;
use crate::util::heap::*;
use crate::util::iter::*;
use crate::util::parse::*;

type Input = [i16; 2];

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    boss_hp: i16,
    player_hp: i16,
    player_mana: i16,
    shield_effect: u8,
    poison_effect: u8,
    recharge_effect: u8,
}

pub fn parse(input: &str) -> Input {
    input.iter_signed().chunk::<2>().next().unwrap()
}

pub fn part1(input: &Input) -> i16 {
    play(*input, false)
}

pub fn part2(input: &Input) -> i16 {
    play(*input, true)
}

fn play(input: Input, hard_mode: bool) -> i16 {
    let [boss_hp, boss_damage] = input;
    let start = State {
        boss_hp,
        player_hp: 50,
        player_mana: 500,
        shield_effect: 0,
        poison_effect: 0,
        recharge_effect: 0,
    };

    let mut todo = MinHeap::new();
    let mut cache = FastSet::with_capacity(5_000);

    todo.push(0, start);
    cache.insert(start);

    while let Some((spent, mut state)) = todo.pop() {
        // Check winning condition
        if apply_spell_effects(&mut state) {
            return spent;
        }

        // Part two
        if hard_mode {
            if state.player_hp > 1 {
                state.player_hp -= 1;
            } else {
                continue;
            }
        }

        // Magic Missile
        if state.player_mana >= 53 {
            let mut next =
                State { boss_hp: state.boss_hp - 4, player_mana: state.player_mana - 53, ..state };

            if apply_spell_effects(&mut next) {
                return spent + 53;
            }
            if boss_turn(&mut next, boss_damage) && cache.insert(next) {
                todo.push(spent + 53, next);
            }
        }

        // Drain
        if state.player_mana >= 73 {
            let mut next = State {
                boss_hp: state.boss_hp - 2,
                player_hp: state.player_hp + 2,
                player_mana: state.player_mana - 73,
                ..state
            };

            if apply_spell_effects(&mut next) {
                return spent + 73;
            }
            if boss_turn(&mut next, boss_damage) && cache.insert(next) {
                todo.push(spent + 73, next);
            }
        }

        // Shield
        if state.player_mana >= 113 && state.shield_effect == 0 {
            let mut next =
                State { player_mana: state.player_mana - 113, shield_effect: 6, ..state };

            if apply_spell_effects(&mut next) {
                return spent + 113;
            }
            if boss_turn(&mut next, boss_damage) && cache.insert(next) {
                todo.push(spent + 113, next);
            }
        }

        // Poison
        if state.player_mana >= 173 && state.poison_effect == 0 {
            let mut next =
                State { player_mana: state.player_mana - 173, poison_effect: 6, ..state };

            if apply_spell_effects(&mut next) {
                return spent + 173;
            }
            if boss_turn(&mut next, boss_damage) && cache.insert(next) {
                todo.push(spent + 173, next);
            }
        }

        // Recharge
        if state.player_mana >= 229 && state.recharge_effect == 0 {
            let mut next =
                State { player_mana: state.player_mana - 229, recharge_effect: 5, ..state };

            if apply_spell_effects(&mut next) {
                return spent + 229;
            }
            if boss_turn(&mut next, boss_damage) && cache.insert(next) {
                todo.push(spent + 229, next);
            }
        }
    }

    unreachable!()
}

/// Applies spell effects to state and returns true if the player has won.
#[inline]
fn apply_spell_effects(state: &mut State) -> bool {
    if state.shield_effect > 0 {
        state.shield_effect -= 1;
    }
    if state.poison_effect > 0 {
        state.poison_effect -= 1;
        state.boss_hp -= 3;
    }
    if state.recharge_effect > 0 {
        state.recharge_effect -= 1;
        state.player_mana += 101;
    }

    state.boss_hp <= 0
}

/// Applies boss attack and returns true if the wizard survives.
#[inline]
fn boss_turn(state: &mut State, mut attack: i16) -> bool {
    if state.shield_effect > 0 {
        attack = (attack - 7).max(1);
    }

    state.player_hp -= attack;
    state.player_hp > 0 && state.player_mana >= 53
}
