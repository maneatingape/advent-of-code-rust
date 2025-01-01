//! # Distress Signal
//!
//! One possible approach is to parse the input into a tree, then compare recursively node
//! by node. We're going to use a much faster and simpler approach by noting an observation about
//! the input data. If the sequence `10` is replaced by any single character greater than `9` then
//! we can compare the 2 packets *lexigraphically*. We'll replace all occurences of `10` with `A`
//! then compare packets character by character.
//!
//! The rules to compare 2 packets become:
//! * If both characters are the same then it's a draw, move onto the next character in each packet.
//! * If the first packet is `]` and the second packet anything else, then the first list is shorter
//!   so the packets are in order.
//! * Conversely if the second packet is `]` and the first packet anything else, the packets are not
//!   in order.
//! * If the first packet is an opening `[` and the second character anything else, then we're
//!   comparing a number with a list, so *push* the second character back onto the list to check
//!   again along with a closing `]` character.
//! * Do a similar push if the second character is an opening `[` and the first anything else.
//! * Finally compare the 2 characters by value. Since we've already covered the equal case, one
//!   is guaranteed to be greater or less than the other.
struct Packet<'a> {
    slice: &'a [u8],
    index: usize,
    extra: Vec<u8>,
}

impl Packet<'_> {
    fn new(str: &str) -> Packet<'_> {
        Packet { slice: str.as_bytes(), index: 0, extra: Vec::new() }
    }
}

pub fn parse(input: &str) -> Vec<&str> {
    input.lines().filter(|line| !line.is_empty()).collect()
}

/// Count adjacent pairs of packets that are in order.
pub fn part1(input: &[&str]) -> usize {
    input
        .chunks_exact(2)
        .enumerate()
        .map(|(i, chunk)| {
            let ordered = compare(chunk[0], chunk[1]);
            if ordered { i + 1 } else { 0 }
        })
        .sum()
}

/// Find the position of `[[2]]` and `[[6]]` in linear `O(n)` time.
///
/// One approach would be to insert `[[2]]` and `[[6]]` into the list, sort in `O(nlogn)` time,
/// then find the indices of the 2 values in `O(n)` time.
///
/// A much faster approach is to iterate over the list, comparing each packet first with `[[2]]`.
/// If the packets are in order, then increment the positions of *both* `[[2]]` and `[[6]]`,
/// since `[[2]]` is less than `[[6]]`.
///
/// If the packet and `[[2]]` are not in order, then also check against `[[6]]`, incrementing only
/// the second index if the 2 packets are in order.
///
/// This obtains the relative indices of `[[2]]` and `[[6]]` efficiently in fewer than `2n` comparisons.
pub fn part2(input: &[&str]) -> u32 {
    let mut first = 1;
    let mut second = 2;

    for packet in input {
        if compare(packet, "[[2]]") {
            first += 1;
            second += 1;
        } else if compare(packet, "[[6]]") {
            second += 1;
        }
    }

    first * second
}

/// Compare 2 packets using the rules listed in the module description.
///
/// It's faster to use 2 temporary `vec`s to store extra characters, rather than copy each
/// packet into a mutable [`VecDeque`]. We use the [`or_else`] method on [`Option`] to check
/// in the temporary `vec` for available characters first.
///
/// [`VecDeque`]: std::collections::VecDeque
/// [`or_else`]: Option::or_else
fn compare(left: &str, right: &str) -> bool {
    let mut left = Packet::new(left);
    let mut right = Packet::new(right);

    while let (Some(a), Some(b)) = (left.next(), right.next()) {
        match (a, b) {
            (a, b) if a == b => (),
            (b']', _) => return true,
            (_, b']') => return false,
            (b'[', b) => {
                right.extra.push(b']');
                right.extra.push(b);
            }
            (a, b'[') => {
                left.extra.push(b']');
                left.extra.push(a);
            }
            (a, b) => return a < b,
        }
    }

    unreachable!();
}

impl Iterator for Packet<'_> {
    type Item = u8;

    // Rely on the fact that all input is valid to avoid bounds checks
    fn next(&mut self) -> Option<Self::Item> {
        self.extra.pop().or_else(|| {
            let (index, slice) = (self.index, self.slice);

            // Replace occurences of "10" with "A"
            if slice[index] == b'1' && slice[index + 1] == b'0' {
                self.index += 2;
                Some(b'A')
            } else {
                self.index += 1;
                Some(slice[index])
            }
        })
    }
}
