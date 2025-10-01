//! # Sand Slabs
//!
//! Inspecting the input provides a useful insight. The x and y coordinates of bricks are
//! restricted to between to 0 and 9 inclusive so the final shape of the pile will resemble a tall
//! narrow tower similar to a [Jenga game](https://en.wikipedia.org/wiki/Jenga).
//!
//! A second insight is that this is a graph problem in disguise. Sorting the bricks in ascending
//! z order is equivalent to a [topological sort](https://en.wikipedia.org/wiki/Topological_sorting)
//! where each brick is a node and a directed edge links bricks that support other bricks.
//!
//! By iterating over each brick in order its final resting location and supporting bricks can be
//! calculated immediately. For example taking the first 3 example bricks:
//!
//! ```none
//!     Brick               Heights    Indices
//!
//!     1,0,1~1,2,1 <- A    0 1 0      X 0 X    Already in final position
//!                         0 1 0      X 0 X
//!                         0 1 0      X 0 X
//!
//!     0,0,2~2,0,2 <- B    2 2 2      1 1 1    Already in final position
//!                         0 1 0      X 0 X
//!                         0 1 0      X 0 X
//!
//!     0,2,3~2,2,3 <- C    2 2 2      1 1 1    Moves downwards by 1
//!                         0 1 0      X 0 X
//!                         2 2 2      2 2 2
//! ```
//!
//! ## Part One
//!
//! If a brick is supported by only a single brick then the parent brick cannot be safely removed
//! so we mark it as unsafe. Mutiple bricks could potentially be independently supported by a
//! single parent brick so using a boolean flag means that we won't overcount.
//!
//! ## Part Two
//!
//! Unsafe bricks are a [dominator](https://en.wikipedia.org/wiki/Dominator_(graph_theory)) in
//! graph theory as every path from the root (floor) to bricks supported by them must pass through
//! the unsafe node.
//!
//! To count the total number of bricks that fall when all unsafe bricks are removed one at a time
//! we build a linked list of bricks as we iterate through the nodes. Each brick has a `depth`
//! which is the number of unsafe "dominator" nodes that connect it to the root. For example:
//!
//! ```none
//!
//!     Depth   0     1     2     1     0
//!           | A ┬-> B --> C ┬-> D ┬-> E
//!           |   |           |     |
//!     Floor |   └-> F ------┘     |
//!           | G ------------------┘
//! ```
//!
//! * `A` and `G` rest on the floor so their depth is 0 as they can never fall.
//! * `B` and `F` are both supported only by `A` so their depth is 1.
//! * `C` will fall if either `A` or `B` is removed so its depth is 2.
//! * `D` will only fall when `A` is removed. Removing `F` would leave it supported by `B` and `C`
//!   or vice-versa. The common ancestor of the path to the root is `A` so its depth is 1.
//! * `E`'s common ancestor is the floor so its depth is 0.
//!
//! In total `0 (A) + 0 (G) + 1 (B) + 1 (F) + 2 (C) + 1 (D) + 0 (E) = 5` bricks will fall.
use crate::util::iter::*;
use crate::util::parse::*;

type Input = (usize, usize);

pub fn parse(input: &str) -> Input {
    // Parse each brick into an array of 6 elements, one for each coordinate.
    let mut bricks: Vec<_> = input.iter_unsigned::<usize>().chunk::<6>().collect();
    // x and y are limited to 10 in each direction so we can use a fixed size array.
    let mut heights = [0; 100];
    let mut indices = [usize::MAX; 100];

    // Calculate the answer to both parts simultaneously for efficiency.
    let mut safe = vec![true; bricks.len()];
    let mut dominator: Vec<(usize, usize)> = Vec::with_capacity(bricks.len());

    // Sort ascending by lowest z coordinate.
    bricks.sort_unstable_by_key(|b| b[2]);

    for (i, &[x1, y1, z1, x2, y2, z2]) in bricks.iter().enumerate() {
        // Treat the 1D array as a 2D grid.
        let start = 10 * y1 + x1;
        let end = 10 * y2 + x2;
        let step = if y2 > y1 { 10 } else { 1 };
        let height = z2 - z1 + 1;

        // Track what's underneath the brick.
        let mut top = 0;
        let mut previous = usize::MAX;
        let mut underneath = 0;
        let mut parent = 0;
        let mut depth = 0;

        // Find the highest z coordinate underneath the brick looking downwards along the z axis
        // so only considering x and y coordinates.
        for j in (start..end + 1).step_by(step) {
            top = top.max(heights[j]);
        }

        for j in (start..end + 1).step_by(step) {
            if heights[j] == top {
                let index = indices[j];
                if index != previous {
                    previous = index;
                    underneath += 1;

                    if underneath == 1 {
                        (parent, depth) = dominator[previous];
                    } else {
                        // Find common ancestor
                        let (mut a, mut b) = (parent, depth);
                        let (mut x, mut y) = dominator[previous];

                        // The depth must be the same.
                        while b > y {
                            (a, b) = dominator[a];
                        }
                        while y > b {
                            (x, y) = dominator[x];
                        }

                        // Bricks at the same depth could still have different paths from the
                        // root so we need to also check the indices match.
                        while a != x {
                            (a, b) = dominator[a];
                            (x, _) = dominator[x];
                        }

                        (parent, depth) = (a, b);
                    }
                }
            }

            // Update the x-y grid underneath the brick the with the new highest point and index.
            heights[j] = top + height;
            indices[j] = i;
        }

        // Increase depth by one for each dominator node in the path from the root.
        if underneath == 1 {
            safe[previous] = false;
            parent = previous;
            depth = dominator[previous].1 + 1;
        }

        dominator.push((parent, depth));
    }

    let part_one = safe.iter().filter(|&&b| b).count();
    let part_two = dominator.iter().map(|(_, d)| d).sum();
    (part_one, part_two)
}

pub fn part1(input: &Input) -> usize {
    input.0
}

pub fn part2(input: &Input) -> usize {
    input.1
}
