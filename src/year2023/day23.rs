//! # A Long Walk
//!
//! The [longest path problem](https://en.wikipedia.org/wiki/Longest_path_problem) is NP-hard and
//! requires an exhaustive search through all possible permutations. To speed things up we use
//! several tricks to reduce the complexity.
//!
//! ## Compression
//!
//! First we "compress" the maze into a much smaller simpler graph. For example the following maze
//! converts into a undirected weighted graph.
//!
//! ```none
//!     #.#####
//!     #....##    Start - A - B
//!     ##.#.## =>         |   |
//!     ##....#            C - D - End   (edge weights are 2)
//!     #####.#
//! ```
//!
//! Each actual input forms a graph of the same shape but with different edge weights that
//! looks like:
//!
//! ```none
//!     Start -  a - b - c - d - e
//!              |   |   |   |   | \
//!              f - A - B - C - D - g
//!              |   |   |   |   |   |
//!              h - E - F - G - H - k
//!              |   |   |   |   |   |
//!              m - K - M - N - P - n
//!              |   |   |   |   |   |
//!              p - Q - R - S - T - q
//!                \ |   |   |   |   |
//!                  r - s - t - u - v - End
//! ```
//!
//! ## Conversion to grid
//!
//! Next we convert this graph into a 6 x 6 square graph that can represented in an array. The
//! start and end are moved to the corners and extra nodes added to the other corners.
//!
//! ```none
//!     Start - b - c - d - e - e`
//!         |   |   |   |   |   |
//!         f - A - B - C - D - g
//!         |   |   |   |   |   |
//!         h - E - F - G - H - k
//!         |   |   |   |   |   |
//!         m - K - M - N - P - n
//!         |   |   |   |   |   |
//!         p - Q - R - S - T - q
//!         |   |   |   |   |   |
//!         p`- r - s - t - u - End
//! ```
//!
//! ## Dynamic programming
//!
//! For a 6x6 grid graph there are 1262816 total possible rook walks, given by
//! [OEIS A007764](https://oeis.org/A007764). However since we want the longest path it only makes
//! sense to consider the paths that visit the most possible nodes, in this case 35 (we have to
//! skip 1). There are only 10180 of these paths making it much faster.
//!
//! A row by row dynamic programming approach from top to bottom finds these paths. For each row
//! we calculate all possible next rows. Interestingly it turns out that there are only 76 possible
//! different rows. Then at each y coordinate we **deduplicate** rows to find the maximum value.
//! This is the most important optimisation as it means that each row is at most 76 elements
//! instead of growing exponentially (76², 76³, ...)
//!
//! ## Example paths
//!
//! Using `S` to represents the start of a line segment and `E` to represent the end, the starting
//! state looks like `S.....` and the end state `.....S`. One example:
//!
//! ```none
//!    Start    S..... |
//!    Row 0    ..SS.E └─┐┌─┐
//!    Row 1    S..S.E ┌─┘|.|
//!    Row 2    ..SSE. └─┐|┌┘
//!    Row 3    SE...S ┌┐└┘└┐
//!    Row 4    S..... |└───┘
//!    Row 5    .....S └────┐
//!    End      .....S      |
//! ```
//!
//! Another example:
//!
//! ```none
//!    Start    S..... |
//!    Row 0    .SSESE └┐┌┐┌┐
//!    Row 1    S.SESE ┌┘||||
//!    Row 2    ...SSE └─┘|||
//!    Row 3    S.E..S ┌─┐└┘|
//!    Row 4    S..... |.└──┘
//!    Row 5    .....S └────┐
//!    End      .....S      |
//! ```
//!
//! ## Next row generation
//!
//! To create the next row from a given row, there are 5 possibilites for each of the 6 columns.
//!
//! ### Leave a blank space, skipping over the column.
//!
//! We do this at most once per row. For example:
//!
//! ```none
//!     Previous .SSESE └┐┌┐┌┐
//!     Current  .....S .└┘└┘|
//!                     ^ Blank space
//! ```
//!
//! ### Start a new (start, end) pair of lines.
//!
//! All lines must eventually connect so we must create lines in pairs. For example:
//!
//! ```none
//!     Previous .....S └────┐
//!     Current  S...ES ┌───┐|
//!                     ^   ^
//!                     New pair
//! ```
//!
//! ### Continue a straight line down from the previous row.
//!
//! The line stays the same kind (`S` or `E`).
//!
//! ```none
//!     Previous .....S └────┐
//!     Current  S...ES ┌───┐|
//!                          ^ Continuation
//! ```
//!
//! ### Move a previous downwards line to the left or right into a different column.
//!
//! The line stays the same kind (`S` or `E`).
//!
//! ```none
//!     Previous .....S └────┐
//!     Current  S..... ┌────┘
//!                     ^ Move
//! ```
//!
//! ### Join two open segments from a previous row.
//!
//! A restriction is that we can't create closed cycles that don't connect to the start or end,
//! as this would skip several nodes. For example this is not allowed:
//!
//! ```none
//!     Previous .....S |┌───┐
//!     Current  S..... |└───┘
//!                      ^ Closed cycles not allowed
//! ```
//!
//! We implement this by not joining any (`S`, `E`) pairs in that order. Joing the reverse order
//! (`E`, `S`) is allowed.
//!
//! Finally there are two special rules when joining two nested line segments.
//! When joining (`S`, `S`) the next `E` convert to an `S` to maintain balance.
//!
//! ```none
//!     Previous S..E ┌──┐
//!     Previous SSEE |┌┐|
//!     Current  ..SE └┘||
//! ```
//!
//! When joining (`E`, `E`) the previous `S` convert to an `E` to maintain balance.
//!
//! ```none
//!     Previous S..E ┌──┐
//!     Previous SSEE |┌┐|
//!     Current  SE.. ||└┘
//! ```
use crate::util::bitset::*;
use crate::util::grid::*;
use crate::util::hash::*;
use crate::util::point::*;
use std::collections::VecDeque;

