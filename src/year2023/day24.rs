//! # Never Tell Me The Odds
//!
//! ## Part One
//!
//! We find the intersection for each pair of hailstones by solving a pair of linear simultaneous
//! equations in 2 unknowns:
//!
//! * `a` and `g` are the x positions of the pair of hailstones.
//! * `b` and `h` are the y positions.
//! * `d` and `j` are the x velocities.
//! * `e` and `k` are the y velocities.
//! * Let `t` and `u` be the times that the first and second hailstone respectively are at the
//!   intersection point.
//!
//! Then we can write:
//!
//! * `a + dt = g + ju` => `dt - ju = g - a`
//! * `b + et = h + ku` => `et - ku = h - b`
//!
//! In matrix form:
//!
//! ```none
//! | d  -j ||u| = | g - a |
//! | e  -k ||t|   | h - b |
//! ```
//!
//! Solve by finding the inverse of the 2x2 matrix and premultiplying both sides. The inverse is:
//!
//! ```none
//! ______1______ | -k  j |
//! d(-k) - (-j)e | -e  d |
//! ```
//!
//! Then we check that both times are non-negative and that the intersection point is inside the
//! target area.
//!
//! ## Part Two
//!
//! First we choose 3 arbitrary hailstones. Then we subtract the position and velocity of
//! the first to make the other two relative.
//!
//! The two hailstones will intercept a line leaving the origin. We can determine this line
//! by intersecting the two planes that the hailstones' velocities lie in. These planes are
//! defined by a normal vector orthogonal to the plane.
//!
//! This normal vector is the [cross product](https://en.wikipedia.org/wiki/Cross_product) of
//! any two vectors that lie in the plane, in this case the velocity and also the vector from the
//! origin to the starting location of the hailstone.
//!
//! The direction but not necessarily the magnitude of the velocity is then given by the cross
//! product of the two normals.
//!
//! Given the rock direction we can calculate the times that the two hailstones are intercepted
//! then use this to determine the original position of the rock, as long as the two times
//! are different.
use crate::util::hash::*;
use crate::util::heap::*;
use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::ops::{Add, Sub};
use std::rc::Rc;

use crate::util::iter::*;
use crate::util::math::*;
use crate::util::parse::*;

const LOW: i128 = 200_000_000_000_000;
const HIGH: i128 = 400_000_000_000_000;
const SCALE_BITS: u32 = 32;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum EventType {
    Start,
    Intersection,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct LineSegment {
    id: usize,
    m: i128,       // Scaled by SCALE_BITS.
    b: i128,       // Scaled by SCALED_BITS.
    x_start: i128, // Scaled by SCALED_BITS.
    x_end: i128,   // Scaled by SCALED_BITS.
}

impl LineSegment {
    fn y_at(&self, x: i128) -> i128 {
        ((self.m * x) >> SCALE_BITS) + self.b
    }
}

#[derive(Clone)]
struct StatusKey {
    segment: LineSegment,
    current_x: Rc<Cell<i128>>,
}

impl PartialEq for StatusKey {
    fn eq(&self, other: &Self) -> bool {
        self.segment.id == other.segment.id
    }
}

impl Eq for StatusKey {}

impl Ord for StatusKey {
    // Even though the y value of the key changes at runtime as current_x sweeps, the algorithm
    // guarantees that any two entries in the table have the same relative ordering for as long as
    // the key remains in the table.
    fn cmp(&self, other: &Self) -> Ordering {
        if self.segment.id == other.segment.id {
            return Ordering::Equal;
        }
        let x = self.current_x.get();
        let y1 = self.segment.y_at(x);
        let y2 = other.segment.y_at(x);

        // Primary sort: y positions anywhere along the range of current_x.
        let mut ord = y1.cmp(&y2);

        // Tie-breaker at intersections: compare current_x use slope of faster-growing id.
        if ord == Ordering::Equal {
            let intersect_x = find_segment_intersection(&self.segment, &other.segment).unwrap();
            if x < intersect_x {
                ord = other.segment.m.cmp(&self.segment.m);
            } else {
                ord = self.segment.m.cmp(&other.segment.m);
            }
        }

        // Fallback tie-breaker.
        if ord == Ordering::Equal {
            ord = self.segment.id.cmp(&other.segment.id);
        }

        ord
    }
}

impl PartialOrd for StatusKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Eq)]
struct SweepEvent {
    x: i128, // Scaled by SCALED_BITS.
    event: EventType,
    y: i128, // Scaled by SCALED_BITS.
    id: usize,
    other: Option<usize>, // only for Intersection
}

