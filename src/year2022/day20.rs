//! # Grove Positioning System
//!
//! We store the numbers as the leaves of an
//! [order statistic tree](https://en.wikipedia.org/wiki/Order_statistic_tree). This variant of
//! a binary tree also stores the size of the subtree at each node, allowing us to find the
//! position of each number and move it to a new location in `log(n)` time.
//!
//! Each node is stored as an element of a `vec` and referred to by index. For example the sample
//! data of 7 numbers would be stored at indices (with size in subscript):
//!
//! ```none
//!                12₇
//!             /       \
//!           10₄        11₃
//!         /   \        / \
//!       7₂     8₂     9₂  \
//!      / \    / \    / \   \
//!     0₁  1₁ 2₁  3₁ 4₁  5₁  6₁
//! ```
//!
//! One very important feature of this specific implementation is that the *index of leaf nodes
//! never changes*. This has some nice benefits:
//! * We don't need to store the value of each node as it's implicit in the index, matching
//!   the index of the original data.
//! * We can find the starting number for each move in `O(1)` constant time, then move the number
//!   in the tree in `log(n)` time for a total complexity of `log(n)`.
//!
//! For example, say we wanted to move the number at index `2` which is `-3` in the sample data.
//! First we calculate the amount to move in both left and right directions (in order to avoid
//! having to wrap around).
//!
//! * Size = `7 - 1` = `6`
//! * Delta Right = `-3.rem_euclid(size)` = `3`
//! * Delta Left = `size - 3` = `6 - 3` = `3`
//!
//! Starting at our parent we then search upwards in the tree until we find a node where either
//! the left or right subtree size is smaller than or equal to the amount
//! (path marked with square brackets).
//! We update the deltas at each step to take into account the size of subtrees we skipped over,
//! also subtracting one from the size of each node, except for the last:
//!
//! ```none
//!               [12₇]
//!             /       \
//!          [10₃]       11₃
//!         /   \        / \
//!       7₂    [8₁]    9₂  \
//!      / \    / \    / \   \
//!     0₁  1₁[2₀] 3₁ 4₁  5₁  6₁
//! ```
//!
//! | Index | Delta Left | Delta Right |
//! | - | - | - |
//! | 8 | 3 | 3 |   Size of 2 reduced to zero.
//! | 10 | 3 | 3 |  Size of 10 reduced to one.
//! | 12 | 4 | 2 |  2 is less than 3
//!
//! As 2 is less than the size of the right subtree, we then we work downwards to the right,
//! until we reach a leaf node adding one to the size of each node except the first and the last.
//! If the delta is greater than the left subtree we go right and subtract the size,
//! otherwise we go left.
//!
//! ```none
//!               [12₇]
//!             /       \
//!           10₃       [11₄]
//!         /   \        / \
//!       7₂     8₁    [9₃] \
//!      / \    / \    / \   \
//!     0₁  1₁ 2₀  3₁ 4₁ [5₁]  6₁
//! ```
//!
//! | iIndex | Delta Right |
//! | - | - |
//! | 11 | 2 |   2 <= 2 go left
//! | 9 | 2 |    2 > 1 go right and subtract 1
//! | 5 | 1 |    Reached leaf node
//!
//! Next we remove the node and its parent from the tree.
//!
//! ```none
//!                12₇
//!             /       \
//!           10₃        11₄
//!         /   \        / \
//!       7₂     3₁     9₃  \
//!      / \           / \   \
//!     0₁  1₁       4₁  5₁   6₁
//! ```
//!
//! Finally we the insert the node and its parent in the new location
//!
//! ```none
//!                12₇
//!             /       \
//!           10₃        11₄
//!         /   \        / \
//!       7₂     3₁     9₃  \
//!      / \           / \   \
//!     0₁  1₁       4₁  8₂   6₁
//!                     / \
//!                    5₁  2₁
//! ```
use crate::util::iter::*;
use crate::util::parse::*;
use std::ops::{Index, IndexMut};

