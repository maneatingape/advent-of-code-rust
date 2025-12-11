//! # Reactor
type Input = Vec<Vec<usize>>;

pub fn parse(input: &str) -> Input {
    let mut graph = vec![vec![]; 26 * 26 * 26];

    for line in input.lines() {
        let mut edges = line.split_ascii_whitespace();
        let from = edges.next().unwrap();
        graph[to_index(from)].extend(edges.map(to_index));
    }

    graph
}

pub fn part1(input: &Input) -> u64 {
    paths(input, "you", "out")
}

pub fn part2(input: &Input) -> u64 {
    let one = paths(input, "svr", "fft") * paths(input, "fft", "dac") * paths(input, "dac", "out");
    let two = paths(input, "svr", "dac") * paths(input, "dac", "fft") * paths(input, "fft", "out");
    one + two
}

fn paths(input: &Input, from: &str, to: &str) -> u64 {
    let mut cache = vec![u64::MAX; input.len()];
    dfs(input, &mut cache, to_index(from), to_index(to))
}

fn dfs(input: &Input, cache: &mut [u64], node: usize, end: usize) -> u64 {
    if node == end {
        1
    } else if cache[node] == u64::MAX {
        let result = input[node].iter().map(|&next| dfs(input, cache, next, end)).sum();
        cache[node] = result;
        result
    } else {
        cache[node]
    }
}

fn to_index(s: &str) -> usize {
    s.bytes().take(3).fold(0, |acc, b| 26 * acc + usize::from(b - b'a'))
}
