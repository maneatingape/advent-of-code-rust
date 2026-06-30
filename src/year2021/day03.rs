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
    entries: u16,    // Number of entries in the file.
    lanes: Vec<u16>, // Contains size counters, one for each digit lane.
    tree: Vec<u16>,  // Contains a balanced binary tree with 2**(width+1) nodes.
}

pub fn parse(input: &str) -> Input {
    let size = input.lines().next().unwrap().len();

    // Allocate frequency counters based on size of each input.
    let mut lanes = vec![0_u16; size];
    let mut tree = vec![0_u16; 1 << (size + 1)];
    let mut offset = 1 << size;

    // Update frequency counters and leaf nodes for each number in the input.
    for chunk in input.as_bytes().chunks(size + 1) {
        let mut binary = 0;

        for (i, b) in chunk[..size].iter().enumerate() {
            let bit = (b & 1) as u16;
            lanes[i] += bit;
            binary = (binary << 1) | bit as usize;
        }

        tree[offset + binary] = 1;
    }

    // Finish populating children counts in the tree.
    while offset > 2 {
        let half = offset / 2;

        for i in half..offset {
            tree[i] = tree[2 * i] + tree[2 * i + 1];
        }

        offset = half;
    }

    let entries = (input.len() / (size + 1)) as u16;

    Input { entries, lanes, tree }
}

pub fn part1(input: &Input) -> u32 {
    let mut gamma = 0;
    let mut epsilon = 0;

    for count in &input.lanes {
        let ones = *count;
        let zeros = input.entries - ones;

        gamma = (gamma << 1) | u32::from(ones > zeros);
        epsilon = (epsilon << 1) | u32::from(zeros > ones);
    }

    gamma * epsilon
}

pub fn part2(input: &Input) -> usize {
    let mut ogr = 2; // Oxygen generator rating.
    let mut csr = 2; // CO2 scrubber rating.
    let size = input.lanes.len();
    let tree = &input.tree;

    // Perform a binary search over the tree.
    for _ in 0..size {
        ogr = 2 * ogr + if tree[ogr + 1] >= tree[ogr] { 2 } else { 0 };
        csr =
            2 * csr + if tree[csr + 1].wrapping_sub(1) < tree[csr].wrapping_sub(1) { 2 } else { 0 };
    }

    ogr = ogr / 2 - (1 << size);
    csr = csr / 2 - (1 << size);

    ogr * csr
}
