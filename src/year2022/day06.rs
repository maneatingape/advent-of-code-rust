//! # Tuning Trouble
//!
//! One solution to this problem is to use the [`windows`] method to slide over groups of the desired
//! size, then construct a [`HashSet`] from the characters. If the [`HashSet`] is the same size
//! as the window then we know that all characters are unique, as sets contain no duplicate elements.
//!
//! We'll use a faster approach that minimizes the work needed. Instead of creating a set for each
//! window, we'll maintain a count of each character. As the window advances we add the next
//! character to the count and remove the character the drops out of the window.
//!
//! [`windows`]: slice::windows
//! [`HashSet`]: std::collections::HashSet

/// Convert the input string into a `vec` of `usize`, where "a" maps to 0 and "z" to 25.
///
/// Notes:
/// * We need to [`trim`] to remove the trailing newline character
/// * Advent of Code input is always ASCII characters, so casting to an `u8` slice is acceptable.
///
/// [`trim`]: str::trim
pub fn parse(input: &str) -> Vec<usize> {
    input.trim().bytes().map(|b| (b - b'a') as usize).collect()
}

/// Find the first unique set of size 4
pub fn part1(input: &[usize]) -> usize {
    find(input, 4)
}

/// Find the first unique set of size 14
pub fn part2(input: &[usize]) -> usize {
    find(input, 14)
}

/// Efficient search algorithm.
///
/// The cardinality of the input is only 26 so a fixed size array can store the count of each
/// character. We are interested in 2 transitions:
/// * If the count for a character was 0 and is now 1, then this is the only character of this type
///   in the window and we should increment the `different` counter by 1.
/// * If the count for the character was 1 and is now 0, then the character is no longer present
///   in the window, and we should decrement the `different` counter by 1.
///
/// All other transitions have no effect on the value of `different`. Once the `different` counter
/// is the same as the window size then we return the 1-based index as our answer.
fn find(input: &[usize], marker: usize) -> usize {
    let mut letters = [0; 26];
    let mut different = 0;

    for i in 0..input.len() {
        let new = input[i];
        letters[new] += 1;
        if letters[new] == 1 {
            different += 1;
        }

        if i >= marker {
            let old = input[i - marker];
            letters[old] -= 1;
            if letters[old] == 0 {
                different -= 1;
            }
        }

        if different == marker {
            return i + 1;
        }
    }

    unreachable!()
}
