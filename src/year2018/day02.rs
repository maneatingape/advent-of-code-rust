//! # Inventory Management System

pub fn parse(input: &str) -> Vec<&[u8]> {
    input.lines().map(str::as_bytes).collect()
}

pub fn part1(input: &[&[u8]]) -> u32 {
    let mut total_twos = 0;
    let mut total_threes = 0;

    for &id in input {
        // Ids are lowercase ASCII only with cardinality of 26.
        let mut freq = [0; 26];
        let mut twos = 0;
        let mut threes = 0;

        for &b in id {
            let index = (b - b'a') as usize;
            let current = freq[index];

            match current {
                0 => (),
                1 => twos += 1,
                2 => {
                    twos -= 1;
                    threes += 1;
                }
                _ => threes -= 1,
            }

            freq[index] += 1;
        }

        if twos > 0 {
            total_twos += 1;
        }
        if threes > 0 {
            total_threes += 1;
        }
    }

    total_twos * total_threes
}

pub fn part2(input: &[&[u8]]) -> String {
    // Manually compare all IDs, as it is faster than other methods considering there are so few total IDs
    for i in 0..input.len() {
        for ii in i + 1..input.len() {
            let id1 = input[i];
            let id2 = input[ii];

            let mut diff = false;
            for (a, b) in id1.iter().zip(id2) {
                if a != b {
                    if diff {
                        diff = false;
                        break;
                    }
                    diff = true;
                }
            }

            if diff {
                // Build the string of characters which are the same between both IDs
                return id1
                    .iter()
                    .zip(id2)
                    .filter_map(|(a, b)| (a == b).then(|| char::from(*a)))
                    .collect();
            }
        }
    }
    unreachable!()
}
