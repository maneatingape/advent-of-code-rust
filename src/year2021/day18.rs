//! # Snailfish
//!
//! The key observation is that snailfish numbers represent
//! [binary trees](https://en.wikipedia.org/wiki/Binary_tree).
//!
//! For example, the first four sample numbers on the problem description look like the following
//! in binary tree form:
//!
//! ```text
//! [1,2]    [[1,2],3]    [9,[8,7]]    [[1,9],[8,5]]
//!   ■          ■            ■              ■
//!  / \        / \          / \           /   \
//! 1   2      ■   3        9   ■         ■     ■
//!           / \              / \       / \   / \
//!          1   2            8   7     1   9 8   5
//! ```
//!
//! The addition rules have an important consequence. Exploding removes two leaf nodes at depth 5
//! and moves them to neighboring nodes. Since exploding repeatedly happens before splitting until
//! there are no more values at depth 5 this means that the tree will never exceed a depth of 5,
//! and even then a depth of 5 is transient.
//!
//! Each level of a tree can contain up to 2ⁿ nodes, so the maximum size of a non-transient
//! snailfish tree is 1 + 2 + 4 + 8 + 16 = 2⁵ - 1 = 31 nodes.
//!
//! This means that we can store each snailfish number as an implicit data structure in a fixed-size
//! array. This is faster, smaller and more convenient than using a traditional struct with
//! pointers. The root node is stored at index 1 (index 0 is unused). For a node at index `i` its
//! left child is at index `2i`, right child at index `2i + 1` and parent at index `i / 2`. As leaf
//! nodes are always greater than or equal to zero, `-1` is used as a special sentinel value for
//! non-leaf nodes.
//!
//! Another optimization is realizing that all of the explode actions before the first split can
//! be pre-computed. Instead of passing two depth-4 numbers to `add()`, we can simplify any depth-4
//! number into depth-3 via explode actions, and track what values it would have spilled left or
//! right had it been part of a larger `add()`. The resulting `Compressed` object is then ready to
//! slide into the left or right half of a new depth-4 tree at the start of `add()`, and all further
//! reduce actions on the sum will be just splits of leaf nodes larger than 9, followed by an
//! explode if the split happened at depth 4.
use crate::util::parse::*;
use crate::util::thread::*;

/// The indices for [in-order traversal](https://en.wikipedia.org/wiki/Tree_traversal) of the first
/// 4 levels of the implicit binary tree stored in an array.
const IN_ORDER: [usize; 30] = [
    2, 4, 8, 16, 17, 9, 18, 19, 5, 10, 20, 21, 11, 22, 23, 3, 6, 12, 24, 25, 13, 26, 27, 7, 14, 28,
    29, 15, 30, 31,
];

#[derive(Clone, Copy, Default)]
struct Snailfish {
    tree: [i32; 32],
}

#[derive(Clone, Copy)]
pub struct Compressed {
    left_spill: i32,
    right_spill: i32,
    nodes: [i32; 14],
}

/// Parse a snailfish number into an implicit binary tree stored in an array.
///
/// Since no number will be greater than 9 initially we can consider each character individually.
/// `[` means move down a level to parse children, `,` means move from left to right node,
/// `]` means move up a level to return to parent and a digit from 0-9 creates a leaf node
/// with that value.
pub fn parse(input: &str) -> Vec<Compressed> {
    input
        .lines()
        .map(|line: &str| {
            let mut tree = [-1; 32];
            let mut i = 1;

            for b in line.bytes() {
                match b {
                    b'[' => i *= 2,
                    b',' => i += 1,
                    b']' => i /= 2,
                    b => tree[i] = b.to_decimal(),
                }
            }
            compress(Snailfish { tree })
        })
        .collect()
}

/// Add all snailfish numbers, reducing to a single magnitude.
pub fn part1(input: &[Compressed]) -> i32 {
    let mut sum = Snailfish::default();

    input
        .iter()
        .copied()
        .reduce(|a, b| {
            sum = add(&a, &b);
            compress(sum)
        })
        .unwrap();

    magnitude(sum)
}

