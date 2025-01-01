//! # Chocolate Charts
//!
//! This solution is heavily inspired by [Askalski's](https://www.reddit.com/user/askalski/)
//! excellent post [Breaking the 1 billion recipes per second barrier](https://www.reddit.com/r/adventofcode/comments/a6wpwa/2018_day_14_breaking_the_1_billion_recipes_per/)
//!
//! The key insight is that after 23 recipes the elves converge into using the *same subset* of
//! recipes. This subset can be stored compactly in about 20% of the space and read sequentially
//! to allow efficient vector processing.
//!
//! Tricks used to speed things up:
//! * Separate writer and reader threads to generate recipes and check them in parallel.
//! * Vector processing of recipes using techniques similar to SIMD.
use crate::util::parse::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;

type Input = (String, usize);

/// Pre-calculate the first 23 recipes.
const PREFIX: [u8; 23] = [3, 7, 1, 0, 1, 0, 1, 2, 4, 5, 1, 5, 8, 9, 1, 6, 7, 7, 9, 2, 5, 1, 0];

pub fn parse(input: &str) -> Input {
    // Send batches of recipes from the writer to the reader for checking.
    let (tx, rx) = channel();
    // Thread safe flag to let writer know when to stop.
    let done = AtomicBool::new(false);
    // Store recipes in fixed size vec prefilled with ones. Part two result is around 20 million
    // so size should be sufficient for most inputs.
    let mut recipes = vec![1; 25_000_000];

    thread::scope(|scope| {
        // Start writer thread to produce new recipes.
        scope.spawn(|| writer(tx, &done, recipes.as_mut_slice()));
        // Reader thread checks recipes for the answers, returning when both parts are found.
        scope.spawn(|| reader(rx, &done, input)).join().unwrap()
    })
}

pub fn part1(input: &Input) -> &str {
    &input.0
}

pub fn part2(input: &Input) -> usize {
    input.1
}

/// Receives batches of recipes from the writer thread, then scans them byte by byte searching
/// for the part two pattern. For simplicity the pattern is always assumed to by six digits.
fn reader(rx: Receiver<&[u8]>, done: &AtomicBool, input: &str) -> (String, usize) {
    let part_one_target = input.unsigned::<usize>() + 10;
    let part_two_target = u32::from_str_radix(input.trim(), 16).unwrap();

    let mut part_one_result = None;
    let mut part_two_result = None;

    let mut history = Vec::new();
    let mut total = 0;
    let mut pattern = 0;

    for slice in rx {
        history.push(slice);
        total += slice.len();

        // The recipes are broken up into batches. Even though these batches originally come
        // from the same contiguous slice, this thread has no way to know that or reassemble
        // the original. The result could potentially be split over two or more slices.
        if part_one_result.is_none() && total >= part_one_target {
            let mut index = 0;
            let mut offset = part_one_target - 10;
            let mut result = String::new();

            for _ in 0..10 {
                // If we go past the end of a slice then check the next one.
                while offset >= history[index].len() {
                    offset -= history[index].len();
                    index += 1;
                }

                // Push each digit into a string as there could be leading zeroes.
                let digit = history[index][offset];
                result.push((digit + b'0') as char);
                offset += 1;
            }

            part_one_result = Some(result);
        }

        // Simple brute force pattern matching. Slices are received in order so the pattern will
        // handle cases when the target is split between two slices.
        if part_two_result.is_none() {
            for (i, n) in slice.iter().copied().enumerate() {
                pattern = ((pattern << 4) | (n as u32)) & 0xffffff;

                if pattern == part_two_target {
                    part_two_result = Some(total - slice.len() + i - 5);
                    break;
                }
            }
        }

        // Signal the writer thread to finish once both results are found.
        if part_one_result.is_some() && part_two_result.is_some() {
            done.store(true, Ordering::Relaxed);
            break;
        }
    }

    (part_one_result.unwrap(), part_two_result.unwrap())
}

