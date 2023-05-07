pub fn parse(input: &str) -> Vec<&[u8]> {
    input.lines().map(|line| line.as_bytes()).collect()
}

pub fn part1(input: &Vec<&[u8]>) -> usize {
    let nice = |line: &&&[u8]| {
        let mut vowels = 0;
        let mut pairs = 0;
        let mut previous = b' ';

        for &c in line.iter() {
            if c == b'a' || c == b'e' || c == b'i' || c == b'o' || c == b'u' {
                vowels += 1;
            }
            if c == previous {
                pairs += 1;
            }
            match (previous, c) {
                (b'a', b'b') | (b'c', b'd') | (b'p', b'q') | (b'x', b'y') => return false,
                _ => previous = c,
            }
        }

        vowels >= 3 && pairs >= 1
    };

    input.iter().filter(nice).count()
}

pub fn part2(input: &Vec<&[u8]>) -> usize {
    let nice = |line: &&&[u8]| {
        let mut first = 0;
        let mut second = 0;
        let mut pairs = [None; 729];

        let mut two_pair = false;
        let mut split_pair = false;

        for (i, c) in line.iter().enumerate() {
            let third = (c - 96) as usize;
            let offset = 27 * second + third;

            match pairs[offset] {
                Some(previous) => if i - previous > 1 { two_pair = true; }
                None => pairs[offset] = Some(i),

            }
            if first == third {
                split_pair = true;
            }

            first = second;
            second = third;
        }

        two_pair && split_pair
    };

    input.iter().filter(nice).count()
}
