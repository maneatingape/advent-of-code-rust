use crate::util::grid::*;
use crate::util::hash::*;
use crate::util::point::*;
use std::collections::VecDeque;

pub struct Input {
    extra: u32,
    horizontal: [[u32; 6]; 6],
    vertical: [[u32; 6]; 6],
}

struct State {
    letter: u8,
    skipped: bool,
    grid: [[u8; 6]; 7],
    convert: [u8; 32],
    result: u32,
}

pub fn parse(input: &str) -> Input {
    let mut grid = Grid::parse(input);
    let width = grid.width;
    let height = grid.height;

    // Modify edge of grid to remove the need for boundary checks.
    grid[Point::new(1, 0)] = b'#';
    grid[Point::new(width - 2, height - 1)] = b'#';

    // Move start and end away from edge.
    let start = Point::new(1, 1);
    let end = Point::new(width - 2, height - 2);

    // Points of interest are start, end and junctions.
    grid[start] = b'P';
    grid[end] = b'P';

    let mut poi = Vec::new();
    poi.push(start);
    poi.push(end);

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let position = Point::new(x, y);

            if grid[position] != b'#' {
                let neighbors =
                    ORTHOGONAL.iter().map(|&o| position + o).filter(|&n| grid[n] != b'#').count();
                if neighbors > 2 {
                    grid[position] = b'P';
                    poi.push(position);
                }
            }
        }
    }

    // BFS to find distances between POIs.
    let mut todo = VecDeque::new();
    let mut edges = FastMap::new();
    let mut weight = FastMap::new();

    for from in poi {
        todo.push_back((from, 0));
        grid[from] = b'#';
        weight.insert((from, from), 0);

        while let Some((position, cost)) = todo.pop_front() {
            for direction in ORTHOGONAL {
                let to = position + direction;

                match grid[to] {
                    b'#' => (),
                    b'P' => {
                        edges.entry(from).or_insert(FastSet::new()).insert(to);
                        edges.entry(to).or_insert(FastSet::new()).insert(from);
                        weight.insert((from, to), cost + 1);
                        weight.insert((to, from), cost + 1);
                    }
                    _ => {
                        todo.push_back((to, cost + 1));
                        grid[to] = b'#';
                    }
                }
            }
        }
    }

    // Convert reduced graph to a 6x6 square grid.
    graph_to_grid(start, end, &edges, &weight)
}

/// The graph is directed so the only allowed steps are down or to the right. The maximum value
/// for any cell is the maximum of either the cell to the left or above.
pub fn part1(input: &Input) -> u32 {
    let mut total = [[0; 6]; 6];

    for y in 0..6 {
        for x in 0..6 {
            let left = if x == 0 { 0 } else { total[y][x - 1] + input.horizontal[y][x - 1] };
            let above = if y == 0 { 0 } else { total[y - 1][x] + input.vertical[y - 1][x] };
            total[y][x] = left.max(above);
        }
    }

    input.extra + total[5][5]
}

/// Graph is undirected so we can also move up or to the right.
pub fn part2(input: &Input) -> u32 {
    let mut state =
        State { letter: 2, skipped: false, grid: [[0; 6]; 7], convert: [0; 32], result: 0 };

    state.grid[0][0] = 1;

    for i in 0..32 {
        state.convert[i] = i as u8;
    }

    dfs(input, &mut state, 0, 0, 0);
    input.extra + state.result
}

