use crate::util::iter::*;
use crate::util::parse::*;
use std::ops::{Index, IndexMut};

#[derive(Clone, Copy, Default)]
struct Node {
    size: u16,
    left: u16,
    right: u16,
    up: u16,
}

struct OrderStatisticTree {
    root: u16,
    size: u16,
    nodes: Vec<Node>,
}

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
    fn from(size: usize) -> Self {
        let mut start = 0;
        let mut end = size;
        let mut level = size;

        let mut nodes = Vec::with_capacity(size * 2);
        let mut remaining = None;

        let empty = u16::MAX;
        let leaf = Node { size: 1, ..Default::default() };
        nodes.resize(size, leaf);

        while level > 0 {
            let mut push = |left: usize, right: usize| {
                let index = nodes.len() as u16;
                nodes[left].up = index;
                nodes[right].up = index;

                let size = nodes[left].size + nodes[right].size;
                nodes.push(Node { size, up: empty, left: left as u16, right: right as u16 });
            };

            for [left, right] in (start..end).chunk::<2>() {
                push(left, right);
            }

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

        OrderStatisticTree { root: (end - 1) as u16, size: (size - 1) as u16, nodes }
    }

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

    fn mix(&mut self, start: usize, value: i64) {
        let mut delta_right = value.rem_euclid(self.size as i64) as u16;
        let mut delta_left = self.size - delta_right;

        if delta_left == 0 || delta_right == 0 {
            return;
        }

        // Temporarily set our size to zero
        let start = start as u16;
        self[start].size = 0;

        let parent = self[start].up;
        let mut dest = parent;

        // Go up
        let (left, right) = loop {
            let Node { left, right, up, .. } = self[dest];
            let left_size = self[left].size;
            let right_size = self[right].size;

            if delta_left <= left_size || delta_right <= right_size {
                break (left, right);
            }

            if self[up].left == dest {
                delta_right -= right_size;
                delta_left += right_size;
            } else {
                delta_right += left_size;
                delta_left -= left_size;
            }

            self[dest].size -= 1;
            dest = up;
        };

        let (left, right) = if delta_left <= self[left].size {
            // Down left
            dest = left;

            loop {
                let Node { size, left, right, .. } = self[dest];

                if size == 1 {
                    break (start, dest);
                }

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
            self[sibling].size += 1;
            self.root = sibling;
        } else {
            let grand_parent = self[parent].up;
            self[sibling].up = grand_parent;

            if self[grand_parent].left == parent {
                self[grand_parent].left = sibling;
            } else {
                self[grand_parent].right = sibling;
            }
        }

        // Insert node
        let up = self[dest].up;

        if self[up].left == dest {
            self[up].left = parent;
        } else {
            self[up].right = parent;
        }

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
