pub fn parse(input: &str) -> Vec<&[u8]> {
    input.lines().map(str::as_bytes).collect()
}

pub fn part1(input: &[&[u8]]) -> String {
    to_snafu(input.iter().map(from_snafu).sum())
}

pub fn part2(_input: &[&[u8]]) -> &'static str {
    "n/a"
}

fn from_snafu(snafu: &&[u8]) -> i64 {
    snafu.iter().fold(0, |acc, c| {
        let digit = match c {
            b'=' => -2,
            b'-' => -1,
            b'0' => 0,
            b'1' => 1,
            b'2' => 2,
            _ => unreachable!(),
        };
        5 * acc + digit
    })
}

fn to_snafu(decimal: i64) -> String {
    let mut n = decimal;
    let mut digits = String::new();

    while n > 0 {
        let next = match n % 5 {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '=',
            4 => '-',
            _ => unreachable!(),
        };
        digits.insert(0, next);
        n = (n + 2) / 5;
    }

    digits
}
