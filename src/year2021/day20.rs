pub struct Input {
    size: usize,
    algorithm: [u8; 512],
    pixels: [u8; 40_000],
}

pub fn parse(input: &str) -> Input {
    fn convert(b: &u8) -> u8 {
        match b {
            b'#' => 1,
            b'.' => 0,
            _ => unreachable!(),
        }
    }

    let bits: Vec<Vec<_>> =
        input.lines().map(|line| line.as_bytes().iter().map(convert).collect()).collect();

    let size = bits.len() - 2;
    let algorithm = bits[0][..512].try_into().unwrap();

    let mut pixels = [0; 40_000];
    for (i, row) in bits[2..].iter().enumerate() {
        let start = (i + 50) * 200 + 50;
        let end = start + size;
        pixels[start..end].copy_from_slice(&row[..size]);
    }

    Input { size, algorithm, pixels }
}

pub fn part1(input: &Input) -> usize {
    enhance(input, 2)
}

pub fn part2(input: &Input) -> usize {
    enhance(input, 50)
}

fn enhance(input: &Input, steps: usize) -> usize {
    let algorithm = input.algorithm;
    let mut pixels = input.pixels;
    let mut next = [0; 40_000];

    let mut start = 50;
    let mut end = 50 + input.size as i32;
    let mut default = 0;

    for _ in 0..steps {
        for y in (start - 1)..(end + 1) {
            let helper = |sx, sy, shift| {
                let result = if sx < end && sy >= start && sy < end {
                    pixels[(sy * 200 + sx) as usize] as usize
                } else {
                    default as usize
                };
                result << shift
            };

            let mut index = if default == 1 { 0b11011011 } else { 0b00000000 };

            for x in (start - 1)..(end + 1) {
                index = ((index << 1) & 0b110110110)
                    + helper(x + 1, y - 1, 6)
                    + helper(x + 1, y, 3)
                    + helper(x + 1, y + 1, 0);

                next[(y * 200 + x) as usize] = algorithm[index];
            }
        }

        pixels = next;
        start -= 1;
        end += 1;
        if default == 0 {
            default = algorithm[0]
        } else {
            default = algorithm[511]
        }
    }

    pixels.iter().filter(|&&p| p == 1).count()
}
