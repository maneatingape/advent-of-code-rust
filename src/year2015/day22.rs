//! # Wizard Simulator 20XX
//!
//! [A* algorithm](https://en.wikipedia.org/wiki/A*_search_algorithm) is ideal for solving this
//! problem. A node in the graph is our current state and each edge is represented by casting a
//! spell to get to a new state.
//!
//! The key to optimizing is to cache previously seen states. As we receive states in strictly
//! increasing order of mana spent if we see a state again then it cannot possibly be optimal
//! and we can discard it.  As an additional heuristic, for any given state, we know that we must
//! spend a minimum mana on every turn, and that the game will last for at least as many
//! turns as it requires for maximum damage to deplete the boss's hit points.  This heuristic
//! does not take into account the fact that maximum damage is only possible while Poison is
//! still active, where re-casting Poison costs more mana but can end the game faster; but
//! that's okay: as long as the heuristic is consistent, underestimating is fine.
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
    spent: i16,
}

impl State {
    /// Applies spell effects to state and returns true if the player has won.
    #[inline]
    fn apply_spell_effects(&mut self) -> bool {
        if self.shield_effect > 0 {
            self.shield_effect -= 1;
        }
        if self.poison_effect > 0 {
            self.poison_effect -= 1;
            self.boss_hp -= 3;
        }
        if self.recharge_effect > 0 {
            self.recharge_effect -= 1;
            self.player_mana += 101;
        }

        self.boss_hp <= 0
    }

    /// Applies boss attack and returns true if the wizard survives.
    #[inline]
    fn boss_turn(&mut self, mut attack: i16) -> bool {
        if self.shield_effect > 0 {
            attack = (attack - 7).max(1);
        }

        self.player_hp -= attack;
        self.player_hp > 0 && self.player_mana >= 53
    }
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

fn heuristic(spent: i16, boss_hp: i16) -> i16 {
    // Assume that Poison is still active; this can deal the boss up to 6 damage prior to the next
    // time we can cast.  Beyond that, we must spend mana every turn; the minimum is 53 for Magic
    // Missile; and we need to survive at least as many turns as what the boss will survive even
    // if we have maximum damage per turn (the most damage possible is 6 from Poison and 4 from
    // Magic Missile from here on out).  Since this is a heuristic, it does not matter that it
    // underestimates actual costs needed to keep Poison active, or that the boss will survive
    // longer than the minimum number of turns for every time we cast a different spell.
    let damage_per_turn = 4 + 6; // Poison still active and cast Magic Missile
    let mana_per_turn = 53; // Magic Missile is cheapest to cast
    spent + (boss_hp + (damage_per_turn - 1) - 6) / damage_per_turn * mana_per_turn
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
        spent: 0,
    };

    let mut todo = MinHeap::new();
    let mut cache = FastSet::with_capacity(5_000);

    todo.push(heuristic(0, boss_hp), start);
    cache.insert(start);

    while let Some((_, mut state)) = todo.pop() {
        let spent = state.spent;
        // Check winning condition
        if state.apply_spell_effects() {
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
            let mut next = State {
                boss_hp: state.boss_hp - 4,
                player_mana: state.player_mana - 53,
                spent: spent + 53,
                ..state
            };

            if next.apply_spell_effects() {
                return spent + 53;
            }
            if next.boss_turn(boss_damage) && cache.insert(next) {
                todo.push(heuristic(next.spent, next.boss_hp), next);
            }
        }

        // Drain
        if state.player_mana >= 73 {
            let mut next = State {
                boss_hp: state.boss_hp - 2,
                player_hp: state.player_hp + 2,
                player_mana: state.player_mana - 73,
                spent: spent + 73,
                ..state
            };

            if next.apply_spell_effects() {
                return spent + 73;
            }
            if next.boss_turn(boss_damage) && cache.insert(next) {
                todo.push(heuristic(next.spent, next.boss_hp), next);
            }
        }

        // Shield
        if state.player_mana >= 113 && state.shield_effect == 0 {
            let mut next = State {
                player_mana: state.player_mana - 113,
                shield_effect: 6,
                spent: spent + 113,
                ..state
            };

            if next.apply_spell_effects() {
                return spent + 113;
            }
            if next.boss_turn(boss_damage) && cache.insert(next) {
                todo.push(heuristic(next.spent, next.boss_hp), next);
            }
        }

        // Poison
        if state.player_mana >= 173 && state.poison_effect == 0 {
            let mut next = State {
                player_mana: state.player_mana - 173,
                poison_effect: 6,
                spent: spent + 173,
                ..state
            };

            if next.apply_spell_effects() {
                return spent + 173;
            }
            if next.boss_turn(boss_damage) && cache.insert(next) {
                todo.push(heuristic(next.spent, next.boss_hp), next);
            }
        }

        // Recharge
        if state.player_mana >= 229 && state.recharge_effect == 0 {
            let mut next = State {
                player_mana: state.player_mana - 229,
                recharge_effect: 5,
                spent: spent + 229,
                ..state
            };

            if next.apply_spell_effects() {
                return spent + 229;
            }
            if next.boss_turn(boss_damage) && cache.insert(next) {
                todo.push(heuristic(next.spent, next.boss_hp), next);
            }
        }
    }

    unreachable!()
}
