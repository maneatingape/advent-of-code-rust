use crate::util::parse::*;
use std::collections::VecDeque;

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

impl Tree {
    fn from(input: &[i64]) -> Tree {
        let len = input.len();
        let mut nodes: Vec<Node> = Vec::with_capacity(len * 2);
        let mut todo: VecDeque<u16> = VecDeque::with_capacity(len);
        let mut next: VecDeque<u16> = VecDeque::with_capacity(len);

        for i in 0..len {
            nodes.push(Node {
                size: 1,
                left: u16::MAX,
                right: u16::MAX,
                up: u16::MAX,
            });
            todo.push_back(i as u16);
        }

        let root = Self::next_layer(&mut nodes, &mut todo, &mut next);
        Tree { root, size: (len - 1) as i64, nodes }
    }

    fn next_layer(nodes: &mut Vec<Node>, todo: &mut VecDeque<u16>, next: &mut VecDeque<u16>) -> u16 {
        let root = todo.len() == 2;

        while todo.len() > 1 {
            let left = todo.pop_front().unwrap();
            let right = todo.pop_front().unwrap();
            let size = nodes[left as usize].size + nodes[right as usize].size;

            let index = nodes.len() as u16;
            nodes[left as usize].up = index;
            nodes[right as usize].up = index;

            nodes.push(Node {
                size,
                up: u16::MAX,
                left,
                right,
            });
            next.push_back(index);
        }

        if root {
            (nodes.len() - 1) as u16
        } else {
            next.extend(todo.drain(..));
            Self::next_layer(nodes, next, todo)
        }
    }

    fn position(&self, start: usize) -> u16 {
        let mut cur = start as u16;
        let mut offset = 0;

        while cur != self.root {
            let next = self.nodes[cur as usize].up;
            let Node { left, right, .. } = self.nodes[next as usize];
            if right == cur {
                offset += self.nodes[left as usize].size
            };
            cur = next;
        }

        offset
    }

    fn value_at(&self, start: u16) -> usize {
        let mut cur = self.root;
        let mut offset = start;

        loop {
            let Node { size, left, right, .. } = self.nodes[cur as usize];

            if size == 1 {
                break cur as usize;
            }

            let size = self.nodes[left as usize].size;
            if offset < size {
                cur = left;
            } else {
                cur = right;
                offset -= size;
            }
        }
    }

    fn mix(&mut self, start: usize, value: i64) {
        let mut cur = start as u16;
        let mut offset = 0;

        while cur != self.root {
            let next = self.nodes[cur as usize].up;
            let Node { left, right, .. } = self.nodes[next as usize];

            if right == cur {
                offset += self.nodes[left as usize].size
            };
            self.nodes[next as usize].size -= 1;
            cur = next;
        }

        let parent = self.nodes[start].up;

        if parent == self.root {
            if self.nodes[self.root as usize].left == start as u16 {
                self.root = self.nodes[self.root as usize].right;
            } else {
                self.root = self.nodes[self.root as usize].left;
            }
        } else {
            let grand_parent = self.nodes[parent as usize].up;
            let next_parent = if self.nodes[parent as usize].left == start as u16 {
                self.nodes[parent as usize].right
            } else {
                self.nodes[parent as usize].left
            };
            self.nodes[next_parent as usize].up = grand_parent;
            if self.nodes[grand_parent as usize].left == parent {
                self.nodes[grand_parent as usize].left = next_parent;
            } else {
                self.nodes[grand_parent as usize].right = next_parent;
            }
        }

        cur = self.root;
        offset = (offset as i64 + value).rem_euclid(self.size) as u16;

        loop {
            let Node { size, left, right, up, .. } = self.nodes[cur as usize];

            if size == 1 {
                self.nodes[parent as usize] = Node {
                    size: 2,
                    left: start as u16,
                    right: cur,
                    up,
                };

                self.nodes[cur as usize].up = parent;
                self.nodes[start].up = parent;

                if self.nodes[up as usize].left == cur {
                    self.nodes[up as usize].left = parent;
                } else {
                    self.nodes[up as usize].right = parent;
                }

                break;
            }

            self.nodes[cur as usize].size += 1;
            let size = self.nodes[left as usize].size;
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
    input.to_signed_iter().collect()
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
