pub fn parse(input: &str) -> Vec<usize> {    
    input.trim().chars().map(|c| c as usize).collect()
}

pub fn part1(input: &[usize]) -> usize {
    find(input, 4)
}

pub fn part2(input: &[usize]) -> usize {
    find(input, 14)
}

fn find(input: &[usize], marker: usize) -> usize {    
    let mut letters = [0; 128];
    let mut different = 0;

    for i in 0..input.len() {
        let new = input[i];
        letters[new] += 1;
        if letters[new] == 1 { different += 1; }

        if i >= marker {
            let old = input[i - marker];
            letters[old] -= 1;
            if letters[old] == 0 { different -= 1; }
        }

        if different == marker { return i + 1 }
    }

    unreachable!()
}