/// Nodes store references to each other as the index in a `vec` of nodes, instead of having to
/// wrap references in a [complicated manner](https://rust-unofficial.github.io/too-many-lists/)
/// to satisy the borrow checker. Another advantage is that each node is compact taking only
/// 8 bytes and stored contiguously which helps memory locality for caching.
#[derive(Clone, Copy, Default)]
struct Node {
    size: u16,
    left: u16,
    right: u16,
    up: u16,
}

/// An important nuance is that the size of the tree to consider when moving a number is the
/// amount of leaf nodes *minus one*.
struct OrderStatisticTree {
    root: u16,
    size: u16,
    nodes: Vec<Node>,
}

/// Syntactic sugar to allow using `u16` values to fetch a node.
impl Index<u16> for OrderStatisticTree {
    type Output = Node;

    #[inline]
    fn index(&self, index: u16) -> &Self::Output {
        &self.nodes[index as usize]
    }
}

impl IndexMut<u16> for OrderStatisticTree {
    #[inline]
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        &mut self.nodes[index as usize]
    }
}

impl OrderStatisticTree {
    /// Build a tree starting from the leaves, then adding one layer at a time until we finish
    /// at the root.
    fn from(size: usize) -> Self {
        let mut start = 0;
        let mut end = size;
        let mut level = size;

        // The number of non-leaf nodes in a balanced binary tree is slightly less than the number
        // of non-leaf nodes.
        let mut nodes = Vec::with_capacity(size * 2);
        let mut remaining = None;

        // The value of leaf nodes is store implicitly as their index is the same as the index of
        // the original input values.
        let empty = u16::MAX;
        let leaf = Node { size: 1, ..Default::default() };
        nodes.resize(size, leaf);

        while level > 0 {
            // Insert a node into the tree, updating the index of its children to point to it.
            let mut push = |left: usize, right: usize| {
                let index = nodes.len() as u16;
                nodes[left].up = index;
                nodes[right].up = index;

                let size = nodes[left].size + nodes[right].size;
                nodes.push(Node { size, up: empty, left: left as u16, right: right as u16 });
            };

            // Take all pairs.
            for [left, right] in (start..end).chunk::<2>() {
                push(left, right);
            }

            // If there's a leftover odd node over then combine with a leftover node from a lower
            // level if available, otherwise save for a higher level.
            if level % 2 == 1 {
                if let Some(right) = remaining {
                    remaining = None;
                    push(end - 1, right);
                } else {
                    remaining = Some(end - 1);
                }
            }

            start = end;
            end = nodes.len();
            level = end - start;
        }

        // The root is initially the last node.
        OrderStatisticTree { root: (end - 1) as u16, size: (size - 1) as u16, nodes }
    }

    /// Find the position (or order) of a leaf node, by starting from the bottom and walking the
    /// tree upwards to the root, counting the size of all subtrees to our left.
    fn position(&self, start: usize) -> usize {
        let mut cur = start as u16;
        let mut offset = 0;

        while cur != self.root {
            let next = self[cur].up;
            let Node { left, right, .. } = self[next];
            if right == cur {
                offset += self[left].size;
            };
            cur = next;
        }

        offset as usize
    }

    /// Find the value at a specific position (or order) by working top down from the root node
    /// until we reach the leaf, subtracting the size of subtrees to our left to find the
    /// remaining offset.
    fn value_at(&self, start: usize) -> usize {
        let mut cur = self.root;
        let mut offset = start as u16;

        loop {
            let Node { size, left, right, .. } = self[cur];

            if size == 1 {
                break cur as usize;
            }

            let size = self[left].size;
            if offset < size {
                cur = left;
            } else {
                cur = right;
                offset -= size;
            }
        }
    }