/// We only use 6 elements but 8 is faster to hash.
type Row = [u8; 8];

/// Undirected weighted graph representing the compressed maze.
struct Graph {
    start: Point,
    end: Point,
    edges: FastMap<Point, Vec<Point>>,
    weight: FastMap<(Point, Point), u32>,
}

/// Distilled two dimensional array of only weights.
pub struct Input {
    extra: u32,
    horizontal: [[u32; 6]; 6],
    vertical: [[u32; 6]; 6],
}

/// Simplify input for faster processing.
pub fn parse(input: &str) -> Input {
    // Convert the raw maze input into a compressed graph.
    let graph = compress(input);
    // Convert graph to a 6x6 square grid.
    graph_to_grid(&graph)
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
    let start = [b'S', 0, 0, 0, 0, 0, 0, 0];
    let end = [0, 0, 0, 0, 0, b'S', 0, 0];

    // Compute all possible different 76 rows and the next possible row.
    let mut todo = VecDeque::new();
    let mut seen = FastSet::new();
    let mut graph = FastMap::new();

    todo.push_back(start);
    seen.insert(start);

    while let Some(row) = todo.pop_front() {
        let mut neighbors = Vec::new();
        dfs(&mut neighbors, row, [0; 8], 0, false, 0, 0);

        for &(next, ..) in &neighbors {
            if seen.insert(next) {
                todo.push_back(next);
            }
        }

        graph.insert(row, neighbors);
    }

    // Step through the each row of the grid, keeping track of the maximum value for each
    // row type.
    let mut current = FastMap::new();
    let mut next = FastMap::new();

    current.insert((start, false), 0);

    for y in 0..6 {
        for ((row, gap), steps) in current.drain() {
            for &(next_row, next_gap, horizontal, vertical) in &graph[&row] {
                // Only 1 gap total is allowed, otherwise we can make a longer path.
                if !(gap && next_gap) {
                    // The bit sets represent the horizonal and vertical moves from the
                    // previous row.
                    let extra = horizontal.biterator().map(|x| input.horizontal[y][x]).sum::<u32>()
                        + vertical.biterator().map(|x| input.vertical[y][x]).sum::<u32>();

                    let e = next.entry((next_row, gap || next_gap)).or_insert(0);
                    *e = (*e).max(steps + extra);
                }
            }
        }

        (current, next) = (next, current);
    }

    // The maximum path must have skipped 1 node.
    input.extra + current[&(end, true)]
}

