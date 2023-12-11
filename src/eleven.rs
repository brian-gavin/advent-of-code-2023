use std::collections::{HashMap, HashSet};

use petgraph::{algo::dijkstra, graphmap::UnGraphMap, visit::EdgeRef};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Node {
    row: usize,
    col: usize,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Empty,
    Galaxy,
}

impl From<char> for Space {
    fn from(c: char) -> Self {
        match c {
            '.' => Space::Empty,
            '#' => Space::Galaxy,
            _ => unreachable!(),
        }
    }
}

type Universe = UnGraphMap<Node, u64>;

pub fn solve1(input: crate::Input) -> u64 {
    solve(input, 1)
}

pub fn solve2(input: crate::Input) -> u64 {
    solve(input, 1_000_000)
}

pub fn solve(input: crate::Input, empty_weight: u64) -> u64 {
    let (universe, galaxies) = build_universe(input, empty_weight);
    let galaxy_src_targets = galaxy_src_targets(&galaxies);
    galaxy_src_targets
        .into_iter()
        .map(|(start, targets)| {
            let paths = dijkstra(&universe, start, None, |e| *e.weight());
            targets
                .into_iter()
                .map(|target| paths.get(&target).unwrap())
                .sum::<u64>()
        })
        .sum()
}

fn galaxy_src_targets(galaxies: &[Node]) -> HashMap<Node, Vec<Node>> {
    let pairs = galaxy_pairs(galaxies);
    let mut m = HashMap::new();
    pairs.iter().for_each(|(n1, n2)| {
        m.entry(*n1).or_insert_with(|| vec![]).push(*n2);
    });
    m
}

fn galaxy_pairs(galaxies: &[Node]) -> Vec<(Node, Node)> {
    let mut pairs: Vec<_> = galaxies
        .iter()
        .flat_map(|n1| {
            galaxies
                .iter()
                .filter_map(move |n2| (n1 != n2).then(|| (*n1.min(n2), *n1.max(n2))))
        })
        .collect();
    pairs.sort();
    pairs.dedup();
    pairs
}

fn galaxy_rows_cols(galaxies: &[Node]) -> (HashSet<usize>, HashSet<usize>) {
    galaxies.iter().map(|node| (node.row, node.col)).unzip()
}

fn build_universe(input: crate::Input, empty_weight: u64) -> (Universe, Vec<Node>) {
    let input = parse_input(input);
    let galaxies: Vec<_> = input
        .iter()
        .filter_map(|(node, space)| Space::Galaxy.eq(space).then_some(*node))
        .collect();
    let (galaxy_rows, galaxy_cols) = galaxy_rows_cols(&galaxies);
    let edge_weight = |n: Node| -> u64 {
        if galaxy_rows.contains(&n.row) && galaxy_cols.contains(&n.col) {
            1
        } else {
            empty_weight
        }
    };
    let universe = Universe::from_edges(
        input
            .into_iter()
            .map(|t| t.0)
            .map(|node| {
                [node.north(), node.south(), node.east(), node.west()]
                    .into_iter()
                    .filter_map(move |o| o.map(|n| (node, n, edge_weight(n))))
            })
            .flatten(),
    );
    (universe, galaxies)
}

fn parse_input(input: crate::Input) -> Vec<(Node, Space)> {
    input
        .enumerate()
        .flat_map(|(row, l)| {
            l.char_indices()
                .map(|(col, c)| (Node { row, col }, c.into()))
                .collect::<Vec<_>>()
        })
        .collect()
}
