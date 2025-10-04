//! # Chronal Coordinates
//!
//! Both parts can be solved with a [BFS](https://en.wikipedia.org/wiki/Breadth-first_search)
//! approach. The bounding box of the coordinates is roughly 300 wide by 300 high so the total
//! complexity would be approximately `O(90,000)`.
//!
//! A much faster approach for both parts is a
//! [sweep line algorithm](https://en.wikipedia.org/wiki/Sweep_line_algorithm). We sweep from
//! top to bottom (minimum y coordinate to maximum y coordinate) computing the area a slice at a
//! time. There are 50 coordinates so the complexity of this approach is much lower at
//! approximately `O(300 * 50) = O(15000)`.
use crate::util::iter::*;
use crate::util::parse::*;
use crate::util::point::*;

pub struct Input {
    min_y: i32,
    max_y: i32,
    points: Vec<Point>,
}

/// Parse points while keeping track of the min and max y coordinates.
pub fn parse(input: &str) -> Input {
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    let points: Vec<_> = input
        .iter_signed()
        .chunk::<2>()
        .map(|[x, y]| {
            min_y = min_y.min(y);
            max_y = max_y.max(y);
            Point::new(x, y)
        })
        .collect();

    Input { min_y, max_y, points }
}

/// Sweep line approach computing the area of each *finite* coordinate. A coordinate has infinite
/// area if any point on the edge of the bounding box formed by the minimum and maximum x and y
/// coordinates is closest to that coordinate.
///
/// We sort the coordinates in ascending x value then for each row, compare the next coordinate
/// against the head of a stack. This quickly eliminates coordinates that are further away at all
/// points. Interestingly this approach is very similar to the previous [`Day 5`].
///
/// [`Day 5`]: crate::year2018::day05
pub fn part1(input: &Input) -> i32 {
    let mut points = input.points.clone();
    let mut area = vec![0; points.len()];
    let mut finite = vec![true; points.len()];
    let mut candidates: Vec<(usize, i32, i32)> = Vec::new();

    // Special value for coordinates that are equidistant from nearest neighbour.
    let marker = usize::MAX;

    // Sorts points left to right so that ranges can be merged.
    points.sort_unstable_by_key(|p| p.x);

    // Sweep top to bottom.
    for row in input.min_y..=input.max_y {
        // Left to right.
        for (j, &p) in points.iter().enumerate() {
            // Manhattan distance is the absolute difference in y coordinates since the x
            // coordinate is already identical.
            let m1 = (p.y - row).abs();
            let x1 = p.x;

            loop {
                if let Some((i, m0, x0)) = candidates.pop() {
                    // Compare against the head of the stack.
                    let delta = m1 - m0;
                    let width = x1 - x0;

                    if delta < -width {
                        // Left coordinate is further away at every points.
                        // Discard and pop next left coordinate from the stack.
                        //
                        //    rrrrrrrrrrrrrrrr     <-- Considering only this row
                        //    ....R...........
                        //    ................
                        //    ................
                        //    ..L.............
                        continue;
                    } else if delta == -width {
                        // Left coordinate is equal from its center leftwards
                        // Replace with special marker value.
                        //
                        //    ...rrrrrrrrrrrrr
                        //    ....R...........
                        //    ................
                        //    ..L.............
                        candidates.push((marker, m0, x0));
                        candidates.push((j, m1, x1));
                    } else if delta == width {
                        // Right coordinate is equal from its center rightwards.
                        // Replace with special marker value.
                        //
                        //    llll............
                        //    ..L.............
                        //    ................
                        //    ....R...........
                        candidates.push((i, m0, x0));
                        candidates.push((marker, m1, x1));
                    } else if delta > width {
                        // Right coordinate is further away at every point.
                        // Discard then check next right coordinate from points.
                        //
                        //    llllllllllllllll
                        //    ..L.............
                        //    ................
                        //    ................
                        //    ....R...........
                        candidates.push((i, m0, x0));
                    } else {
                        // Coordinates split the distance, some points closer to left and others
                        // closer to right. Add both to candidates.
                        //
                        //    lllll.rrrrrrrrrr
                        //    .........R......
                        //    ..L.............
                        //    ................
                        //    ................
                        candidates.push((i, m0, x0));
                        candidates.push((j, m1, x1));
                    }
                } else {
                    // Nothing on stack to compare with, push coordinate.
                    candidates.push((j, m1, x1));
                }

                break;
            }
        }

        // Any coordinates that are closest to the bounding box edges are infinite.
        let left = candidates[0].0;
        if left != marker {
            finite[left] = false;
        }

        let right = candidates[candidates.len() - 1].0;
        if right != marker {
            finite[right] = false;
        }

        // Only consider finite coordinates.
        for window in candidates.windows(3) {
            let (_, m0, x0) = window[0];
            let (i, m1, x1) = window[1];
            let (_, m2, x2) = window[2];

            // Skip coordinates where all points are equally distant from their neighbor.
            if i != marker {
                if row == input.min_y || row == input.max_y {
                    // All coordinates the are closest to the top or bottom row are infinite.
                    finite[i] = false;
                } else {
                    // Count points closest to the left, to the right and the coordinate itself.
                    let left = (x1 - x0 + m0 - m1 - 1) / 2;
                    let right = (x2 - x1 + m2 - m1 - 1) / 2;
                    area[i] += left + 1 + right;
                }
            }
        }

        candidates.clear();
    }

    // Find largest area closest to finite coordinate.
    (0..points.len()).filter_map(|i| finite[i].then_some(area[i])).max().unwrap()
}

