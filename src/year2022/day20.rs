use crate::util::iter::*;
use crate::util::parse::*;
use std::ops::{Index, IndexMut};

#[derive(Clone, Copy)]
struct Node {
    size: u16,
    left: u16,
    right: u16,
    up: u16,
}

struct Tree {
    root: u16,
    size: i64,
    nodes: Vec<Node>,
}

impl Index<u16> for Tree {
    type Output = Node;

    #[inline]
    fn index(&self, index: u16) -> &Self::Output {
        &self.nodes[index as usize]
    }
}

impl IndexMut<u16> for Tree {
    #[inline]
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        &mut self.nodes[index as usize]
    }
}

impl Tree {
    fn from(input: &[i64]) -> Tree {
        let mut size = input.len();
        let mut start = 0;
        let mut end = size;

        let mut remaining = None;
        let mut nodes = Vec::with_capacity(size * 2);

        let empty = u16::MAX;
        let leaf = Node { size: 1, left: empty, right: empty, up: empty };
        nodes.resize(size, leaf);

        while size > 0 {
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

            if size % 2 == 1 {
                if let Some(right) = remaining {
                    remaining = None;
                    push(end - 1, right);
                } else {
                    remaining = Some(end - 1);
                }
            }

            start = end;
            end = nodes.len();
            size = end - start;
        }

        Tree { root: (end - 1) as u16, size: (input.len() - 1) as i64, nodes }
    }

    fn position(&self, start: usize) -> u16 {
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

        offset
    }

    fn value_at(&self, start: u16) -> usize {
        let mut cur = self.root;
        let mut offset = start;

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
        let start = start as u16;
        let mut cur = start;
        let mut offset = 0;

        while cur != self.root {
            let next = self[cur].up;
            let Node { left, right, .. } = self[next];

            if right == cur {
                offset += self[left].size;
            };
            self[next].size -= 1;
            cur = next;
        }

        let parent = self[start].up;

        if parent == self.root {
            if self[self.root].left == start {
                self.root = self[self.root].right;
            } else {
                self.root = self[self.root].left;
            }
        } else {
            let grand_parent = self[parent].up;
            let next_parent =
                if self[parent].left == start { self[parent].right } else { self[parent].left };
            self[next_parent].up = grand_parent;
            if self[grand_parent].left == parent {
                self[grand_parent].left = next_parent;
            } else {
                self[grand_parent].right = next_parent;
            }
        }

        cur = self.root;
        offset = (offset as i64 + value).rem_euclid(self.size) as u16;

        loop {
            let Node { size, left, right, up } = self[cur];

            if size == 1 {
                self[parent] = Node { size: 2, left: start, right: cur, up };

                self[cur].up = parent;
                self[start].up = parent;

                if self[up].left == cur {
                    self[up].left = parent;
                } else {
                    self[up].right = parent;
                }

                break;
            }

            self[cur].size += 1;
            let size = self[left].size;
            if offset < size {
                cur = left;
            } else {
                cur = right;
                offset -= size;
            }
        }
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
    let mut tree = Tree::from(input);

    for _ in 0..rounds {
        for (i, n) in input.iter().enumerate() {
            tree.mix(i, n * key);
        }
    }

    let start = input.iter().position(|&n| n == 0).unwrap();
    let zeroth = tree.position(start);
    [1000, 2000, 3000]
        .iter()
        .map(|offset| tree.value_at((zeroth + offset) % (input.len() as u16)))
        .map(|index| input[index] * key)
        .sum()
}
