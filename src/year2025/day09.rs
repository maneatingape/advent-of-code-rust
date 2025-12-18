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
    let mut tiles: Vec<_> = input.iter_unsigned::<u64>().chunk::<2>().collect();
    tiles.sort_unstable_by_key(|&[x, y]| (y, x));
    tiles
}

pub fn part1(tiles: &[Tile]) -> u64 {
    // let mut tiles = tiles.to_vec();

    // tiles.sort_by_key(|&[x, y]| (x, y));

    let (top_left_tiles, top_right_tiles) = get_potential_left_corner_tiles(tiles.iter().copied());
    let (bottom_left_tiles, bottom_right_tiles) =
        get_potential_left_corner_tiles(tiles.iter().copied().rev());

    find_largest_from_all_corners(&top_left_tiles, &bottom_right_tiles)
        .max(find_largest_from_all_corners(&bottom_left_tiles, &top_right_tiles))
}

/// This function filters `sorted_tiles` into two lists, one containing all tiles that could be the top left
/// corner of the largest rectangle (assuming the largest rectangle has a top left corner), and the second
/// containing all tiles that could be the top right corner.
///
/// It assumes `sorted_tiles` is sorted in ascending "y" values, or, to get the top right and bottom right corners,
/// that `sorted_tiles` is sorted in descending "y" order.
///
/// It works (for the top left corners, for illustration) by only returning tiles (from the set of all tiles, "T") within
/// the region:
///
///   R = { (x, y) ∈ ℝ² : ∀ (tx, ty) ∈ T, tx ≤ x ⇒ ty ≥ y }
///
/// Tiles outside of this region can not possibly be a corner of the largest rectangle. Assume, for proof by contradiction,
/// that the top left corner of the largest rectangle is in the complement of the set "R":
///
///   R' = { (x, y) ∈ ℝ² : ¬ (∀ (tx, ty) ∈ T, tx ≤ x ⇒ ty ≥ y) }
///      = { (x, y) ∈ ℝ² : ∃ (tx, ty) ∈ T, tx ≤ x ∧ ty < y }
///
/// That is, for the corner (x, y), there exists another tile (tx, ty) that is to the left and above the corner tile, which
/// means the tile isn't the corner of the largest possible rectangle, completing the proof by contradiction.
///
/// The `top_tiles` and `bottom_tiles` are the corner points of this region `R`, built up by scanning through tiles
/// in either left to right or right to left order.
// fn get_potential_left_corner_tiles(
//     sorted_tiles: impl Iterator<Item = [u64; 2]>,
// ) -> (Vec<[u64; 2]>, Vec<[u64; 2]>) {
//     let mut top_tiles = Vec::new();
//     let mut top_tiles_last_y = u64::MAX;

//     let mut bottom_tiles = Vec::new();
//     let mut bottom_tiles_last_y = u64::MIN;

//     let mut it = sorted_tiles.peekable();

//     while let Some(first_in_column) = it.next() {
//         let mut last_in_column = first_in_column;

//         while let Some(p) = it.next_if(|p| p[0] == first_in_column[0]) {
//             last_in_column = p;
//         }

//         let (x, top_y, bottom_y) = (
//             first_in_column[0],
//             first_in_column[1].min(last_in_column[1]),
//             first_in_column[1].max(last_in_column[1]),
//         );

//         if top_y < top_tiles_last_y {
//             top_tiles.push([x, top_y]);
//             top_tiles_last_y = top_y;
//         }

//         if bottom_y > bottom_tiles_last_y {
//             bottom_tiles.push([x, bottom_y]);
//             bottom_tiles_last_y = bottom_y;
//         }
//     }

//     (top_tiles, bottom_tiles)
// }

fn get_potential_left_corner_tiles(
    sorted_tiles: impl Iterator<Item = [u64; 2]>,
) -> (Vec<[u64; 2]>, Vec<[u64; 2]>) {
    let mut left_tiles = Vec::new();
    let mut left_tiles_last_x = u64::MAX;

    let mut right_tiles = Vec::new();
    let mut right_tiles_last_x = u64::MIN;

    let mut it = sorted_tiles.peekable();

    while let Some(first_in_row) = it.next() {
        let mut last_in_row = first_in_row;

        while let Some(p) = it.next_if(|p| p[1] == first_in_row[1]) {
            last_in_row = p;
        }

        // let (x, top_y, bottom_y) = (
        //     first_in_column[0],
        //     first_in_column[1].min(last_in_column[1]),
        //     first_in_column[1].max(last_in_column[1]),
        // );

        let (y, left_x, right_x) = (
            first_in_row[1],
            first_in_row[0].min(last_in_row[0]),
            first_in_row[0].max(last_in_row[0]),
        );

        if left_x < left_tiles_last_x {
            left_tiles.push([left_x, y]);
            left_tiles_last_x = left_x;
        }

        if right_x > right_tiles_last_x {
            right_tiles.push([right_x, y]);
            right_tiles_last_x = right_x;
        }
    }

    (left_tiles, right_tiles)
}

#[inline]
fn find_largest_from_all_corners(corner: &[[u64; 2]], opposite_corner: &[[u64; 2]]) -> u64 {
    let mut largest = 0_u64;

    for &p in corner {
        for &q in opposite_corner {
            largest = largest.max((p[0].abs_diff(q[0]) + 1) * (p[1].abs_diff(q[1]) + 1));
        }
    }

    largest
}

pub fn part2(tiles: &[Tile]) -> u64 {
    // Track the largest area so far during scanning:
    let mut largest_area: u64 = 0;

    // Each red tile (`x`, `y`) becomes a candidate for being a top corner of the largest area, and during the
    // scan the `interval` containing the maximum possible width is updated:
    let mut candidates: Vec<Candidate> = Vec::with_capacity(512);

    // Maintain an ordered list of descending edges, i.e. [begin_interval_0, end_interval_0, begin_interval_1, end_interval_1, ...]:
    let mut descending_edges: Vec<u64> = vec![];
    let mut intervals_from_descending_edges = vec![];

    // Invariants on the input data (defined by the puzzle) result in points arriving in pairs on the same y line:
    let mut it = tiles.iter();

    while let (Some(&[x0, y]), Some(&[x1, y1])) = (it.next(), it.next()) {
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
    match ordered_list.binary_search(&value) {
        Ok(i) => {
            ordered_list.remove(i);
        }
        Err(i) => {
            ordered_list.insert(i, value);
        }
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
