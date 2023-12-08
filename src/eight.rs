use num::integer::lcm;

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

type Map = std::collections::HashMap<String, Node>;

pub fn solve1(input: crate::Input) -> u64 {
    let (directions, map) = parse_input(input);
    search(
        map.get_key_value("AAA").unwrap(),
        &map,
        directions.chars().cycle(),
    )
}

fn search<'a>(
    mut cur: (&'a String, &'a Node),
    map: &'a Map,
    mut directions: impl Iterator<Item = char>,
) -> u64 {
    let mut steps = 0;
    while !cur.0.ends_with('Z') {
        let d = directions.next().unwrap();
        cur = match d {
            'L' => map.get_key_value(&cur.1.left).unwrap(),
            'R' => map.get_key_value(&cur.1.right).unwrap(),
            _ => unreachable!(),
        };
        steps += 1;
    }
    steps
}

pub fn solve2(input: crate::Input) -> u64 {
    let (directions, map) = parse_input(input);
    let nodes = map
        .iter()
        .filter(|(k, _)| k.ends_with('A'))
        .collect::<Vec<_>>();
    multi_search(nodes, &map, &directions)
}

fn multi_search<'a>(nodes: Vec<(&'a String, &'a Node)>, map: &'a Map, directions: &str) -> u64 {
    nodes
        .into_iter()
        .map(|n| search(n, map, directions.chars().cycle()))
        .reduce(lcm)
        .unwrap()
}

fn parse_input(mut input: crate::Input) -> (String, Map) {
    fn parse_line(l: String) -> (String, Node) {
        let (node, lr) = l.split_once(" = ").unwrap();
        let connections = lr
            .trim_matches('(')
            .trim_matches(')')
            .split_once(", ")
            .unwrap();
        (
            node.to_string(),
            Node {
                left: connections.0.to_string(),
                right: connections.1.to_string(),
            },
        )
    }
    let directions = input.next().unwrap();
    let _ = input.next();
    let map = Map::from_iter(input.map(parse_line));
    (directions, map)
}