/// Generates recipes then sends them to the reader thread for checking in batches.
/// Processing is broken into alternating "cold" and "hot" loops. An outer enclosing loop checks
/// periodically for the done signal from the reader thread.
///
/// The "cold" loop processes recipes serially one by one but can handle input corner cases.
/// It's used when either:
/// * One or both elves are within the first 23 recipes.
/// * One or both elves are within the last 16 recipes.
///
/// The "hot" loop processes recipes efficiently in chunks of 16. The vast majority of recipes
/// are calculated in this loop. As much as possible is parallelized using techniques similar to
/// SIMD but using regular instructions instead of SIMD instrinsics or Rust's portable SIMD API.
///
/// Interestingly on an Apple M2 Max this "poor man's SIMD" has the same performance as using
/// the portable SIMD API. This is probably due to the fact that the serial loops that write new
/// recipes take the majority of the time.
fn writer<'a>(tx: Sender<&'a [u8]>, done: &AtomicBool, mut recipes: &'a mut [u8]) {
    // The first 23 recipes have already been generated
    // so the elves start at position 0 and 8 respectively.
    let mut elf1 = 0;
    let mut index1 = 0;

    let mut elf2 = 8;
    let mut index2 = 0;

    let mut base = 0;
    let mut size = 23;
    let mut needed = 23;

    // Store the smaller subset of recipes used by the elves.
    let mut write = 0;
    let mut snack: Vec<u8> = vec![0; 5_000_000];

    while !done.load(Ordering::Relaxed) {
        // Cold loop to handle start and end transitions.
        while elf1 < 23 || elf2 < 23 || write - index1.max(index2) <= 16 {
            // After the first 23 recipes both elves converge on the same set of ingredients.
            let recipe1 = if elf1 < 23 {
                PREFIX[elf1]
            } else {
                index1 += 1;
                snack[index1 - 1]
            };

            let recipe2 = if elf2 < 23 {
                PREFIX[elf2]
            } else {
                index2 += 1;
                snack[index2 - 1]
            };

            // Add next recipe.
            let next = recipe1 + recipe2;
            if next < 10 {
                recipes[size - base] = next;
                size += 1;
            } else {
                recipes[size - base + 1] = next - 10;
                size += 2;
            }

            if needed < size {
                let digit = recipes[needed - base];
                needed += 1 + digit as usize;

                snack[write] = digit;
                write += 1;
            }

            // Wrap around to start if necessary.
            elf1 += 1 + recipe1 as usize;
            if elf1 >= size {
                elf1 -= size;
                index1 = 0;
            }

            elf2 += 1 + recipe2 as usize;
            if elf2 >= size {
                elf2 -= size;
                index2 = 0;
            }
        }

        // Hot loop to handle the majority of recipes in the middle. Process at most 10,000 recipes
        // at a time in order to produce batches between 160,000 and 320,000 bytes in size.
        // This size is roughly tuned in order to maximize reader thread throughput.
        let batch_size = 10_000.min((write - index1.max(index2) - 1) / 16);

        for _ in 0..batch_size {
            // Snacks can be processed sequentially.
            let first = from_be_bytes(&snack, index1);
            let second = from_be_bytes(&snack, index2);
            let third = from_be_bytes(&snack, index1 + 8);
            let fourth = from_be_bytes(&snack, index2 + 8);

            // Each elf will skip forward between 16 and 32 snacks.
            elf1 += 16 + lsb(prefix_sum(first)) + lsb(prefix_sum(third));
            elf2 += 16 + lsb(prefix_sum(second)) + lsb(prefix_sum(fourth));
            index1 += 16;
            index2 += 16;

            // Process the digits in parallel using techniques similar to SIMD.
            let (digits1, indices1, extra1) = unpack(first, second);
            let (digits2, indices2, extra2) = unpack(third, fourth);

            // Scatter each digit into the correct location, leaving "holes" where ones should go.
            // This is handled correctly by prefilling `recipes`` with ones when initializing.
            for shift in (0..64).step_by(8) {
                let digit = lsb(digits1 >> shift);
                let index = lsb(indices1 >> shift);
                recipes[size - base + index] = digit as u8;

                let digit = lsb(digits2 >> shift);
                let index = lsb(indices2 >> shift);
                recipes[size - base + index + extra1] = digit as u8;
            }

            size += extra1 + extra2;

            // Write the recipes that will actually be used in subsequent loops to a smaller
            // contiguous vec.
            while needed < size {
                let digit = recipes[needed - base];
                needed += 1 + digit as usize;

                snack[write] = digit;
                write += 1;
            }
        }

        // Split the mutable `recipes` slice into two parts. This allows the reader thread to
        // access the head in parallel while the reader thread continues to write to the tail,
        // ensuring unique ownership of each part of memory to prevent any concurrency issues.
        let (head, tail) = recipes.split_at_mut(size - base);
        let _unused = tx.send(head);
        recipes = tail;
        base = size;
    }

    // Drop the sender to make the receiver hang up.
    drop(tx);
}

