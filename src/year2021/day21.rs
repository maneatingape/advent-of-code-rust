//! # Dirac Dice
use crate::util::iter::*;
use crate::util::parse::*;

/// Rolling the Dirac dice 3 times results in 27 quantum universes. However, the dice total is
/// one of only 7 possible values. Instead of handling 27 values, we encode the possible dice
/// totals with the number of times that they occur. For example, a score of 3 (1 + 1 + 1) only
/// happens once in the 27 rolls, but a score of 6 happens a total of 7 times.
const DIRAC: [Pair; 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

type Pair = (usize, usize);

/// Extract the starting position for both players converting to zero-based indices.
pub fn parse(input: &str) -> Pair {
    let [_, one, _, two]: [usize; 4] = input.iter_unsigned().chunk::<4>().next().unwrap();
    (one - 1, two - 1)
}

/// The initial deterministic dice roll total is 6 (1 + 2 + 3) and increases by 9 each turn.
/// An interesting observation is that since the player's position is always modulo 10, we can
/// also increase the dice total modulo 10, as (a + b) % 10 = (a % 10) + (b % 10). Additionally,
/// both players end up back in the same position every 10 moves, so we can compute the score per
/// batch of 10 moves before simulating only the remainder.
pub fn part1(input: &Pair) -> usize {
    let mut dice = 6;
    let (player, other) = *input;

    // Utilize the periodic visitation pattern to compute the 10-turn score increase per player.
    let batch_score = |position, offsets: &[usize]| -> usize {
        offsets.iter().map(|offset| (position + offset) % 10 + 1).sum()
    };
    let player_batch = batch_score(player, &[6, 0, 2, 2, 0, 6, 0, 2, 2, 0]);
    let other_batch = batch_score(other, &[5, 8, 9, 8, 5, 0, 3, 4, 3, 0]);

    let batches = 999 / player_batch.max(other_batch);
    let mut rolls = batches * 60; // 2 players * 3 dice * 10 turns per batch.
    let mut state = ((player, player_batch * batches), (other, other_batch * batches));

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

/// Handle each player seperately. The number of universes for each player grows independently.
/// When a player wins, then multiply the number of states by the other player's states.
pub fn part2(input: &Pair) -> usize {
    let (player, other) = *input;

    let mut current = [[[0; 21]; 10], [[0; 21]; 10]];
    let mut wins = [0; 2];
    let mut universes = 1;
    let mut turn = 0;

    current[0][player][0] = 1;
    current[1][other][0] = 1;

    while universes > 0 {
        let mut next = [[0; 21]; 10];

        for (dice, frequency) in DIRAC {
            for (position, row) in current[turn].iter().enumerate() {
                let next_position = (position + dice) % 10;
                let (todo, won) = row.split_at(20 - next_position);

                for (score, &amount) in todo.iter().enumerate() {
                    next[next_position][score + next_position + 1] += frequency * amount;
                }

                wins[turn] += universes * frequency * won.iter().sum::<usize>();
            }
        }

        current[turn] = next;
        universes = current[turn].iter().flatten().sum();
        turn ^= 1;
    }

    wins[0].max(wins[1])
}
