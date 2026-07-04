//! # Binary Diagnostic
//!
//! Part one collects 12 frequency counters, one for each digit lane. Each line of input increments
//! the corresponding counters for each `1`. The number of `0` in the lane can be recovered by
//! subtraction, making it easy to construct both the gamma and epsilon rate.
//!
//! Part two constructs a binary tree, stored as an array. Using an array of 2*4096 bins is enough
//! to treat each of the 1000 unique inputs as a leaf in the second half of the array, and then
//! each node in the first half is computed as the sum of its two children from the right half. The
//! array can be built in linear time, at which point finding the oxygen generator rating and
//! CO2 scrubber rating are each a binary search in the tree.
pub struct Input {
    size: usize,       // Total number of binary digits in each number.
    entries: usize,    // Number of entries in the file.
    lanes: Vec<usize>, // Contains size counters, one for each digit lane.
    tree: Vec<usize>,  // Contains a balanced binary tree with 2**(width+1) nodes.
}

pub fn parse(input: &str) -> Input {
    let size = input.lines().next().unwrap().len();
    let offset = 1 << size;
    let entries = input.len() / (size + 1);

    // Allocate frequency counters based on size of each input.
    let mut lanes = vec![0; size];
    let mut tree = vec![0; 1 << (size + 1)];

    // Update frequency counters and leaf nodes for each number in the input.
    for chunk in input.as_bytes().chunks(size + 1) {
        let mut binary = 0;

        for (i, b) in chunk[..size].iter().enumerate() {
            let bit = usize::from(b & 1);
            lanes[i] += bit;
            binary = (binary << 1) | bit;
        }

        tree[offset + binary] = 1;
    }

    // Finish populating children counts in the tree.
    for i in (1..offset).rev() {
        tree[i] = tree[2 * i] + tree[2 * i + 1];
    }

    Input { size, entries, lanes, tree }
}

pub fn part1(input: &Input) -> u32 {
    let (gamma, epsilon) = input.lanes.iter().fold((0, 0), |(gamma, epsilon), &ones| {
        let zeros = input.entries - ones;
        ((gamma << 1) | u32::from(ones > zeros), (epsilon << 1) | u32::from(zeros > ones))
    });

    gamma * epsilon
}

pub fn part2(input: &Input) -> usize {
    let Input { size, tree, .. } = input;
    let offset = 1 << size;

    let mut ogr = 2; // Oxygen generator rating.
    let mut csr = 2; // CO2 scrubber rating.

    // Perform a binary search over the tree.
    for _ in 0..*size {
        ogr = 2 * (ogr + usize::from(tree[ogr + 1] >= tree[ogr]));
        csr = 2 * (csr + usize::from(tree[csr + 1].wrapping_sub(1) < tree[csr].wrapping_sub(1)));
    }

    (ogr / 2 - offset) * (csr / 2 - offset)
}
