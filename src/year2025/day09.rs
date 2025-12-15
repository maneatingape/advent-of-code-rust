//! # Movie Theater
use crate::util::iter::*;
use crate::util::parse::*;

type Tile = [u64; 2];

struct Candidate {
    x: u64,
    y: u64,
    interval: Interval,
}

/// The set { x in u64 | l <= x <= r }.
#[derive(Clone, Copy)]
struct Interval {
    l: u64,
    r: u64,
}

impl Interval {
    fn new(l: u64, r: u64) -> Self {
        debug_assert!(l <= r);

        Interval { l, r }
    }

    fn intersects(self, other: Self) -> bool {
        other.l <= self.r && self.l <= other.r
    }

    fn intersection(self, other: Self) -> Self {
        debug_assert!(self.intersects(other));

        Interval::new(self.l.max(other.l), self.r.min(other.r))
    }

    fn contains(self, x: u64) -> bool {
        self.l <= x && x <= self.r
    }
}

pub fn parse(input: &str) -> Vec<Tile> {
    input.iter_unsigned::<u64>().chunk::<2>().collect()
}

pub fn part1(tiles: &[Tile]) -> u64 {
    let mut area = 0;

    for (i, &[x1, y1]) in tiles.iter().enumerate() {
        for &[x2, y2] in tiles.iter().skip(i + 1) {
            let dx = x1.abs_diff(x2) + 1;
            let dy = y1.abs_diff(y2) + 1;
            area = area.max(dx * dy);
        }
    }

    area
}

pub fn part2(tiles: &[Tile]) -> u64 {
    let mut tiles = tiles.to_vec();

    tiles.sort_unstable_by_key(|&[x, y]| (y, x));

    let tiles = tiles;

    // Track the largest area so far during scanning:
    let mut largest_area: u64 = 0;

    // Each red tile (`x`, `y`) becomes a candidate for being a top corner of the largest area, and during the
    // scan the `interval` containing the maximum possible width is updated:
    let mut candidates: Vec<Candidate> = Vec::with_capacity(512);

    // Maintain an ordered list of descending edges, i.e. [begin_interval_0, end_interval_0, begin_interval_1, end_interval_1, ...]:
    let mut descending_edges: Vec<u64> = vec![];
    let mut intervals_from_descending_edges = vec![];

    // Invariants on the input data (defined by the puzzle) result in points arriving in pairs on the same y line:
    let mut it = tiles.into_iter();

    while let (Some([x0, y]), Some([x1, y1])) = (it.next(), it.next()) {
        debug_assert_eq!(y, y1);

        // Update the descending edges; since we are scanning from top to bottom, and within each line left to right,
        // when we, starting from outside of the region, hit a corner tile it is either:
        //
        // - The corner of two edges, one going right and one going down. In this case, the `descending_edges` won't contain
        //   the `x` coordinate, and we should "toggle" it on to denote that there is a new descending edge.
        // - The corner of two edges, one going right and one going up. The `descending_edges` will contain an `x` coordinate,
        //   that should be "toggled" off.
        //
        // Simular arguments work for when we are scanning inside the edge and we hit the corner that ends the edge; this is also
        // why corners always arrive in pairs.
        //
        // Do the update:
        for x in [x0, x1] {
            toggle_value_membership_in_ordered_list(&mut descending_edges, x);
        }

        // Every pair of descending edges in the ordered list defines a region; find the resulting intervals on this line:
        update_intervals_from_descending_edges(
            &descending_edges,
            &mut intervals_from_descending_edges,
        );

        // Check the rectangles this red tile could be a bottom tile for, with the current candidates:
        for candidate in &candidates {
            for x in [x0, x1] {
                if candidate.interval.contains(x) {
                    largest_area = largest_area
                        .max((candidate.x.abs_diff(x) + 1) * (candidate.y.abs_diff(y) + 1));
                }
            }
        }

        // Update candidates when their interval shrinks due to descending edge changes, and drop them when their interval becomes empty:
        candidates.retain_mut(|candidate| {
            if let Some(intersection_containing_x) =
                intervals_from_descending_edges.iter().find(|i| i.contains(candidate.x))
            {
                candidate.interval = intersection_containing_x.intersection(candidate.interval);

                true
            } else {
                false
            }
        });

        // Add any new candidates:
        for x in [x0, x1] {
            if let Some(&containing) =
                intervals_from_descending_edges.iter().find(|i| i.contains(x))
            {
                candidates.push(Candidate { x, y, interval: containing });
            }
        }
    }

    largest_area
}

// Adds `value` if it isn't in `ordered_list`, removes it if it is, maintaining the order.
fn toggle_value_membership_in_ordered_list(ordered_list: &mut Vec<u64>, value: u64) {
    let mut i = 0;

    while i < ordered_list.len() && ordered_list[i] < value {
        i += 1;
    }

    if i == ordered_list.len() || ordered_list[i] != value {
        ordered_list.insert(i, value);
    } else {
        ordered_list.remove(i);
    }
}

// Changes the list of descending edges, [begin_interval_0, end_interval_0, begin_interval_1, end_interval_1, ...],
// into a vector containing the intervals.
#[inline]
fn update_intervals_from_descending_edges(descending_edges: &[u64], to_update: &mut Vec<Interval>) {
    debug_assert!(descending_edges.len().is_multiple_of(2));

    to_update.clear();

    let mut it = descending_edges.iter();

    while let (Some(&l), Some(&r)) = (it.next(), it.next()) {
        to_update.push(Interval::new(l, r));
    }
}