/// Find the largest magnitude of any two snailfish numbers, remembering that snailfish addition
/// is *not* commutative.
pub fn part2(input: &[Compressed]) -> i32 {
    // Build a master list to share among threads.
    let items: Vec<_> = input.iter().enumerate().collect();

    // Use as many cores as possible to parallelize the calculation.
    let result = spawn_parallel_iterator(&items, |iter| {
        iter.flat_map(|(i, a)| {
            items.iter().filter(move |&(j, _)| i != j).map(|(_, b)| magnitude(add(a, b)))
        })
        .max()
    });

    result.into_iter().flatten().max().unwrap()
}

/// Add two snailfish numbers.
///
/// The initial step creates a new root node then makes the numbers the left and right children
/// of this new root node, by copying the respective ranges of the implicit trees.
///
/// We can optimize the rules a little; the first round of explode was already done in creating
/// compressed arguments, and a split runs its own inline explode.
fn add(left: &Compressed, right: &Compressed) -> Snailfish {
    let mut tree = [-1; 32];

    // Copy left into place.
    tree[4..6].copy_from_slice(&left.nodes[0..2]);
    tree[8..12].copy_from_slice(&left.nodes[2..6]);
    tree[16..24].copy_from_slice(&left.nodes[6..14]);

    // Copy right into place.
    tree[6..8].copy_from_slice(&right.nodes[0..2]);
    tree[12..16].copy_from_slice(&right.nodes[2..6]);
    tree[24..32].copy_from_slice(&right.nodes[6..14]);

    // Adjust by the explode spillover between sides.
    match (right.left_spill, left.right_spill) {
        (-1, -1) => (),
        (left_spill, -1) => augment_leaf(&mut tree, left_spill, 23),
        (-1, right_spill) => augment_leaf(&mut tree, right_spill, 24),
        (left_spill, right_spill) => tree[23] += left_spill + right_spill,
    }

    // Now we process all split operations; any further explode actions are done during any split
    // that creates a temporary depth 5.
    let mut full = Snailfish { tree };
    while split(&mut full) {}

    full
}

/// Perform all initial explodes to create a compressed number from a snailfish number.
/// This is a destructive operation, as no caller needs the original afterwards.
fn compress(mut full: Snailfish) -> Compressed {
    let left_spill = full.tree[16];
    for from in 17..31 {
        let to = if from % 2 == 0 { from / 2 - 1 } else { from + 1 };
        let value = full.tree[from];
        if value >= 0 {
            full.tree[from / 2] = 0;
            augment_leaf(&mut full.tree, value, to);
        }
    }
    let right_spill = full.tree[31];
    let mut nodes = [-1; 14];
    nodes.copy_from_slice(&full.tree[2..16]);

    Compressed { left_spill, right_spill, nodes }
}

/// Augment the correct leaf by the given non-negative value. Walks up the tree starting at the
/// given index until finding a leaf node. Storing the tree as an implicit structure has a nice
/// benefit that finding the next left or right node is straightforward.
fn augment_leaf(tree: &mut [i32], value: i32, mut to: usize) {
    while tree[to] == -1 {
        to /= 2;
    }
    tree[to] += value;
}

/// Split a node into two child nodes.
///
/// Search the tree in an *in-order* traversal, splitting the first node over `10` found (if any).
/// We can optimize the rules by immediately exploding if this happens in a node 4 levels deep.
fn split(full: &mut Snailfish) -> bool {
    let tree = &mut full.tree;
    for &i in &IN_ORDER {
        if tree[i] >= 10 {
            if i < 16 {
                // Still room to add another layer of depth.
                tree[2 * i] = tree[i] / 2;
                tree[2 * i + 1] = (tree[i] + 1) / 2;
                tree[i] = -1;
            } else {
                // Avoid going too deep by performing the followup explode now.
                if i > 16 {
                    let value = tree[i] / 2;
                    augment_leaf(tree, value, i - 1);
                }
                if i < 31 {
                    let value = (tree[i] + 1) / 2;
                    augment_leaf(tree, value, i + 1);
                }
                tree[i] = 0;
            }
            return true;
        }
    }
    false
}

/// Calculate the magnitude of a snailfish number in place without using recursion.
///
/// This operation is destructive but much faster than using a recursive approach and acceptable
/// as we no longer need the original snailfish number afterward.
fn magnitude(full: Snailfish) -> i32 {
    let mut tree = full.tree;
    for i in (1..16).rev() {
        if tree[i] == -1 {
            tree[i] = 3 * tree[2 * i] + 2 * tree[2 * i + 1];
        }
    }
    tree[1]
}
