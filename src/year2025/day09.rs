//! # Movie Theater
use crate::util::iter::*;
use crate::util::parse::*;

type Tile = [u32; 2];

struct Candidate {
    x: u32,
    y: u32,
    interval: Interval,
}

/// The set { x in u32 | l <= x <= r }.
#[derive(Clone, Copy)]
struct Interval {
    l: u32,
    r: u32,
}

impl Interval {
    fn new(l: u32, r: u32) -> Self {
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

    fn contains(self, x: u32) -> bool {
        self.l <= x && x <= self.r
    }
}

pub fn parse(input: &str) -> Vec<Tile> {
    let mut tiles: Vec<_> = input.iter_unsigned::<u32>().chunk::<2>().collect();
    tiles.sort_unstable_by_key(|&[x, y]| (y, x));
    tiles
}

pub fn part1(tiles: &[Tile]) -> u64 {
    let (top_left_tiles, top_right_tiles) = get_potential_left_corner_tiles(tiles.iter().copied());
    let (bottom_left_tiles, bottom_right_tiles) =
        get_potential_left_corner_tiles(tiles.iter().copied().rev());

    find_largest_from_all_corners(&top_left_tiles, &bottom_right_tiles, true)
        .max(find_largest_from_all_corners(&bottom_left_tiles, &top_right_tiles, false))
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
/// Tiles outside of this region cannot possibly be a corner of the largest rectangle. Assume, for proof by contradiction,
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
///
/// With just this selection of candidate edge points, the number of points that have to be
/// compared is already reduced compared to a naive quadratic pairing of all original points.
/// But exploiting the relationships we just proved above, we can further reduce the comparisons
/// to O(n log n) by repeatedly picking the mid-point of `top_tiles`, finding which corresponding
/// point in `bottom_tiles` forms the best rectangle, and then recursively checking just two of the
/// four combinations of the sublists remaining on either side of the pivots.
/// [This post](https://codeforces.com/blog/entry/128350) goes more into the theory.
fn get_potential_left_corner_tiles(
    sorted_tiles: impl Iterator<Item = [u32; 2]>,
) -> (Vec<[u32; 2]>, Vec<[u32; 2]>) {
    let mut left_tiles = Vec::new();
    let mut left_tiles_last_x = u32::MAX;

    let mut right_tiles = Vec::new();
    let mut right_tiles_last_x = u32::MIN;

    let mut it = sorted_tiles.peekable();

    while let Some(first_in_row) = it.next() {
        let mut last_in_row = first_in_row;

        while let Some(p) = it.next_if(|p| p[1] == first_in_row[1]) {
            last_in_row = p;
        }

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

    right_tiles.reverse();
    (left_tiles, right_tiles)
}

#[inline]
fn find_largest_from_all_corners(
    corner: &[[u32; 2]],
    opposite_corner: &[[u32; 2]],
    top_left: bool,
) -> u64 {
    // Helper struct for a work queue of remaining pairings that need to be checked
    struct Work {
        p_lo: usize,
        p_hi: usize,
        q_lo: usize,
        q_hi: usize,
    }

    fn addrange(work: &mut Vec<Work>, p_lo: usize, p_hi: usize, q_lo: usize, q_hi: usize) {
        if p_lo <= p_hi && q_lo <= q_hi {
            let job = Work { p_lo, p_hi, q_lo, q_hi };
            work.push(job);
        }
    }

    // Instead of performing an O(n^2) pairing of every point between the two sets, we can
    // divide and conquer for O(n log n) work by repeatedly dividing the set corner against
    // the partitions of opposite_corner that correspond to the best result from the halfway
    // point of corner.
    let mut largest = 0_u64;
    let start = Work { p_lo: 0, p_hi: corner.len() - 1, q_lo: 0, q_hi: opposite_corner.len() - 1 };
    let mut work = vec![start];

    while let Some(job) = work.pop() {
        // For a given point in corner, sweep the points in opposite_corner to find the
        // partition point for the best rectangle on the sweep.
        let p_mid = usize::midpoint(job.p_lo, job.p_hi);
        let p = corner[p_mid];
        let mut best_i = None;
        let mut maxsize = 0_u64;
        let mut q_lim = job.q_lo;

        for (q_i, q) in opposite_corner.iter().enumerate().take(job.q_hi + 1).skip(job.q_lo) {
            if p[0] > q[0] {
                q_lim = q_i;
            } else if (p[1] < q[1]) == top_left {
                let size = (p[0].abs_diff(q[0]) + 1) as u64 * (p[1].abs_diff(q[1]) + 1) as u64;
                if size > maxsize {
                    maxsize = size;
                    best_i = Some(q_i);
                }
            }
        }

        // The sweep determined how to partition smaller searches on the left and right halves
        if let Some(i) = best_i {
            largest = largest.max(maxsize);
            if p_mid > 0 {
                addrange(&mut work, job.p_lo, p_mid - 1, job.q_lo, i);
            }
            addrange(&mut work, p_mid + 1, job.p_hi, i, job.q_hi);
        } else {
            if p_mid > 0 && q_lim > 0 {
                addrange(&mut work, job.p_lo, p_mid - 1, job.q_lo, q_lim - 1);
            }
            addrange(&mut work, p_mid + 1, job.p_hi, q_lim, job.q_hi);
        }
    }

    largest
}

pub fn part2(tiles: &[Tile]) -> u64 {
    // Track the largest area so far during scanning:
    let mut largest_area: u64 = 0;

    // Each red tile (`x`, `y`) becomes a candidate for being a top corner of the largest area, and during the
    // scan, the `interval` containing the maximum possible width is updated:
    let mut candidates: Vec<Candidate> = Vec::with_capacity(512);

    // Maintain an ordered list of descending edges, i.e. [begin_interval_0, end_interval_0, begin_interval_1, end_interval_1, ...]:
    let mut descending_edges: Vec<u32> = vec![];
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
        // - The corner of two edges, one going right and one going up. The `descending_edges` will contain an `x` coordinate
        //   that should be "toggled" off.
        //
        // Similar arguments work for when we are scanning inside the edge and we hit the corner that ends the edge; this is also
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
                    largest_area = largest_area.max(
                        (candidate.x.abs_diff(x) + 1) as u64 * (candidate.y.abs_diff(y) + 1) as u64,
                    );
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
fn toggle_value_membership_in_ordered_list(ordered_list: &mut Vec<u32>, value: u32) {
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
fn update_intervals_from_descending_edges(descending_edges: &[u32], to_update: &mut Vec<Interval>) {
    debug_assert!(descending_edges.len().is_multiple_of(2));

    to_update.clear();

    let mut it = descending_edges.iter();

    while let (Some(&l), Some(&r)) = (it.next(), it.next()) {
        to_update.push(Interval::new(l, r));
    }
}
