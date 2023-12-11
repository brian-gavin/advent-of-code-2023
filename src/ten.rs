use std::collections::HashMap;

use petgraph::{algo::dijkstra, graphmap::UnGraphMap};

use crate::Input;

#[derive(Clone, Copy, Debug)]
enum Pipe {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}

impl From<char> for Pipe {
    fn from(c: char) -> Self {
        use Pipe::*;
        match c {
            '|' => NS,
            '-' => EW,
            'L' => NE,
            'J' => NW,
            '7' => SW,
            'F' => SE,
            '.' => Ground,
            'S' => Start,
            _ => unreachable!(),
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Node {
    row: isize,
    col: isize,
}

impl Node {
    fn north(&self) -> Option<Node> {
        self.row.gt(&0).then(|| Node {
            row: self.row - 1,
            col: self.col,
        })
    }

    fn south(&self) -> Option<Node> {
        self.row.lt(&139).then(|| Node {
            row: self.row + 1,
            col: self.col,
        })
    }

    fn east(&self) -> Option<Node> {
        self.col.lt(&139).then(|| Node {
            row: self.row,
            col: self.col + 1,
        })
    }

    fn west(&self) -> Option<Node> {
        self.col.gt(&0).then(|| Node {
            row: self.row,
            col: self.col - 1,
        })
    }
}

type PipeMaze = UnGraphMap<Node, ()>;

pub fn solve1(input: Input) -> u64 {
    let (start, maze, debug) = build_graph(input);
    let paths = dijkstra(&maze, start, None, |_| 1u64);
    debug_assert!(paths.keys().all(|n| debug.contains_key(n)));
    paths.values().copied().max().unwrap()
}

pub fn solve2(_input: Input) -> u64 {
    todo!()
}

fn north_edge(m: &HashMap<Node, Pipe>, node: Node) -> Option<Node> {
    use Pipe::*;
    node.north().and_then(|north| match m.get(&north).unwrap() {
        Start | NS | SW | SE => Some(north),
        _ => None,
    })
}

fn south_edge(m: &HashMap<Node, Pipe>, node: Node) -> Option<Node> {
    use Pipe::*;
    node.south().and_then(|south| match m.get(&south).unwrap() {
        Start | NS | NE | NW => Some(south),
        _ => None,
    })
}

fn east_edge(m: &HashMap<Node, Pipe>, node: Node) -> Option<Node> {
    use Pipe::*;
    node.east().and_then(|east| match m.get(&east).unwrap() {
        Start | NW | SW | EW => Some(east),
        _ => None,
    })
}

fn west_edge(m: &HashMap<Node, Pipe>, node: Node) -> Option<Node> {
    use Pipe::*;
    node.west().and_then(|west| match m.get(&west).unwrap() {
        Start | NE | SE | EW => Some(west),
        _ => None,
    })
}
type EdgeFn = fn(&HashMap<Node, Pipe>, Node) -> Option<Node>;

fn make_edges(m: &HashMap<Node, Pipe>, n: Node, one: EdgeFn, two: EdgeFn) -> Vec<(Node, Node)> {
    [one(m, n), two(m, n)]
        .into_iter()
        .filter_map(|n2| n2)
        .map(|n2| (n, n2))
        .collect()
}

fn build_graph(input: Input) -> (Node, PipeMaze, HashMap<Node, Pipe>) {
    let input = parse_input(input);
    let start = input
        .iter()
        .find_map(|(node, p)| match p {
            Pipe::Start => Some(*node),
            _ => None,
        })
        .unwrap();
    let node_to_pipe: HashMap<Node, Pipe> = HashMap::from_iter(input.iter().cloned());
    let maze = PipeMaze::from_edges(
        input
            .iter()
            .copied()
            .filter_map(|(node, p)| match p {
                Pipe::NS => Some(make_edges(&node_to_pipe, node, north_edge, south_edge)),
                Pipe::EW => Some(make_edges(&node_to_pipe, node, east_edge, west_edge)),
                Pipe::NE => Some(make_edges(&node_to_pipe, node, north_edge, east_edge)),
                Pipe::NW => Some(make_edges(&node_to_pipe, node, north_edge, west_edge)),
                Pipe::SW => Some(make_edges(&node_to_pipe, node, south_edge, west_edge)),
                Pipe::SE => Some(make_edges(&node_to_pipe, node, south_edge, east_edge)),
                Pipe::Ground | Pipe::Start => None,
            })
            .flatten(),
    );
    (start, maze, node_to_pipe)
}

fn parse_input(input: Input) -> Vec<(Node, Pipe)> {
    input
        .enumerate()
        .flat_map(|(row, l)| {
            l.bytes()
                .map(|b| b as char)
                .enumerate()
                .map(move |(col, c)| {
                    (
                        Node {
                            row: row as _,
                            col: col as _,
                        },
                        Pipe::from(c),
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}
