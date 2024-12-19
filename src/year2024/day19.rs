//! # Linen Layout
//!
//! Solves both parts simultaneously. Part one is the number of designs with non-zero possible
//! combinations.
//!
//! An elegant approach to check if the design starts with any towel is to first build a
//! [trie](https://en.wikipedia.org/wiki/Trie). Each node in the trie stores a `bool` indicating
//! if it's a valid towel and links to the next node for each possible color.
//!
//! There are only 5 colors. A custom [perfect hash](https://en.wikipedia.org/wiki/Perfect_hash_function)
//! function maps indices between 0 and 7 so that they fit into a fixed size array. This is faster
//! than using a `HashSet`.
//!
//! Additionally we store the Trie in a flat `vec`. This is simpler and faster than creating
//! objects on the heap using [`Box`].
type Input = (usize, usize);

pub fn parse(input: &str) -> Input {
    let (prefix, suffix) = input.split_once("\n\n").unwrap();

    // Build Trie from all towels.
    let mut trie = Vec::with_capacity(1_000);
    trie.push(Node::new());

    for towel in prefix.split(", ") {
        let mut i = 0;

        for j in towel.bytes().map(perfect_hash) {
            if trie[i].next[j] == 0 {
                // This is a new prefix, so update the index to point to it then push new node.
                trie[i].next[j] = trie.len();
                i = trie.len();
                trie.push(Node::new());
            } else {
                // Follow existing prefix.
                i = trie[i].next[j];
            }
        }

        trie[i].set_towel();
    }

    let mut part_one = 0;
    let mut part_two = 0;
    let mut ways = Vec::with_capacity(100);

    for design in suffix.lines().map(str::as_bytes) {
        let size = design.len();

        // Reset state.
        ways.clear();
        ways.resize(size + 1, 0);

        // There's 1 way to create any possible first prefix.
        ways[0] = 1;

        for start in 0..size {
            // Only consider suffixes that have a valid prefix.
            if ways[start] > 0 {
                // Walk trie from root to leaf.
                let mut i = 0;

                for end in start..size {
                    // Get next link.
                    i = trie[i].next[perfect_hash(design[end])];

                    // This is not a valid prefix, stop the search.
                    if i == 0 {
                        break;
                    }

                    // Add the number of possible ways this prefix can be reached.
                    ways[end + 1] += trie[i].towels() * ways[start];
                }
            }
        }

        // Last element is the total possible combinations.
        let total = ways[size];
        part_one += (total > 0) as usize;
        part_two += total;
    }

    (part_one, part_two)
}

pub fn part1(input: &Input) -> usize {
    input.0
}

pub fn part2(input: &Input) -> usize {
    input.1
}

/// Hashes the five possible color values white (w), blue (u), black (b), red (r), or green (g)
/// to 0, 2, 4, 5 and 1 respectively. This compresses the range to fit into an array of 6 elements.
fn perfect_hash(b: u8) -> usize {
    let n = b as usize;
    (n ^ (n >> 4)) % 8
}

/// Simple Node object that uses indices to link to other nodes.
struct Node {
    next: [usize; 6],
}

impl Node {
    fn new() -> Self {
        Node { next: [0; 6] }
    }

    // Index 3 is not used by the hash, so we cheekily repurpose for the number of towels.
    fn set_towel(&mut self) {
        self.next[3] = 1;
    }

    fn towels(&self) -> usize {
        self.next[3]
    }
}
