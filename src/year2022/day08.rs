type Input = (usize, Vec<i8>);

pub fn parse(input: &str) -> Input {
    let bytes = input.as_bytes();
    let width = bytes.iter().position(|b| b.is_ascii_whitespace()).unwrap();
    let digits = bytes
        .iter()
        .filter(|b| b.is_ascii_digit())
        .map(|&b| 6 * (b - b'0') as i8)
        .collect();
    (width, digits)
}

pub fn part1(input: &Input) -> usize {
    let (width, digits) = input;
    let mut visible: Vec<bool> = vec![false; digits.len()];

    for i in 1..(*width - 1) {
        let mut left_max = -1;
        let mut right_max = -1;
        let mut top_max = -1;
        let mut bottom_max = -1;

        for j in 0..(*width - 1) {
            let left = (i * width) + j;
            if digits[left] > left_max {
                visible[left] = true;
                left_max = digits[left];
            }

            let right = (i * width) + (width - j - 1);
            if digits[right] > right_max {
                visible[right] = true;
                right_max = digits[right];
            }

            let top = (j * width) + i;
            if digits[top] > top_max {
                visible[top] = true;
                top_max = digits[top];
            }

            let bottom = (width - j - 1) * width + i;
            if digits[bottom] > bottom_max {
                visible[bottom] = true;
                bottom_max = digits[bottom];
            }
        }
    }

    4 + visible.iter().filter(|&&b| b).count()
}

pub fn part2(input: &Input) -> u64 {
    let (width, digits) = input;
    let ones: u64 = 0x0041041041041041;
    let mask: u64 = 0xffffffffffffffc0;
    let mut scenic = vec![1; digits.len()];

    for i in 1..(*width - 1) {
        let mut left_max = ones;
        let mut right_max = ones;
        let mut top_max = ones;
        let mut bottom_max = ones;

        for j in 1..(*width - 1) {
            let left = (i * width) + j;
            scenic[left] *= (left_max >> digits[left]) & 0x3f;
            left_max = (left_max & (mask << digits[left])) + ones;

            let right = (i * width) + (width - j - 1);
            scenic[right] *= (right_max >> digits[right]) & 0x3f;
            right_max = (right_max & (mask << digits[right])) + ones;

            let top = (j * width) + i;
            scenic[top] *= (top_max >> digits[top]) & 0x3f;
            top_max = (top_max & (mask << digits[top])) + ones;

            let bottom = (width - j - 1) * width + i;
            scenic[bottom] *= (bottom_max >> digits[bottom]) & 0x3f;
            bottom_max = (bottom_max & (mask << digits[bottom])) + ones;
        }
    }

    *scenic.iter().max().unwrap()
}