impl Ord for SweepEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        self.x
            .cmp(&other.x)
            .then_with(|| self.event.cmp(&other.event))
            .then_with(|| self.y.cmp(&other.y))
            .then_with(|| self.id.cmp(&other.id))
    }
}

impl PartialOrd for SweepEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for SweepEvent {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.event == other.event && self.y == other.y && self.id == other.id
    }
}

struct SweepStatus {
    tree: BTreeSet<StatusKey>,
    current_x: Rc<Cell<i128>>,
}

impl SweepStatus {
    fn new(current_x: Rc<Cell<i128>>) -> Self {
        Self { tree: BTreeSet::new(), current_x }
    }

    fn get_neighbors(&self, key: &StatusKey) -> (Option<LineSegment>, Option<LineSegment>) {
        let above = self.tree.range(key..).nth(1).map(|k| k.segment);
        let below = self.tree.range(..key).next_back().map(|k| k.segment);
        (above, below)
    }

    fn swap_segments(
        &mut self,
        seg1: LineSegment,
        seg2: LineSegment,
    ) -> (Option<LineSegment>, Option<LineSegment>) {
        let key1 = StatusKey { segment: seg1, current_x: Rc::clone(&self.current_x) };
        let key2 = StatusKey { segment: seg2, current_x: Rc::clone(&self.current_x) };
        // Tweak the clock to ensure cmp() still sees old order for both segments.
        let old_x = self.current_x.get();
        self.current_x.set(old_x - 1);

        // Sanity check before deleting the old values.
        let (above, below) = self.get_neighbors(&key1);
        assert!(above.is_some_and(|s| s.id == seg2.id) || below.is_some_and(|s| s.id == seg2.id));
        let removed1 = self.tree.remove(&key1);
        let removed2 = self.tree.remove(&key2);
        assert!(removed1 && removed2);

        // Another time tweak before re-inserting.
        self.current_x.set(old_x + 1);
        self.tree.insert(key1.clone());
        self.tree.insert(key2.clone());
        let (ab1, bl1) = self.get_neighbors(&key1);
        let (ab2, bl2) = self.get_neighbors(&key2);

        // Now the accurate time and correct neighbors.
        self.current_x.set(old_x);
        if ab1.is_some_and(|s| s.id == seg2.id) { (ab2, bl1) } else { (ab1, bl2) }
    }
}

