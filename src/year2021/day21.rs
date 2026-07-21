//! # Dirac Dice
use crate::util::iter::*;
use crate::util::parse::*;

type Pair = (usize, usize);
type State = (Pair, Pair);

/// Rolling the Dirac dice 3 times results in 27 quantum universes. However, the dice total is
/// one of only 7 possible values. Instead of handling 27 values, we encode the possible dice
/// totals with the number of times that they occur. For example, a score of 3 (1 + 1 + 1) only
/// happens once in the 27 rolls, but a score of 6 happens a total of 7 times.
const DIRAC: [Pair; 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

/// Extract the starting position for both players converting to zero-based indices.
pub fn parse(input: &str) -> State {
    let [_, one, _, two]: [usize; 4] = input.iter_unsigned().chunk::<4>().next().unwrap();
    ((one - 1, 0), (two - 1, 0))
}

/// The initial deterministic dice roll total is 6 (1 + 2 + 3) and increases by 9 each turn.
/// An interesting observation is that since the player's position is always modulo 10, we can
/// also increase the dice total modulo 10, as (a + b) % 10 = (a % 10) + (b % 10).
/// Additionally, both players end up in the same position every 10 moves, so we can pre-compute
/// the score per 10 moves before simulating the remainder.
pub fn part1(input: &State) -> usize {
    let mut state = *input;
    let mut dice = 6;
    let ((player_position, _), (other_position, _)) = state;

    // Player 1 loops through offsets 6, 10, 2, 2, 10, 6, 10, 2, 2, 10.
    let skip_one = 4 * ((player_position + 2) % 10 + 1)
        + 2 * ((player_position + 6) % 10 + 1)
        + 4 * (player_position + 1);
    // Player 2 cycles through offsets 5, 8, 9, 8, 5, 10, 3, 4, 3, 10.
    let skip_two = 2 * ((other_position + 3) % 10 + 1)
        + ((other_position + 4) % 10 + 1)
        + 2 * ((other_position + 5) % 10 + 1)
        + 2 * ((other_position + 8) % 10 + 1)
        + ((other_position + 9) % 10 + 1)
        + 2 * (other_position + 1);
    let skips = 999 / skip_one.max(skip_two);
    let mut rolls = skips * 60;
    state = ((player_position, skip_one * skips), (other_position, skip_two * skips));

    loop {
        // Player position is 0 based from 0 to 9, but score is 1 based from 1 to 10.
        let ((player_position, player_score), (other_position, other_score)) = state;
        let next_position = (player_position + dice) % 10;
        let next_score = player_score + next_position + 1;

        dice = (dice + 9) % 10;
        rolls += 3;

        // Return the score of the losing player times the number of dice rolls.
        if next_score >= 1000 {
            break other_score * rolls;
        }

        // Swap the players so that they take alternating turns.
        state = ((other_position, other_score), (next_position, next_score));
    }
}

/// [Memoization](https://en.wikipedia.org/wiki/Memoization) is the key to solving part two in a
/// reasonable time. For each possible starting universe we record the number of winning and losing
/// recursive universes so that we can reuse the result and avoid unnecessary calculations.
///
/// Each player can be in position 1 to 10 and can have a score from 0 to 20 (as a score of 21
/// ends the game). This is a total of (10 × 21)² = 44,100 possible states. For speed this
/// can fit in an array with perfect hashing, instead of using a slower `HashMap`.
///
/// In fact, the cache can be computed and utilized only at compile time; there are only 100 possible
/// starting locations.
pub fn part2(input: &State) -> usize {
    let ((player_position, _), (other_position, _)) = *input;
    ANSWER_TABLE[player_position][other_position]
}

const fn flat_index(
    player_position: usize,
    other_position: usize,
    player_score: usize,
    other_score: usize,
) -> usize {
    player_position + 10 * other_position + 100 * player_score + 2100 * other_score
}

const fn compute_cache() -> [(usize, usize); 44_100] {
    let mut cache = [(0, 0); 44_100];

    // Iterate in reverse by total score, so that dependencies are available.
    let mut total_score = 40;
    loop {
        let mut player_score = 0;
        while player_score < 21 {
            if total_score >= player_score && (total_score - player_score) < 21 {
                let other_score = total_score - player_score;

                let mut player_position = 0;
                while player_position < 10 {
                    let mut other_position = 0;
                    while other_position < 10 {
                        let mut player_wins = 0;
                        let mut other_wins = 0;

                        let mut i = 0;
                        while i < DIRAC.len() {
                            let (dice, frequency) = DIRAC[i];
                            let next_position = (player_position + dice) % 10;
                            let next_score = player_score + next_position + 1;

                            if next_score >= 21 {
                                player_wins += frequency;
                            } else {
                                let idx = flat_index(
                                    other_position,
                                    next_position,
                                    other_score,
                                    next_score,
                                );
                                let (next_other_wins, next_player_wins) = cache[idx];
                                player_wins += next_player_wins * frequency;
                                other_wins += next_other_wins * frequency;
                            }
                            i += 1;
                        }

                        let idx =
                            flat_index(player_position, other_position, player_score, other_score);
                        cache[idx] = (player_wins, other_wins);

                        other_position += 1;
                    }
                    player_position += 1;
                }
            }
            player_score += 1;
        }
        if total_score == 0 {
            break;
        }
        total_score -= 1;
    }

    cache
}

const ANSWER_TABLE: [[usize; 10]; 10] = {
    let cache = compute_cache();
    let mut table = [[0; 10]; 10];

    let mut player_position = 0;
    while player_position < 10 {
        let mut other_position = 0;
        while other_position < 10 {
            let (wins, losses) = cache[flat_index(player_position, other_position, 0, 0)];
            table[player_position][other_position] = if wins > losses { wins } else { losses };
            other_position += 1;
        }
        player_position += 1;
    }

    table
};
