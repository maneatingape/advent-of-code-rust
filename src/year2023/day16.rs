//! # The Floor Will Be Lava
//!
//! Each `-` or `|` splitter is a node in a graph connected by the light beams. Although each
//! splitter emits two beams the graph is not a binary tree. There can be cycles between splitters
//! and beams can also leave the grid.
//!
//! To speed things up
//! [Tarjan's algorithm](https://en.wikipedia.org/wiki/Tarjan%27s_strongly_connected_components_algorithm)
//! is used to find cycles in the graph, then the energized tiles are cached in reverse topological
//! order. As some cycles contain about half the total splitters in the grid, this results in a
//! significant savings.
//!
//! A specialized bit set is used to cache the energized tiles. Each input is 110 x 110 tiles,
//! needing 12,100 bits or 190 `u64`s to store the grid. Bitwise logic allows merging bitsets
//! and counting the number of elements very quickly.
use crate::util::grid::*;
use crate::util::hash::*;
use crate::util::point::*;
use State::*;

type Input = (u32, u32);

struct Graph {
    grid: Grid<u8>,
    seen: Grid<[bool; 2]>,
    state: Grid<State>,
    stack: Vec<usize>,
    nodes: Vec<Node>,
}

struct Node {
    tiles: BitSet,
    from: FastSet<Point>,
    to: FastSet<Point>,
}

impl Node {
    fn new() -> Self {
        Node { tiles: BitSet::new(), from: FastSet::new(), to: FastSet::new() }
    }
}

/// Fixed size bitset large enough to store the entire 110 x 110 grid.
struct BitSet {
    bits: [u64; 190],
}

impl BitSet {
    fn new() -> Self {
        BitSet { bits: [0; 190] }
    }

    fn insert(&mut self, position: Point) {
        let index = (110 * position.y + position.x) as usize;
        let base = index / 64;
        let offset = index % 64;
        self.bits[base] |= 1 << offset;
    }

    fn union(&mut self, other: &BitSet) {
        self.bits.iter_mut().zip(other.bits.iter()).for_each(|(a, b)| *a |= b);
    }

    fn size(&self) -> u32 {
        self.bits.iter().map(|&b| b.count_ones()).sum()
    }
}

/// Used by Tarjan's algorithm.
#[derive(Clone, Copy)]
enum State {
    Todo,
    OnStack(usize),
    Done(usize),
}

/// Computes both parts together in order to reuse cached results.
pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);
    let width = grid.width;
    let height = grid.height;

    let graph = &mut Graph {
        grid,
        seen: Grid::new(width, height, [false; 2]),
        state: Grid::new(width, height, Todo),
        stack: Vec::new(),
        nodes: Vec::new(),
    };

    let part_one = follow(graph, ORIGIN, RIGHT);
    let mut part_two = part_one;

    for x in 0..width {
        part_two = part_two.max(follow(graph, Point::new(x, 0), DOWN));
        part_two = part_two.max(follow(graph, Point::new(x, height - 1), UP));
    }

    for y in 0..height {
        part_two = part_two.max(follow(graph, Point::new(0, y), RIGHT));
        part_two = part_two.max(follow(graph, Point::new(width - 1, y), LEFT));
    }

    (part_one, part_two)
}

pub fn part1(input: &Input) -> u32 {
    input.0
}

pub fn part2(input: &Input) -> u32 {
    input.1
}

/// Starting from an edge, find either the first node marked by a splitter of any orientation or
/// exit from another edge of the grid. If the node is not yet computed, then recursively compute
/// the node and all descendants, caching the result to speed up future checks.
///
/// This does not mark paths in the grid to avoid corrupting potential future calculations.
fn follow(graph: &mut Graph, mut position: Point, mut direction: Point) -> u32 {
    let mut node = Node::new();

    while graph.grid.contains(position) {
        match graph.grid[position] {
            // Retrieve cached value or compute recursively.
            b'|' | b'-' => {
                let index = match graph.state[position] {
                    Todo => strong_connect(graph, position),
                    Done(index) => index,
                    OnStack(_) => unreachable!(),
                };
                node.tiles.union(&graph.nodes[index].tiles);
                break;
            }
            // Mirrors change direction.
            b'\\' => direction = Point::new(direction.y, direction.x),
            b'/' => direction = Point::new(-direction.y, -direction.x),
            // If we have already travelled on this path then this must be an exit from a splitter
            // node already computed. The energized tiles can be at most equal so exit early.
            _ => {
                let index = (direction == LEFT || direction == RIGHT) as usize;
                if graph.seen[position][index] {
                    return 0;
                }
            }
        }

        node.tiles.insert(position);
        position += direction;
    }

    node.tiles.size()
}

