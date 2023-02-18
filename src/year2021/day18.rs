type Snailfish = [i32; 63];

const IN_ORDER: [usize; 30] = [
    1, 3,  7, 15, 16,  8, 17, 18, 4,  9, 19, 20, 10, 21, 22,
    2, 5, 11, 23, 24, 12, 25, 26, 6, 13, 27, 28, 14, 29, 30,
];

pub fn parse(input: &str) -> Vec<Snailfish> {
    fn helper(bytes: &[u8]) -> Snailfish {
        let mut tree = [-1; 63];
        let mut i = 0;

        for &b in bytes.iter() {
            match b {
                b'[' => i = 2 * i + 1,
                b',' => i += 1,
                b']' => i = (i - 1) / 2,
                b => tree[i] = (b - 48) as i32,
            }
        }

        tree
    }
    input.lines().map(|line| helper(line.as_bytes())).collect()
}

pub fn part1(input: &[Snailfish]) -> i32 {
    let mut sum = input
        .iter()
        .copied()
        .reduce(|acc, n| add(&acc, &n))
        .unwrap();
    magnitude(&mut sum)
}

pub fn part2(input: &[Snailfish]) -> i32 {
    let mut result = 0;

    for (i, a) in input.iter().enumerate() {
        for (j, b) in input.iter().enumerate() {
            if i != j {
                result = result.max(magnitude(&mut add(a, b)));
            }
        }
    }

    result
}

fn add(left: &Snailfish, right: &Snailfish) -> Snailfish {
    let mut tree = [-1; 63];

    tree[3..5].copy_from_slice(&left[1..3]);
    tree[7..11].copy_from_slice(&left[3..7]);
    tree[15..23].copy_from_slice(&left[7..15]);
    tree[31..47].copy_from_slice(&left[15..31]);

    tree[5..7].copy_from_slice(&right[1..3]);
    tree[11..15].copy_from_slice(&right[3..7]);
    tree[23..31].copy_from_slice(&right[7..15]);
    tree[47..63].copy_from_slice(&right[15..31]);

    for pair in (31..63).step_by(2) {
        if tree[pair] >= 0 {
            explode(&mut tree, pair);
        }
    }

    while split(&mut tree) {}
    tree
}

fn explode(tree: &mut Snailfish, pair: usize) {
    if pair > 31 {
        let mut i = pair - 1;
        while i > 0 {
            if tree[i] >= 0 {
                tree[i] += tree[pair];
                break;
            }
            i = (i - 1) / 2;
        }
    }

    if pair < 61 {
        let mut i = pair + 2;
        while i > 0 {
            if tree[i] >= 0 {
                tree[i] += tree[pair + 1];
                break;
            }
            i = (i - 1) / 2;
        }
    }

    tree[pair] = -1;
    tree[pair + 1] = -1;
    tree[(pair - 1) / 2] = 0;
}

fn split(tree: &mut Snailfish) -> bool {
    for &i in IN_ORDER.iter() {
        if tree[i] >= 10 {
            tree[2 * i + 1] = tree[i] / 2;
            tree[2 * i + 2] = (tree[i] + 1) / 2;
            tree[i] = -1;

            if i >= 15 {
                explode(tree, 2 * i + 1);
            }
            return true;
        }
    }
    false
}

fn magnitude(tree: &mut Snailfish) -> i32 {
    for i in (0..31).rev() {
        if tree[i] == -1 {
            tree[i] = 3 * tree[2 * i + 1] + 2 * tree[2 * i + 2];
        }
    }
    tree[0]
}
