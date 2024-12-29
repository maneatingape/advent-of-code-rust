//! # Disk Fragmenter
//!
//! ## Part One
//!
//! Computes the checksum by simultaneously scanning forward for free blocks and
//! backwards for files. No memory is allocated which makes it very fast.
//!
//! ## Part Two
//!
//! We build 10 [min heaps](https://en.wikipedia.org/wiki/Heap_(data_structure)) in an array to
//! store the free space offsets. The index of the array implicitly stores the size of the
//! free block. The heaps are implemented as a simple reversed `vec`. Usually items are added
//! directly to the top of the heap, so this is faster than a real heap.
//!
//! When moving a file to a free block, the corresponding heap is popped and then any leftover
//! space is pushed back to the heap at a smaller index. The heap at index zero is not used
//! but makes the indexing easier.

/// [Triangular numbers](https://en.wikipedia.org/wiki/Triangular_number) offset by two.
/// Files can be a max size of 9 so we only need the first 10 values, including zero to make
/// indexing easier.
const TRIANGLE: [usize; 10] = [0, 0, 1, 3, 6, 10, 15, 21, 28, 36];

/// Remove any trailing newlines and convert to `usize`.
pub fn parse(input: &str) -> Vec<usize> {
    input.trim().bytes().map(|b| (b - b'0') as usize).collect()
}

/// Block by block checksum comparison that doesn't allocate any memory.
pub fn part1(disk: &[usize]) -> usize {
    // Start at the first free block and the last file.
    let mut left = 0;
    let mut right = disk.len() - 2 + disk.len() % 2;
    let mut needed = disk[right];
    let mut block = 0;
    let mut checksum = 0;

    while left < right {
        // When moving to the next free block, add the checksum for the file we're skipping over.
        (checksum, block) = update(checksum, block, left, disk[left]);
        let mut available = disk[left + 1];
        left += 2;

        while available > 0 {
            if needed == 0 {
                if left == right {
                    break;
                }
                right -= 2;
                needed = disk[right];
            }

            // Take as much space as possible from the current free block range.
            let size = needed.min(available);
            (checksum, block) = update(checksum, block, right, size);
            available -= size;
            needed -= size;
        }
    }

    // Account for any remaining file blocks left over.
    (checksum, _) = update(checksum, block, right, needed);
    checksum
}

pub fn part2(disk: &[usize]) -> usize {
    let mut block = 0;
    let mut checksum = 0;
    let mut free: Vec<_> = (0..10).map(|_| Vec::with_capacity(1_100)).collect();

    // Build a min-heap (leftmost free block first) where the size of each block is
    // implicit in the index of the array.
    for (index, &size) in disk.iter().enumerate() {
        if index % 2 == 1 && size > 0 {
            free[size].push(block);
        }

        block += size;
    }

    // Add sentinel value and reverse vecs so that smallest blocks are last.
    for heap in &mut free {
        heap.push(block);
        heap.reverse();
    }

    for (index, &size) in disk.iter().enumerate().rev() {
        block -= size;

        // Count any previous free blocks to decrement block offset correctly.
        if index % 2 == 1 {
            continue;
        }

        // Find the leftmost free block that can fit the file (if any).
        let mut next_block = block;
        let mut next_index = usize::MAX;

        for (i, heap) in free.iter().enumerate().skip(size) {
            let top = heap.len() - 1;
            let first = heap[top];

            if first < next_block {
                next_block = first;
                next_index = i;
            }
        }

        // We can make smaller free block from bigger blocks but not the other way around.
        // As an optimization if all blocks of the biggest size are after our position then
        // we can ignore them.
        if !free.is_empty() {
            let biggest = free.len() - 1;
            let top = free[biggest].len() - 1;

            if free[biggest][top] > block {
                free.pop();
            }
        }

        // Update the checksum with the file's location (possibly unchanged).
        let id = index / 2;
        let extra = next_block * size + TRIANGLE[size];
        checksum += id * extra;

        // If we used a free block, remove then add back any leftover space.
        if next_index != usize::MAX {
            free[next_index].pop();

            // Insert the new smaller block into the correct location.
            // Most frequently this is directly at the end of the vector so even though this
            // is technically `O(n)`, in practice it's faster than a real heap.
            let to = next_index - size;
            if to > 0 {
                let mut i = free[to].len();
                let value = next_block + size;

                while free[to][i - 1] < value {
                    i -= 1;
                }

                free[to].insert(i, value);
            }
        }
    }

    checksum
}

/// Convenience function to update checksum based on file location and size.
#[inline]
fn update(checksum: usize, block: usize, index: usize, size: usize) -> (usize, usize) {
    let id = index / 2;
    let extra = block * size + TRIANGLE[size];
    (checksum + id * extra, block + size)
}
