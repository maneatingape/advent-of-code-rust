//! # Marble Mania
//!
//! Efficient solution using an append only `vec` and generating only the minimum number of marbles
//! needed to play the game.
//!
//! First let's consider some other slower approaches:
//!
//! We could store marbles in a `vec`, inserting and removing elements to make room. Each of these
//! operations takes `O(n)` complexity. For part two if number of marbles is 100,000 then the
//! total complexity is `100,000 * 100 * 100,000 = 10¹²` which is infeasible.
//!
//! A better approach is a linked list. Insert and remove operations are now `O(1)` for a total
//! part two complexity of `100,000 * 1 * 100  = 10⁷`. This is slow but practical. However linked
//! lists have a number of drawbacks:
//!
//! 1. Poor cache locality
//! 2. Allocation per element
//! 3. Ownership issues complex enough to inspire an entire
//!    [blog post series](https://rust-unofficial.github.io/too-many-lists/).
//!
//! ## First optimization
//!
//! The first key insight is that we can generate the marble sequence by only appending to a `vec`.
//! We keep track of the head `()` and tail `<>` of the circle. Each turn adds two marbles to the
//! head and removes one from the tail, growing the circle by one each time.
//! For example the first 4 marbles look like:
//!
//! ```none
//!    <0>
//!     0  <0> (1)
//!     0   0  <1>  0  (2)
//!     0   0   1  <0>  2  1  (3)
//!     0   0   1   0  <2>  1  3  0  (4)
//! ```
//!
//! Things start to get interesting at the 19th marble. When we pick the 23rd marble this will
//! be 7 places counter clockwise, so we can optimize by not adding it at all the the circle.
//! Instead we save the value for later.
//!
//! ```none
//!     18th marble
//!     ...<9>  2  10   5  11   1  12   6  13   3  14   7  15   0  16   8  17   4  (18)
//!
//!     19th marble, saving value of previous tail 9.
//!     ...<2> 10   5  11   1  12   6  13   3  14   7  15   0  16   8  17   4  18  (19)
//! ```
//!
//! For the 20th, 21st and 22nd marbles we re-write the history of the tail then move it backwards.
//!
//! ```none
//!     20th marble
//!     ... 2  20   9  <2> 10   5  11   1  12   6  13   3  14   7  15   0  16   8  17   4  18  (19)
//!         ^  ^^
//!
//!     21st marble
//!     ... 2  20  10 <21> 10   5  11   1  12   6  13   3  14   7  15   0  16   8  17   4  18  (19)
//!                ^^  ^^
//!
//!     22nd marble (move tail)
//!     ...<2> 20  10  21   5  22  11   1  12   6  13   3  14   7  15   0  16   8  17   4  18  (19)
//!                         ^  ^^
//! ```
//!
//! The 23rd marble is never added to the circle instead increasing the current player's score.
//! The cycle then begins again, handling the next 18 marbles normally, then the next 19th to 22nd
//! marbles specially.
//!
//! ## Second optimization
//!
//! It may seem that we need to generate `(last marble / 23)` blocks. However in each block we add
//! 37 marbles (2 each for the first 18 marbles and 1 for the 19th) while the marble added to each
//! player's score advances `23 - 7 = 16` marbles. This means we only need to generate about
//!  `16/37` or `44%` of the total blocks to solve the game deterministcally. This saves both
//! processing time and memory storage proportionally.
use crate::util::iter::*;
use crate::util::parse::*;

type Input = [usize; 2];

pub fn parse(input: &str) -> Input {
    input.iter_unsigned().chunk::<2>().next().unwrap()
}

pub fn part1(input: &Input) -> u64 {
    let [players, last] = *input;
    game(players, last)
}

pub fn part2(input: &Input) -> u64 {
    let [players, last] = *input;
    game(players, last * 100)
}

fn game(players: usize, last: usize) -> u64 {
    // Play the game in blocks of 23.
    let blocks = last / 23;
    // The number of marbles needed for scoring.
    let needed = 2 + 16 * blocks;
    // Each block adds 37 marbles, so allow a little extra capacity to prevent reallocation.
    let mut circle: Vec<u32> = Vec::with_capacity(needed + 37);
    // The score for each block is deterministic so the number of players only affects how scores
    // are distributed. Type is `u64` to prevent overflow during part two.
    let mut scores = vec![0; players];
    // The first marble picked up and removed by the player is 9.
    let mut pickup = 9;
    // The first block is pre-generated, so we start at marble 23.
    let mut head = 23;
    // Keep track of previous marbles to re-add to the start of the circle and for scoring.
    let mut tail = 0;
    // Add pre-generated marbles for first block.
    let start = [2, 20, 10, 21, 5, 22, 11, 1, 12, 6, 13, 3, 14, 7, 15, 0, 16, 8, 17, 4, 18, 19];
    circle.extend_from_slice(&start);

    for _ in 0..blocks {
        // Score the previous block.
        scores[head as usize % players] += (head + pickup) as u64;
        // The next marble picked up is from the current block.
        pickup = circle[tail + 18];

        // Generate the next block only until we have enough marbles to finish the game.
        if circle.len() <= needed {
            // Extending a vector from a slice is faster than adding elements one at a time.
            let slice = &[
                circle[tail],
                head + 1,
                circle[tail + 1],
                head + 2,
                circle[tail + 2],
                head + 3,
                circle[tail + 3],
                head + 4,
                circle[tail + 4],
                head + 5,
                circle[tail + 5],
                head + 6,
                circle[tail + 6],
                head + 7,
                circle[tail + 7],
                head + 8,
                circle[tail + 8],
                head + 9,
                circle[tail + 9],
                head + 10,
                circle[tail + 10],
                head + 11,
                circle[tail + 11],
                head + 12,
                circle[tail + 12],
                head + 13,
                circle[tail + 13],
                head + 14,
                circle[tail + 14],
                head + 15,
                circle[tail + 15],
                head + 16,
                circle[tail + 16],
                head + 17,
                circle[tail + 17],
                head + 18,
                // circle[tail + 18] 19th marble is picked up and removed.
                head + 19,
            ];
            circle.extend_from_slice(slice);

            // Overwrite the tail for the 20th, 21st and 22nd marbles.
            let slice = &[
                circle[tail + 19],
                head + 20,
                circle[tail + 20],
                head + 21,
                circle[tail + 21],
                head + 22,
            ];
            circle[tail + 16..tail + 22].copy_from_slice(slice);
        }

        // Marbles increase by 23 per block but the tail only by 16 as we reset by 7 marbles
        // according to the rules.
        head += 23;
        tail += 16;
    }

    *scores.iter().max().unwrap()
}