/// Convert 8 bytes in [big endian order](https://en.wikipedia.org/wiki/Endianness) into a `usize`.
#[inline]
fn from_be_bytes(slice: &[u8], index: usize) -> usize {
    usize::from_be_bytes(slice[index..index + 8].try_into().unwrap())
}

/// Convenience function that returns least significant byte.
#[inline]
fn lsb(u: usize) -> usize {
    u & 0xff
}

/// Compute the prefix sum of each byte within a `usize`. Let `a..h` denote the bytes from most
/// significant to least significant and `Σx..y` denote the sum from `x` to `y` inclusive.
///
/// ```none
///     s               |   a   |   b   |   c   |   d   |   e   |   f   |   g   |   h   |
///     s += (s >> 8)   |   a   | Σa..b | Σb..c | Σc..d | Σd..e | Σe..f | Σf..g | Σg..h |
///     s += (s >> 16)  |   a   | Σa..b | Σa..c | Σa..d | Σb..e | Σc..f | Σd..g | Σe..h |
///     s += (s >> 32)  |   a   | Σa..b | Σa..c | Σa..d | Σa..e | Σa..f | Σa..g | Σa..h |
/// ```
#[inline]
fn prefix_sum(u: usize) -> usize {
    let mut s = u;
    s += s >> 8;
    s += s >> 16;
    s += s >> 32;
    s
}

/// Takes two groups of 8 digits each packed into a `usize` as input, then returns the output
/// digits and their respective locations. Ones from sums greater than ten are implicit and not
/// included since recipes has already been pre-filled with ones.
#[inline]
fn unpack(first: usize, second: usize) -> (usize, usize, usize) {
    const ONES: usize = 0x0101010101010101;
    const SIXES: usize = 0x0606060606060606;
    const INDICES: usize = 0x0001020304050607;

    // Example values, showing each byte in a columm:
    //
    // first      | 04 | 01 | 09 | 08 | 00 | 03 | 05 | 07 |
    // second     | 03 | 00 | 02 | 04 | 09 | 06 | 05 | 01 |
    // sum        | 07 | 01 | 0b | 0c | 09 | 09 | 0a | 08 |
    let sum = first + second;

    // Add 6 to each byte so that sums greater than or equal to ten become greater than or equal
    // to 16, setting the first bit in the high nibble of each byte.
    //
    // sum        | 07 | 01 | 0b | 0c | 09 | 09 | 0a | 08 |
    // SIXES      | 06 | 06 | 06 | 06 | 06 | 06 | 06 | 06 |
    // total      | 0d | 07 | 11 | 12 | 0f | 0f | 10 | 0e |
    // tens       | 00 | 00 | 01 | 01 | 00 | 00 | 01 | 00 |
    let tens = ((sum + SIXES) >> 4) & ONES;

    // Multiply by 10 to "spread" a 10 into each byte that has a total greater than 10.
    //
    // tens       | 00 | 00 | 01 | 01 | 00 | 00 | 01 | 00 |
    // tens * 10  | 00 | 00 | 0a | 0a | 00 | 00 | 0a | 00 |
    // digits     | 07 | 01 | 01 | 02 | 09 | 09 | 00 | 08 |
    let digits = sum - 10 * tens;

    // Columns greater than 10 will takes 2 bytes when written to recipes. Each index is
    // offset by the number of 10s before it. Adding the normal increase indices gives the
    // final location of each byte.
    //
    // tens       | 00 | 00 | 01 | 01 | 00 | 00 | 01 | 00 |
    // prefix sum | 00 | 00 | 01 | 02 | 02 | 02 | 03 | 03 |
    // INDICES    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 |
    // indices    | 00 | 02 | 03 | 05 | 06 | 07 | 09 | 0a |
    let indices = prefix_sum(tens) + INDICES;

    // The total number of bytes that need to be written is one plus the last index.
    let extra = 1 + lsb(indices);

    (digits, indices, extra)
}
