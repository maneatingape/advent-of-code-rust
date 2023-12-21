use crate::util::hash::*;
use crate::util::iter::*;
use crate::util::parse::*;

type Point3D = (u32, u32, u32);
type Brick = (Vec<Point3D>, Vec<Point3D>);
type Input = (usize, usize);

pub fn parse(input: &str) -> Input {
    let mut bricks = Vec::new();

    for [x1, y1, z1, x2, y2, z2] in input.iter_unsigned::<u32>().chunk::<6>() {
        let mut cubes = Vec::new();
        let mut shadow = Vec::new();

        if x2 > x1 {
            for x in x1..=x2 {
                cubes.push((x, y1, z1));
                shadow.push((x, y1, z1));
            }
        } else if y2 > y1 {
            for y in y1..=y2 {
                cubes.push((x1, y, z1));
                shadow.push((x1, y, z1));
            }
        } else if z2 > z1 {
            for z in z1..=z2 {
                cubes.push((x1, y1, z));
            }
            shadow.push((x1, y1, z1));
        } else {
            cubes.push((x1, y1, z1));
            shadow.push((x1, y1, z1));
        }

        bricks.push((cubes, shadow));
    }

    let (settled, _) = fall(&bricks);
    let mut part_one = 0;
    let mut part_two = 0;

    for i in 0..settled.len() {
        let mut first = settled.clone();
        first.remove(i);

        let (_, moved) = fall(&first);

        if moved == 0 {
            part_one += 1;
        }
        part_two += moved;
    }

    (part_one, part_two)
}

pub fn part1(input: &Input) -> usize {
    input.0
}

pub fn part2(input: &Input) -> usize {
    input.1
}

fn fall(bricks: &[Brick]) -> (Vec<Brick>, usize) {
    let mut bricks = bricks.to_vec();
    let mut space = FastSet::new();

    for brick in &bricks {
        for &point in &brick.0 {
            space.insert(point);
        }
    }

    let mut change = true;
    let mut moved = vec![false; bricks.len()];

    while change {
        change = false;

        for i in 0..bricks.len() {
            let brick = &mut bricks[i];
            let next = down(&brick.0, &brick.1);

            if next.1.iter().any(|point| space.contains(point)) {
                continue;
            }
            if next.1.iter().any(|&(_, _, z)| z == 0) {
                continue;
            }

            for point in &brick.0 {
                space.remove(point);
            }
            for &point in &next.0 {
                space.insert(point);
            }

            change = true;
            moved[i] = true;
            *brick = next;
        }
    }

    let sum = moved.iter().filter(|&&b| b).count();
    (bricks, sum)
}

fn down(cubes: &[Point3D], shadow: &[Point3D]) -> (Vec<Point3D>, Vec<Point3D>) {
    let first = cubes.iter().map(|&(x, y, z)| (x, y, z - 1)).collect();
    let second = shadow.iter().map(|&(x, y, z)| (x, y, z - 1)).collect();
    (first, second)
}
