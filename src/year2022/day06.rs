//! # Tuning Trouble
//!
//! One solution to this problem is to use the [`windows`] method to slide over groups of the desired
//! size, then construct a [`HashSet`] from the characters. If the [`HashSet`] is the same size
//! as the window then we know that all characters are unique, as sets contain no duplicate elements.
//!
//! We'll use a faster approach that minimizes the work needed. Instead of creating a set for each
//! window, we'll maintain the last position seen of each character. As we advance character by
//! character we lookup the previous position. If this is within the packet size, then we advance
//! the start of the packet to exclude that character. Once the packet has reached the desired
//! size then we return the current index.
//!
//! [`windows`]: slice::windows
//! [`HashSet`]: std::collections::HashSet

/// Return the input directly.
pub fn parse(input: &str) -> &str {
    input
}

/// Find the first unique set of size 4
pub fn part1(input: &str) -> usize {
    find(input, 4)
}

/// Find the first unique set of size 14
pub fn part2(input: &str) -> usize {
    find(input, 14)
}

/// The cardinality of the input is only 26 so a fixed size array can store the last position
/// of each character.
fn find(input: &str, marker: usize) -> usize {
    let mut start = 0;
    let mut seen = [0; 26];

    for (i, b) in input.bytes().enumerate() {
        // Use the character as an index into the array.
        let index = (b - b'a') as usize;
        let previous = seen[index];
        // Positions are 1-based.
        seen[index] = i + 1;

        // There's a duplicate so advance the start of the window one character past it.
        start = start.max(previous);
        // We've reached the desired packet size with no duplicates so finish.
        if i + 1 - start == marker {
            return i + 1;
        }
    }

    unreachable!()
}