/// Convert maze to unidrected graph.
fn compress(input: &str) -> Graph {
    let mut grid = Grid::parse(input);
    let width = grid.width;
    let height = grid.height;

    // Move start and end away from edge.
    let start = Point::new(1, 1);
    let end = Point::new(width - 2, height - 2);

    // Modify edge of grid to remove the need for boundary checks.
    grid[start + UP] = b'#';
    grid[end + DOWN] = b'#';

    // BFS to find distances between POIs. Points of interest are the start, the end and junctions.
    let mut poi = VecDeque::new();
    let mut seen = FastSet::new();
    let mut edges = FastMap::new();
    let mut weight = FastMap::new();

    poi.push_back(start);
    grid[end] = b'P';

    while let Some(from) = poi.pop_front() {
        // Mark this POI as done.
        grid[from] = b'#';

        for direction in ORTHOGONAL {
            if grid[from + direction] != b'#' {
                let mut to = from + direction;
                let mut cost = 1;

                while grid[to] != b'P' {
                    let mut neighbors =
                        ORTHOGONAL.iter().map(|&o| to + o).filter(|&n| grid[n] != b'#');
                    let next = neighbors.next().unwrap();

                    // More than 1 neighbor means that we've reached a junction.
                    // Mark it as a POI then stop.
                    if neighbors.next().is_some() {
                        grid[to] = b'P';
                        break;
                    }

                    // Follow maze path towards next POI.
                    grid[to] = b'#';
                    to = next;
                    cost += 1;
                }

                // Graph is undirected so add both edges.
                edges.entry(from).or_insert(Vec::new()).push(to);
                edges.entry(to).or_insert(Vec::new()).push(from);
                weight.insert((from, to), cost);
                weight.insert((to, from), cost);

                // Queue POI for processing if we haven't seen it before.
                if seen.insert(to) {
                    poi.push_back(to);
                }
            }
        }
    }

    Graph { start, end, edges, weight }
}

/// Convert graph to 6 x6 two dimensional array of weights.
fn graph_to_grid(graph: &Graph) -> Input {
    let Graph { start, end, edges, weight } = graph;

    // There's only 1 edge from both the start and end nodes, so we always have to travel these
    // steps. Add 2 steps to account for moving the start and end positions in the previous step.
    let extra = 2 + weight[&(*start, edges[start][0])] + weight[&(*end, edges[end][0])];

    // Perimeter nodes only have 3 edges. Interior nodes have 4 edges.
    let mut seen = FastSet::new();
    let mut next_perimeter = |point: &Point| {
        *edges[point].iter().find(|&&next| edges[&next].len() == 3 && seen.insert(next)).unwrap()
    };

    let mut grid = [[ORIGIN; 6]; 6];
    let mut horizontal = [[0; 6]; 6];
    let mut vertical = [[0; 6]; 6];

    // Place start in top left.
    grid[0][0] = next_perimeter(start);

    // Fill out top edge and left edge. Since the graph is square it doesn't matter which of the
    // 2 children becomes top and which becomes left.
    for i in 1..5 {
        let left = grid[0][i - 1];
        let above = grid[i - 1][0];

        let next_left = next_perimeter(&left);
        let next_above = next_perimeter(&above);

        grid[0][i] = next_left;
        grid[i][0] = next_above;
        horizontal[0][i - 1] = weight[&(left, next_left)];
        vertical[i - 1][0] = weight[&(above, next_above)];
    }

    // Add two extra corners by duplicating the last node in the row or column.
    // This won't affect the overall path as the weight of the added edge is 0.
    grid[0][5] = grid[0][4];
    grid[5][0] = grid[4][0];

    // Add remaining interior nodes.
    for y in 1..6 {
        for x in 1..6 {
            let left = grid[y][x - 1];
            let above = grid[y - 1][x];

            let (&next, _) = edges
                .iter()
                .find(|(k, v)| v.contains(&above) && v.contains(&left) && seen.insert(**k))
                .unwrap();

            grid[y][x] = next;
            horizontal[y][x - 1] = weight[&(left, next)];
            vertical[y - 1][x] = weight[&(above, next)];
        }
    }

    Input { extra, horizontal, vertical }
}