#[derive(Clone, Copy)]
struct Vector {
    x: i128,
    y: i128,
    z: i128,
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl Vector {
    fn cross(self, other: Self) -> Self {
        let x = self.y * other.z - self.z * other.y;
        let y = self.z * other.x - self.x * other.z;
        let z = self.x * other.y - self.y * other.x;
        Self { x, y, z }
    }

    // Changes the magnitude (but not direction) of the vector.
    // Prevents numeric overflow.
    fn gcd(self) -> Self {
        let gcd = self.x.gcd(self.y).gcd(self.z);
        Self { x: self.x / gcd, y: self.y / gcd, z: self.z / gcd }
    }

    fn sum(self) -> i128 {
        self.x + self.y + self.z
    }
}

pub fn parse(input: &str) -> Vec<[i64; 6]> {
    input.iter_signed().chunk::<6>().collect()
}

pub fn part1(input: &[[i64; 6]]) -> usize {
    // Shared sweep-line clock, initialized below LOW.
    let current_x = Rc::new(Cell::new(0_i128));
    let mut segments = Vec::with_capacity(input.len());

    // Input files have around 60% of the n² intersections within bounds; but n is small enough that
    // oversizing this set does not hurt.
    let mut seen = FastSet::with_capacity(30_000);

    // The canonical Bentley-Ottman algorithm never has more than 3n events in the queue at once,
    // but that requires the ability to remove intersection events when segments are swapped to no
    // longer be adjacent in scan_line. Easier is to just allow the queue to be as large as the
    // expected output, even though that is oversized.
    let mut todo = MinHeap::with_capacity(seen.capacity());
    let mut status = SweepStatus::new(Rc::clone(&current_x));

    // For each point that intersects the bounding box, insert start and end events.
    for &[a, b, _, c, d, _] in input {
        if let Some(seg) = clip_to_bounds(segments.len(), a, b, c, d) {
            segments.push(seg);
            todo.push(
                SweepEvent {
                    x: seg.x_start,
                    event: EventType::Start,
                    y: seg.y_at(seg.x_start),
                    id: seg.id,
                    other: None,
                },
                (),
            );
            todo.push(
                SweepEvent {
                    x: seg.x_end,
                    event: EventType::End,
                    y: seg.y_at(seg.x_end),
                    id: seg.id,
                    other: None,
                },
                (),
            );
        }
    }

    // For each action in the work queue, update the scan_line order of currently-active segments,
    // and queue up any intersections revealed by adjacent segments.
    while let Some((event, ())) = todo.pop() {
        current_x.set(event.x);
        match event.event {
            EventType::Start => {
                let seg = segments[event.id];
                let key = StatusKey { segment: seg, current_x: Rc::clone(&current_x) };
                status.tree.insert(key.clone());

                let (above, below) = status.get_neighbors(&key);
                if let Some(ab) = above {
                    check_intersect(&seg, &ab, &mut todo, &mut seen, current_x.get());
                }
                if let Some(bl) = below {
                    check_intersect(&seg, &bl, &mut todo, &mut seen, current_x.get());
                }
            }

            EventType::End => {
                let seg = segments[event.id];
                let key = StatusKey { segment: seg, current_x: Rc::clone(&current_x) };
                let (above, below) = status.get_neighbors(&key);
                let removed = status.tree.remove(&key);
                assert!(removed);
                if let Some(ab) = above
                    && let Some(bl) = below
                {
                    check_intersect(&ab, &bl, &mut todo, &mut seen, current_x.get());
                }
            }

            EventType::Intersection => {
                let id1 = event.id;
                let id2 = event.other.unwrap();
                let seg1 = segments[id1];
                let seg2 = segments[id2];

                // The algorithm says the two must be neighbors, but not which one was first.
                let (above, below) = status.swap_segments(seg1, seg2);
                // It is okay if check_intersect is called more than necessary.
                if let Some(ab) = above {
                    check_intersect(&seg1, &ab, &mut todo, &mut seen, current_x.get());
                    check_intersect(&seg2, &ab, &mut todo, &mut seen, current_x.get());
                }
                if let Some(bl) = below {
                    check_intersect(&seg1, &bl, &mut todo, &mut seen, current_x.get());
                    check_intersect(&seg2, &bl, &mut todo, &mut seen, current_x.get());
                }
            }
        }
    }

    seen.len()
}

pub fn part2(input: &[[i64; 6]]) -> i128 {
    // Calculations need the range of `i128`.
    let widen = |i: usize| {
        let [x, y, z, dx, dy, dz] = input[i].map(|n| n as i128);
        (Vector { x, y, z }, Vector { x: dx, y: dy, z: dz })
    };

    // Take 3 arbitrary hailstones.
    let (p0, v0) = widen(0);
    let (p1, v1) = widen(1);
    let (p2, v2) = widen(2);

    // Subtract the positions and velocities to make them relative.
    // The first hailstone is stationary at the origin.
    let p3 = p1 - p0;
    let p4 = p2 - p0;
    let v3 = v1 - v0;
    let v4 = v2 - v0;

    // Find the normal to the plane that the second and third hailstones' velocities lie in.
    // This is the cross product of their respective position and velocity.
    // The cross product `s` of these two vectors is the same direction but not necessarily the
    // same magnitude of the desired velocity of the rock.
    // Only the direction is relevant (not the magnitude) so we can normalize the vector by the
    // GCD of its components in order to prevent numeric overflow.
    let q = v3.cross(p3).gcd();
    let r = v4.cross(p4).gcd();
    let s = q.cross(r).gcd();

    // Find the times when the second and third hailstone intercept this vector.
    // If the times are different then we can extrapolate the original position of the rock.
    let t = (p3.y * s.x - p3.x * s.y) / (v3.x * s.y - v3.y * s.x);
    let u = (p4.y * s.x - p4.x * s.y) / (v4.x * s.y - v4.y * s.x);
    assert_ne!(t, u);

    // Calculate the original position of the rock, remembering to add the first hailstone's
    // position to convert back to absolute coordinates.
    let a = (p0 + p3).sum();
    let b = (p0 + p4).sum();
    let c = (v3 - v4).sum();
    (u * a - t * b + u * t * c) / (u - t)
}

fn clip_to_bounds(id: usize, x: i64, y: i64, dx: i64, dy: i64) -> Option<LineSegment> {
    let x0 = x as i128;
    let y0 = y as i128;
    let vx = dx as i128;
    let vy = dy as i128;
    let (tx_min, tx_max) = if dx > 0 {
        (((LOW - x0) << SCALE_BITS) / vx, ((HIGH - x0) << SCALE_BITS) / vx)
    } else {
        (((HIGH - x0) << SCALE_BITS) / vx, ((LOW - x0) << SCALE_BITS) / vx)
    };

    let (ty_min, ty_max) = if dy > 0 {
        (((LOW - y0) << SCALE_BITS) / vy, ((HIGH - y0) << SCALE_BITS) / vy)
    } else {
        (((HIGH - y0) << SCALE_BITS) / vy, ((LOW - y0) << SCALE_BITS) / vy)
    };

    let t_start = tx_min.max(ty_min).max(0);
    let t_end = tx_max.min(ty_max);
    if t_start >= t_end {
        return None;
    }

    // Scale to fixed-point domain.
    let mut x_start = (x0 << SCALE_BITS) + (t_start * vx);
    let mut x_end = (x0 << SCALE_BITS) + (t_end * vx);

    let m = (vy << SCALE_BITS) / vx;
    let b = (y0 << SCALE_BITS) - (m * x0);

    if x_start > x_end {
        (x_start, x_end) = (x_end, x_start);
    }

    Some(LineSegment { id, m, b, x_start, x_end })
}

fn check_intersect(
    seg1: &LineSegment,
    seg2: &LineSegment,
    todo: &mut MinHeap<SweepEvent, ()>,
    seen: &mut FastSet<(usize, usize)>,
    now: i128,
) {
    let pair_key = if seg1.id < seg2.id { (seg1.id, seg2.id) } else { (seg2.id, seg1.id) };
    if seen.contains(&pair_key) {
        return;
    }
    if let Some(ix) = find_segment_intersection(seg1, seg2)
        && ix >= now
    {
        seen.insert(pair_key);
        todo.push(
            SweepEvent {
                x: ix,
                event: EventType::Intersection,
                y: seg1.y_at(ix),
                id: seg1.id,
                other: Some(seg2.id),
            },
            (),
        );
    }
}

fn find_segment_intersection(seg1: &LineSegment, seg2: &LineSegment) -> Option<i128> {
    if seg1.m == seg2.m {
        return None; // Parallel lines.
    }
    let num = seg2.b - seg1.b;
    let den = seg1.m - seg2.m;
    let intersect_x = (num << SCALE_BITS) / den;
    (intersect_x >= seg1.x_start
        && intersect_x <= seg1.x_end
        && intersect_x >= seg2.x_start
        && intersect_x <= seg2.x_end)
        .then_some(intersect_x)
}