#[allow(clippy::needless_range_loop)]
fn graph_to_grid(
    start: Point,
    end: Point,
    edges: &FastMap<Point, FastSet<Point>>,
    weight: &FastMap<(Point, Point), u32>,
) -> Input {
    let mut extra = 2;
    extra += edges[&start].iter().map(|&e| weight[&(start, e)]).sum::<u32>();
    extra += edges[&end].iter().map(|&e| weight[&(e, end)]).sum::<u32>();

    let mut places = [[ORIGIN; 6]; 6];
    let mut horizontal = [[0; 6]; 6];
    let mut vertical = [[0; 6]; 6];

    let mut point = *edges[&start].iter().next().unwrap();
    let mut seen = FastSet::new();
    let mut next_perimeter = |point: Point| {
        seen.insert(point);
        *edges[&point]
            .iter()
            .find(|&next| edges[next].len() == 3 && !seen.contains(next))
            .unwrap_or(&ORIGIN)
    };

    for y in 0..5 {
        places[y][0] = point;
        point = next_perimeter(point);
    }

    for x in 1..6 {
        places[5][x] = point;
        point = next_perimeter(point);
    }

    for y in (1..5).rev() {
        places[y][5] = point;
        point = next_perimeter(point);
    }

    for x in (1..5).rev() {
        places[0][x] = point;
        point = next_perimeter(point);
    }

    for y in 1..5 {
        for x in 1..5 {
            let above = places[y - 1][x];
            let left = places[y][x - 1];
            let (&point, _) = edges
                .iter()
                .find(|(k, v)| !seen.contains(k) && v.contains(&above) && v.contains(&left))
                .unwrap();

            places[y][x] = point;
            seen.insert(point);
        }
    }

    places[0][5] = places[0][4];
    places[5][0] = places[5][1];

    for y in 0..6 {
        for x in 0..5 {
            let key = (places[y][x], places[y][x + 1]);
            horizontal[y][x] = *weight.get(&key).unwrap_or(&0);
        }
    }

    for y in 0..5 {
        for x in 0..6 {
            let key = (places[y][x], places[y + 1][x]);
            vertical[y][x] = *weight.get(&key).unwrap_or(&0);
        }
    }

    Input { extra, horizontal, vertical }
}

/// Modified depth first search that only allows paths that skip one node.
///
/// For a 6x6 grid there are 1262816 total possible rook walks
/// (given by [OEIS A007764](https://oeis.org/A007764)).
///
/// However since we want the longest path it only makes sense to consider the paths that visit the
/// most possible nodes. There are only 10180 of these paths making it much faster.
fn dfs(input: &Input, state: &mut State, mut row: usize, mut col: usize, mut steps: u32) {
    // Wrap around at end of each row.
    if col == 6 {
        // We've reached the bottom right corner.
        if row == 5 {
            state.result = state.result.max(steps);
            return;
        }
        row += 1;
        col = 0;
    }

    if state.grid[row][col] == 0 {
        // Skip only 1 node in each path.
        if !(state.skipped || (row == 5 && col == 5)) {
            state.skipped = true;
            state.grid[row + 1][col] = 0;
            dfs(input, state, row, col + 1, steps);
            state.skipped = false;
        }

        // Create new paths (except on the final row).
        if row < 5 {
            let id = state.letter;
            steps += input.vertical[row][col];

            for end in (col + 1)..6 {
                state.grid[row + 1][end - 1] = 0;
                steps += input.horizontal[row][end - 1];

                if state.grid[row][end] == 0 {
                    state.grid[row + 1][col] = id;
                    state.grid[row + 1][end] = id;
                    let extra = input.vertical[row][end];
                    state.letter += 1;
                    dfs(input, state, row, end + 1, steps + extra);
                    state.letter -= 1;
                } else {
                    state.grid[row + 1][col] = state.convert[state.grid[row][end] as usize];
                    state.grid[row + 1][end] = 0;
                    dfs(input, state, row, end + 1, steps);
                    break;
                }
            }
        }
    } else {
        let index = state.grid[row][col] as usize;
        let id = state.convert[index];

        // Straight down
        if row < 5 || col == 5 {
            state.grid[row + 1][col] = id;
            let extra = input.vertical[row][col];
            dfs(input, state, row, col + 1, steps + extra);
        }

        for end in (col + 1)..6 {
            state.grid[row + 1][end - 1] = 0;
            steps += input.horizontal[row][end - 1];

            if state.grid[row][end] == 0 {
                // Move down only if not final row (except final corner).
                if row < 5 || end == 5 {
                    state.grid[row + 1][end] = id;
                    let extra = input.vertical[row][end];
                    dfs(input, state, row, end + 1, steps + extra);
                }
            } else {
                // Join two path together as long as they are different.
                // (prevent disjoint loops)
                let other = state.convert[state.grid[row][end] as usize];

                if id != other {
                    state.grid[row + 1][end] = 0;
                    state.convert[index] = other;
                    dfs(input, state, row, end + 1, steps);
                    state.convert[index] = id;
                }

                break;
            }
        }
    }
}
