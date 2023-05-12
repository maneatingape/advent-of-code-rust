pub fn parse(input: &str) -> Vec<&[u8]> {
    input.lines().map(|line| line.as_bytes()).collect()
}

pub fn part1(input: &Vec<&[u8]>) -> usize {
    let nice = |line: &&&[u8]| {
        let mut vowels = 0;
        let mut pairs = 0;
        let mut previous = 0;

        for c in line.iter() {
            let current = 1 << (c - b'a');
            if 0x101000a & current & (previous << 1) != 0 {
                return false;
            }
            if 0x0104111 & current != 0 {
                vowels += 1;
            }
            if previous == current {
                pairs += 1;
            } else {
                previous = current;
            }
        }

        vowels >= 3 && pairs >= 1
    };

    input.iter().filter(nice).count()
}

pub fn part2(input: &Vec<&[u8]>) -> usize {
    let mut pairs = [0; 729];

    let nice = |(base, line): &(usize, &&[u8])| {
        let mut first = 0;
        let mut second = 0;

        let mut two_pair = false;
        let mut split_pair = false;

        for (offset, c) in line.iter().enumerate() {
            let third = (c - b'a' + 1) as usize;
            let index = 27 * second + third;

            let position = base * 1000 + offset;
            let delta = position - pairs[index];

            if delta > offset {
                pairs[index] = position;
            } else if delta > 1 {
                two_pair = true;
            }
            if first == third {
                split_pair = true;
            }

            first = second;
            second = third;
        }

        two_pair && split_pair
    };

    input.iter().enumerate().filter(nice).count()
}