/// Traces the path of a beam until we hit the flat side of another splitter or exit the grid.
fn beam(graph: &mut Graph, node: &mut Node, mut position: Point, mut direction: Point) {
    while graph.grid.contains(position) {
        match graph.grid[position] {
            b'|' => {
                // If we encounter the pointy edge of a splitter then this additional splitter is
                // also part of this node. Nodes can contain multiple splitters in the same path.
                if direction == UP || direction == DOWN {
                    node.from.insert(position);
                } else {
                    node.to.insert(position);
                    break;
                }
            }
            b'-' => {
                if direction == LEFT || direction == RIGHT {
                    node.from.insert(position);
                } else {
                    node.to.insert(position);
                    break;
                }
            }
            // Mirrors change direction.
            b'\\' => direction = Point::new(direction.y, direction.x),
            b'/' => direction = Point::new(-direction.y, -direction.x),
            // If we are travelling horizontally or vertically in the same tile where
            // we have travelled in the same orientation before, then we're in a loop so break.
            // Beams can cross perpendicularly without causing a cycle.
            _ => {
                let index = (direction == LEFT || direction == RIGHT) as usize;
                if graph.seen[position][index] {
                    break;
                }
                graph.seen[position][index] = true;
            }
        }

        node.tiles.insert(position);
        position += direction;
    }
}

/// Tarjan's algorithm to find strongly connected components, e.g cycles in a directed graph.
fn strong_connect(graph: &mut Graph, position: Point) -> usize {
    // Push current index to stack and insert a dummy node to keep the vector index correct when
    // processing children.
    let index = graph.nodes.len();
    graph.stack.push(index);
    graph.nodes.push(Node::new());

    // Find all tiles energized by this node, the splitters that are part of it (`from`) and the
    // possible splitters that are children (`to`).
    let mut node = Node::new();

    if graph.grid[position] == b'|' {
        beam(graph, &mut node, position, UP);
        beam(graph, &mut node, position + DOWN, DOWN);
    } else {
        beam(graph, &mut node, position, LEFT);
        beam(graph, &mut node, position + RIGHT, RIGHT);
    }

    // Mark all splitters belonging to this node as in progress.
    node.from.iter().for_each(|&p| graph.state[p] = OnStack(index));

    // If any children connect to a previous node then lowlink will become less than current index.
    let mut lowlink = index;

    for &next in &node.to {
        match graph.state[next] {
            Todo => lowlink = lowlink.min(strong_connect(graph, next)),
            OnStack(other) => lowlink = lowlink.min(other),
            Done(_) => (),
        }
    }

    // We are the root of a cycle (possibly an independent component of one).
    if lowlink == index {
        // Merge all nodes in the cycle into this one.
        while let Some(next) = graph.stack.pop()
            && next != index
        {
            let other = &graph.nodes[next];
            node.tiles.union(&other.tiles);
            node.from.extend(other.from.iter());
            node.to.extend(other.to.iter());
        }

        // Mark node as done.
        node.from.iter().for_each(|&p| graph.state[p] = Done(index));

        // Merge depduplicated children, removing self-references that point to this node.
        for &next in node.to.difference(&node.from) {
            if let Done(other) = graph.state[next] {
                node.tiles.union(&graph.nodes[other].tiles);
            }
        }
    }

    // Replace dummy node with real thing.
    graph.nodes[index] = node;
    lowlink
}