/// Modified depth first search that only allows rows that skip one node.
fn dfs(
    result: &mut Vec<(Row, bool, u32, u32)>,
    previous: Row,
    current: Row,
    start: usize,
    gap: bool,
    horizontal: u32,
    vertical: u32,
) {
    // We're done, push the result to the possible successors.
    if start == 6 {
        result.push((current, gap, horizontal, vertical));
        return;
    }

    // Previous row above has no vertical descending path.
    if previous[start] == 0 {
        // Skip at most 1 column per row.
        if !gap {
            dfs(result, previous, current, start + 1, true, horizontal, vertical);
        }

        let mut horizontal = horizontal;

        for end in (start + 1)..6 {
            horizontal |= 1 << (end - 1);

            if previous[end] == 0 {
                // Start a new path pair.
                let mut next = current;
                next[start] = b'S';
                next[end] = b'E';

                let vertical = vertical | (1 << start) | (1 << end);

                dfs(result, previous, next, end + 1, gap, horizontal, vertical);
            } else {
                // Move an existing path.
                let mut next = current;
                next[start] = previous[end];

                let vertical = vertical | (1 << start);

                dfs(result, previous, next, end + 1, gap, horizontal, vertical);
                break;
            }
        }
    } else {
        // Continue vertical path straight down.
        let mut next = current;
        next[start] = previous[start];
        dfs(result, previous, next, start + 1, gap, horizontal, vertical | (1 << start));

        let mut horizontal = horizontal;

        for end in (start + 1)..6 {
            horizontal |= 1 << (end - 1);

            if previous[end] == 0 {
                // Move existing path.
                let mut next = current;
                next[end] = previous[start];

                let vertical = vertical | (1 << end);

                dfs(result, previous, next, end + 1, gap, horizontal, vertical);
            } else {
                // Merge two path segments.
                match (previous[start], previous[end]) {
                    // No other changes needed.
                    (b'E', b'S') => {
                        dfs(result, previous, current, end + 1, gap, horizontal, vertical);
                    }
                    // Convert previous S to E.
                    (b'E', b'E') => {
                        let mut next = current;

                        for i in (0..start).rev() {
                            if current[i] == b'S' {
                                next[i] = b'E';
                                break;
                            }
                        }

                        dfs(result, previous, next, end + 1, gap, horizontal, vertical);
                    }
                    // Convert next E to S.
                    (b'S', b'S') => {
                        let mut modified = previous;
                        let mut level = 0;

                        for i in (end + 1)..6 {
                            if previous[i] == b'S' {
                                level += 1;
                            }
                            if previous[i] == b'E' {
                                if level == 0 {
                                    modified[i] = b'S';
                                    break;
                                }
                                level -= 1;
                            }
                        }

                        dfs(result, modified, current, end + 1, gap, horizontal, vertical);
                    }
                    _ => (), // (S, E) not allowed
                }
                break;
            }
        }
    }
}
