//! # Dirac Dice
use crate::util::iter::*;
use crate::util::parse::*;

type Pair = (usize, usize);
type State = (Pair, Pair);

/// Rolling the Dirac dice 3 times results in 27 quantum universes. However the dice total is
/// one of only 7 possible values. Instead of handling 27 values, we encode the possible dice
/// totals with the number of times that they occur. For example a score of 3 (1 + 1 + 1) only
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
pub fn part1(input: &State) -> usize {
    let mut state = *input;
    let mut dice = 6;
    let mut rolls = 0;

    loop {
        // Player position is 0 based from 0 to 9, but score is 1 based from 1 to 10
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
/// recursive universes so that we can re-use the result and avoid unnecessary calculations.
///
/// Each player can be in position 1 to 10 and can have a score from 0 to 20 (as a score of 21
/// ends the game). This is a total of (10 * 21) ^ 2 = 44100 possible states. For speed this
/// can fit in an array with perfect hashing, instead of using a slower `HashMap`.
pub fn part2(input: &State) -> usize {
    let mut cache = vec![None; 44100];
    let (win, lose) = dirac(*input, &mut cache);
    win.max(lose)
}

fn dirac(state: State, cache: &mut [Option<Pair>]) -> Pair {
    let ((player_position, player_score), (other_position, other_score)) = state;

    // Calculate the perfect hash of the state and lookup the cache in case we've seen this before.
    let index = player_position + 10 * other_position + 100 * player_score + 2100 * other_score;
    if let Some(result) = cache[index] {
        return result;
    }

    let helper = |(win, lose), &(dice, frequency)| {
        let next_position = (player_position + dice) % 10;
        let next_score = player_score + next_position + 1;

        if next_score >= 21 {
            (win + frequency, lose)
        } else {
            // Sneaky trick here to handle both players with the same function.
            // We swap the order of players state each turn, so that turns alternate
            // and record the result as (lose, win) instead of (win, lose).
            let next_state = ((other_position, other_score), (next_position, next_score));
            let (next_lose, next_win) = dirac(next_state, cache);
            (win + frequency * next_win, lose + frequency * next_lose)
        }
    };

    // Compute the number of wins and losses from this position and add to the cache.
    let result = DIRAC.iter().fold((0, 0), helper);
    cache[index] = Some(result);
    result
}
