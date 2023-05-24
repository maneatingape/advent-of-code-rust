use crate::util::iter::*;
use crate::util::parse::*;

pub struct RebootStep {
    on: bool,
    cube: Cube,
}

impl RebootStep {
    fn from((command, points): (&str, [i32; 6])) -> RebootStep {
        let on = command == "on";
        let cube = Cube::from(points);
        RebootStep { on, cube }
    }
}

#[derive(Clone, Copy)]
pub struct Cube {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
    z1: i32,
    z2: i32,
}

impl Cube {
    fn from(points: [i32; 6]) -> Cube {
        let [a, b, c, d, e, f] = points;
        let x1 = a.min(b);
        let x2 = a.max(b);
        let y1 = c.min(d);
        let y2 = c.max(d);
        let z1 = e.min(f);
        let z2 = e.max(f);
        Cube { x1, x2, y1, y2, z1, z2 }
    }

    fn intersect(&self, other: &Cube) -> Option<Cube> {
        let x1 = self.x1.max(other.x1);
        let x2 = self.x2.min(other.x2);
        let y1 = self.y1.max(other.y1);
        let y2 = self.y2.min(other.y2);
        let z1 = self.z1.max(other.z1);
        let z2 = self.z2.min(other.z2);
        (x1 <= x2 && y1 <= y2 && z1 <= z2).then_some(Cube { x1, x2, y1, y2, z1, z2 })
    }

    fn volume(&self) -> i64 {
        let w = (self.x2 - self.x1 + 1) as i64;
        let h = (self.y2 - self.y1 + 1) as i64;
        let d = (self.z2 - self.z1 + 1) as i64;
        w * h * d
    }
}

pub fn parse(input: &str) -> Vec<RebootStep> {
    let first = input.split_ascii_whitespace().step_by(2);
    let second = input.iter_signed().chunk::<6>();
    first.zip(second).map(RebootStep::from).collect()
}

pub fn part1(input: &[RebootStep]) -> i64 {
    let region = Cube { x1: -50, x2: 50, y1: -50, y2: 50, z1: -50, z2: 50 };

    let filtered: Vec<_> = input
        .iter()
        .flat_map(|RebootStep { on, cube }| {
            region.intersect(cube).map(|next| RebootStep { on: *on, cube: next })
        })
        .collect();

    part2(&filtered)
}

pub fn part2(input: &[RebootStep]) -> i64 {
    let mut total = 0;
    let mut candidates = Vec::new();
    let on_cubes = input.iter().enumerate().filter_map(|(i, rs)| rs.on.then_some((i, rs.cube)));

    for (i, cube) in on_cubes {
        input[(i + 1)..]
            .iter()
            .flat_map(|rs| cube.intersect(&rs.cube))
            .for_each(|next| candidates.push(next));

        total += cube.volume() + subsets(&cube, -1, &candidates);
        candidates.clear();
    }

    total
}

fn subsets(cube: &Cube, sign: i64, candidates: &[Cube]) -> i64 {
    let mut total = 0;

    for (i, other) in candidates.iter().enumerate() {
        if let Some(next) = cube.intersect(other) {
            total += sign * next.volume() + subsets(&next, -sign, &candidates[(i + 1)..]);
        }
    }

    total
}
