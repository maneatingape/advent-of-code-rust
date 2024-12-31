//! # Snailfish
//!
//! The key observation is that snailfish numbers represent
//! [binary trees](https://en.wikipedia.org/wiki/Binary_tree).
//!
//! For example the first four sample numbers on the problem description look like the following
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
//! and moves them to neighbouring nodes. Since exploding repeatedly happens before splitting until
//! there are no more values at depth 5 this means that the tree will never exceed a depth of 5.
//!
//! Each level of a tree can contain up to 2ⁿ nodes, so the maximum size of a snailfish tree is
//! 1 + 2 + 4 + 8 + 16 + 32 = 2⁶-1 = 63 nodes.
//!
//! This means that we can store each snailfish number as an implicit data structure in a fixed size
//! array. This is faster, smaller and more convenient than using a traditional struct with pointers.
//! The root node is stored at index 0. For a node at index `i` its left child is at index
//! `2i + 1`, right child at index `2i + 2` and parent at index `i / 2`. As leaf nodes are
//! always greater than or equal to zero, `-1` is used as a special sentinel value for non-leaf nodes.
use crate::util::thread::*;
use std::sync::atomic::{AtomicI32, Ordering};

type Snailfish = [i32; 63];

/// The indices for [in-order traversal](https://en.wikipedia.org/wiki/Tree_traversal) of the first
/// 4 levels of the implicit binary tree stored in an array.
const IN_ORDER: [usize; 30] = [
    1, 3, 7, 15, 16, 8, 17, 18, 4, 9, 19, 20, 10, 21, 22, 2, 5, 11, 23, 24, 12, 25, 26, 6, 13, 27,
    28, 14, 29, 30,
];

/// Parse a snailfish number into an implicit binary tree stored in an array.
///
/// Since no number will greater than 9 initially we can consider each character individually.
/// `[` means moves down a level to parse children, `,` means move from left to right node,
/// `]` means move up a level to return to parent and a digit from 0-9 creates a leaf node
/// with that value.
pub fn parse(input: &str) -> Vec<Snailfish> {
    fn helper(bytes: &[u8]) -> Snailfish {
        let mut tree = [-1; 63];
        let mut i = 0;

        for &b in bytes {
            match b {
                b'[' => i = 2 * i + 1,
                b',' => i += 1,
                b']' => i = (i - 1) / 2,
                b => tree[i] = (b - 48) as i32,
            }
        }

        tree
    }
    input.lines().map(|line| helper(line.as_bytes())).collect()
}

/// Add all snailfish numbers, reducing to a single magnitude.
pub fn part1(input: &[Snailfish]) -> i32 {
    let mut sum = input.iter().copied().reduce(|acc, n| add(&acc, &n)).unwrap();
    magnitude(&mut sum)
}

/// Find the largest magnitude of any two snailfish numbers, remembering that snailfish addition
/// is *not* commutative.
pub fn part2(input: &[Snailfish]) -> i32 {
    let mut pairs = Vec::new();

    for (i, a) in input.iter().enumerate() {
        for (j, b) in input.iter().enumerate() {
            if i != j {
                pairs.push((a, b));
            }
        }
    }

    // Use as many cores as possible to parallelize the calculation.
    let shared = AtomicI32::new(0);
    spawn_parallel_iterator(&pairs, |iter| worker(&shared, iter));
    shared.load(Ordering::Relaxed)
}

/// Pair addition is independent so we can parallelize across multiple threads.
fn worker(shared: &AtomicI32, iter: ParIter<'_, (&Snailfish, &Snailfish)>) {
    let mut partial = 0;

    for (a, b) in iter {
        partial = partial.max(magnitude(&mut add(a, b)));
    }

    shared.fetch_max(partial, Ordering::Relaxed);
}

/// Add two snailfish numbers.
///
/// The initial step creates a new root node then makes the numbers the left and right children
/// of this new root node, by copying the respective ranges of the implicit trees.
///
/// We can optimize the rules a little. This initial combination is the only time that more than one
/// pair will be 4 levels deep simultaneously, so we can sweep from left to right on all possible
/// leaf nodes in one pass.
fn add(left: &Snailfish, right: &Snailfish) -> Snailfish {
    let mut tree = [-1; 63];

    tree[3..5].copy_from_slice(&left[1..3]);
    tree[7..11].copy_from_slice(&left[3..7]);
    tree[15..23].copy_from_slice(&left[7..15]);
    tree[31..47].copy_from_slice(&left[15..31]);

    tree[5..7].copy_from_slice(&right[1..3]);
    tree[11..15].copy_from_slice(&right[3..7]);
    tree[23..31].copy_from_slice(&right[7..15]);
    tree[47..63].copy_from_slice(&right[15..31]);

    for pair in (31..63).step_by(2) {
        if tree[pair] >= 0 {
            explode(&mut tree, pair);
        }
    }

    while split(&mut tree) {}
    tree
}

/// Explode a specific pair identified by an index.
///
/// Storing the tree as an implicit structure has a nice benefit that finding the next left or right
/// node is straightforward. We first move to the next left or right leaf node by adding or
/// subtracting one to the index. If this node is empty then we move to the parent node until we
/// find a leaf node.
///
/// The leaf node at index 31 has no possible nodes to the left and similarly the leaf node at
/// index 62 has no possible nodes to the right.
fn explode(tree: &mut Snailfish, pair: usize) {
    if pair > 31 {
        let mut i = pair - 1;
        loop {
            if tree[i] >= 0 {
                tree[i] += tree[pair];
                break;
            }
            i = (i - 1) / 2;
        }
    }

    if pair < 61 {
        let mut i = pair + 2;
        loop {
            if tree[i] >= 0 {
                tree[i] += tree[pair + 1];
                break;
            }
            i = (i - 1) / 2;
        }
    }

    tree[pair] = -1;
    tree[pair + 1] = -1;
    tree[(pair - 1) / 2] = 0;
}

/// Split a node into two child nodes.
///
/// Search the tree in an *in-order* traversal, splitting the first node over `10` found (if any).
/// We can optimize the rules by immediately exploding if this results in a node 4 levels deep,
/// as we know that the prior optimzation in the [`add`] function means that this is the only
/// explosion possible.
fn split(tree: &mut Snailfish) -> bool {
    for &i in &IN_ORDER {
        if tree[i] >= 10 {
            tree[2 * i + 1] = tree[i] / 2;
            tree[2 * i + 2] = (tree[i] + 1) / 2;
            tree[i] = -1;

            if i >= 15 {
                explode(tree, 2 * i + 1);
            }
            return true;
        }
    }
    false
}

/// Calculate the magnitude of a snailfish number in place without using recursion.
///
/// This operation is destructive but much faster than using a recursive approach and acceptable
/// as we no longer need the original snailfish number afterwards.
fn magnitude(tree: &mut Snailfish) -> i32 {
    for i in (0..31).rev() {
        if tree[i] == -1 {
            tree[i] = 3 * tree[2 * i + 1] + 2 * tree[2 * i + 2];
        }
    }
    tree[0]
}