pub fn part2(input: &Input) -> i32 {
    part2_testable(input, 10_000)
}

/// Sweep from top to bottom to find the size of the roughly circular area that is less than
/// a specified maximum distance from all other points.
///
/// Finding the center of this circle to act as a starting point is an interesting sub-problem.
/// The two dimensional [geometric median](https://en.wikipedia.org/wiki/Geometric_median) that
/// minimizes the Euclidean distance to all other points has no general closed form formula.
/// The [centroid](https://en.wikipedia.org/wiki/Centroid) is close but not exact as it minimizes
/// the distance *squared*.
///
/// However the Manhattan distance is independent for each axis, so we can instead solve for the
/// one dimensional case. This is the [median](https://en.wikipedia.org/wiki/Median) of each axis.
/// Intuitively this makes sense, as the median has the same number of points on either side,
/// so moving either direction, the increase from half the points is cancelled out by the decrease
/// of the other half of the points.
///
/// The algorithm is:
/// * Find center
/// * Go upwards from center until top edge of circle reached.
/// * For each row of circle, find left and right extents
/// * Add area of row to total, then advance to row below.
pub fn part2_testable(input: &Input, max_distance: i32) -> i32 {
    // Sort points in ascending order in order to find median.
    let mut xs: Vec<_> = input.points.iter().map(|p| p.x).collect();
    xs.sort_unstable();

    let mut ys: Vec<_> = input.points.iter().map(|p| p.y).collect();
    ys.sort_unstable();

    // Find coordinate closest to median point.
    let x = xs[xs.len() / 2];
    let mut y = ys[ys.len() / 2];

    // Calculate minimum distance.
    let median = Point::new(x, y);
    let mut y_distance: i32 = input.points.iter().map(|o| o.manhattan(median)).sum();

    // Find top of region
    while y_distance + prev(&ys, y) < max_distance {
        y_distance += prev(&ys, y);
        y -= 1;
    }

    let mut left = x;
    let mut left_dist = y_distance;
    let mut right = x;
    let mut right_dist = y_distance;
    let mut area = 0;

    // Sweep top to bottom.
    while y_distance < max_distance {
        // Expand moving left edge to the left
        while left_dist < max_distance {
            left_dist += prev(&xs, left);
            left -= 1;
        }
        // Contract moving left edge to the right
        while left_dist >= max_distance {
            left_dist += next(&xs, left);
            left += 1;
        }
        // Expand moving right edge to the right
        while right_dist < max_distance {
            right_dist += next(&xs, right);
            right += 1;
        }
        // Contract moving right edge to the left
        while right_dist >= max_distance {
            right_dist += prev(&xs, right);
            right -= 1;
        }

        // Move downwards one row.
        let next = next(&ys, y);
        y_distance += next;
        left_dist += next;
        right_dist += next;

        y += 1;
        area += right - left + 1;
    }

    area
}

/// Calculate the change in distance moving left or up.
fn prev(slice: &[i32], n: i32) -> i32 {
    slice.iter().map(|&s| if s >= n { 1 } else { -1 }).sum()
}

/// Calculate the change in distance moving down or right.
fn next(slice: &[i32], n: i32) -> i32 {
    slice.iter().map(|&s| if s <= n { 1 } else { -1 }).sum()
}