    /// Moves a number `value` places to the right if value is positive or `value` places to the
    /// left if value is negative.
    fn mix(&mut self, start: usize, value: i64) {
        // Take value modulo the amount of numbers (minus one) as any higher amounts
        // just wrap around. Compute the value for both left and right directions as only one
        // will be possible from our location in the tree.
        let mut delta_right = value.rem_euclid(self.size as i64) as u16;
        let mut delta_left = self.size - delta_right;

        // Zero moves are a no-op.
        if delta_left == 0 || delta_right == 0 {
            return;
        }

        // Temporarily set our size to zero so that we don't count ourselves when calculating
        // distance.
        let start = start as u16;
        self[start].size = 0;

        let parent = self[start].up;
        let mut dest = parent;

        // Go up the tree until we have enough room to move either left or right, whichever comes
        // first. The root node will always have enough room.
        let (left, right) = loop {
            let Node { left, right, up, .. } = self[dest];
            let left_size = self[left].size;
            let right_size = self[right].size;

            if delta_left <= left_size || delta_right <= right_size {
                break (left, right);
            }

            // Count the size of either the left or right subtree as appropriate.
            if self[up].left == dest {
                delta_right -= right_size;
                delta_left += right_size;
            } else {
                delta_right += left_size;
                delta_left -= left_size;
            }

            // We'll be removing the node from this location so adjust size.
            self[dest].size -= 1;
            dest = up;
        };

        // Move down, preferring either the left or right branches depending on which has enough
        // room. This code is almost identical to the `position` function above.
        let (left, right) = if delta_left <= self[left].size {
            // Down left
            dest = left;

            loop {
                let Node { size, left, right, .. } = self[dest];

                if size == 1 {
                    break (start, dest);
                }

                // Increase the size of all ancestors by one.
                self[dest].size += 1;
                let right_size = self[right].size;

                if delta_left <= right_size {
                    dest = right;
                } else {
                    delta_left -= right_size;
                    dest = left;
                }
            }
        } else {
            // Down right
            dest = right;

            loop {
                let Node { size, left, right, .. } = self[dest];

                if size == 1 {
                    break (dest, start);
                }

                // Increase the size of all ancestors by one.
                self[dest].size += 1;
                let left_size = self[left].size;

                if delta_right <= left_size {
                    dest = left;
                } else {
                    delta_right -= left_size;
                    dest = right;
                }
            }
        };

        // Remove node
        let sibling =
            if self[parent].left == start { self[parent].right } else { self[parent].left };

        if parent == self.root {
            // Corner case that could happen if the tree becomes very unbalanced.
            self[sibling].size += 1;
            self.root = sibling;
        } else {
            // Remove both ourselves and our immediate parent. We'll re-use both nodes when
            // re-inserting so that we never need to allocate new nodes.
            let grand_parent = self[parent].up;
            self[sibling].up = grand_parent;

            if self[grand_parent].left == parent {
                self[grand_parent].left = sibling;
            } else {
                self[grand_parent].right = sibling;
            }
        }

        // Insert node into new location.
        let up = self[dest].up;

        // Update our new grand-parent.
        if self[up].left == dest {
            self[up].left = parent;
        } else {
            self[up].right = parent;
        }

        // Re-use the parent and original node so that we don't need to allocate any new nodes.
        self[parent] = Node { size: 2, left, right, up };
        self[left] = Node { size: 1, up: parent, ..Default::default() };
        self[right] = Node { size: 1, up: parent, ..Default::default() };
    }
}

pub fn parse(input: &str) -> Vec<i64> {
    input.iter_signed().collect()
}

pub fn part1(input: &[i64]) -> i64 {
    decrypt(input, 1, 1)
}

pub fn part2(input: &[i64]) -> i64 {
    decrypt(input, 811589153, 10)
}

/// The tree implementation does the heavy lifting so the encryption logic is relatively simple.
fn decrypt(input: &[i64], key: i64, rounds: usize) -> i64 {
    let mut tree = OrderStatisticTree::from(input.len());

    for _ in 0..rounds {
        for (i, n) in input.iter().enumerate() {
            tree.mix(i, n * key);
        }
    }

    let start = input.iter().position(|&n| n == 0).unwrap();
    let zeroth = tree.position(start);
    [1000, 2000, 3000]
        .iter()
        .map(|offset| tree.value_at((zeroth + offset) % input.len()))
        .map(|index| input[index] * key)
        .sum()
}
