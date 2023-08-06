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

/// Replace `10` with `A` in each packet and strip empty lines.
pub fn parse(input: &str) -> Vec<String> {
    input.lines().filter(|line| !line.is_empty()).map(|line| line.replace("10", "A")).collect()
}

/// Count adjacent pairs of packets that are in order.
pub fn part1(input: &[String]) -> usize {
    input
        .chunks_exact(2)
        .enumerate()
        .map(|(i, chunk)| {
            let ordered = compare(&chunk[0], &chunk[1]);
            if ordered {
                i + 1
            } else {
                0
            }
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
pub fn part2(input: &[String]) -> u32 {
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
    let mut left_iter = left.chars();
    let mut right_iter = right.chars();
    let mut left_extra: Vec<char> = Vec::new();
    let mut right_extra: Vec<char> = Vec::new();

    while let (Some(a), Some(b)) = (
        left_extra.pop().or_else(|| left_iter.next()),
        right_extra.pop().or_else(|| right_iter.next()),
    ) {
        match (a, b) {
            (a, b) if a == b => (),
            (']', _) => return true,
            (_, ']') => return false,
            ('[', b) => {
                right_extra.push(']');
                right_extra.push(b);
            }
            (a, '[') => {
                left_extra.push(']');
                left_extra.push(a);
            }
            (a, b) => return a < b,
        }
    }

    unreachable!();
}
