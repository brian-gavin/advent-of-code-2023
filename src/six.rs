use std::iter;

pub fn solve1(input: crate::Input) -> u64 {
    solve(parse_input(input))
}

pub fn solve2(input: crate::Input) -> u64 {
    solve(parse_input2(input))
}

fn solve(input: impl Iterator<Item = (u64, u64)>) -> u64 {
    input
        .map(|(race_t, record_d)| {
            (0..=race_t)
                .filter(move |charge_t| {
                    let v = *charge_t;
                    let t = race_t - *charge_t;
                    let d = t * v;
                    d.gt(&record_d)
                })
                .count() as u64
        })
        .reduce(|a, n| a * n)
        .unwrap()
}

fn parse_input(input: crate::Input) -> impl Iterator<Item = (u64, u64)> {
    let mut input = input.map(|s| {
        s.split_once(':')
            .unwrap()
            .1
            .split_ascii_whitespace()
            .filter_map(|n| n.parse::<u64>().ok())
            .collect::<Vec<_>>()
    });
    iter::zip(input.next().unwrap(), input.next().unwrap())
}

fn parse_input2(input: crate::Input) -> impl Iterator<Item = (u64, u64)> {
    let mut input = input.filter_map(|s| {
        s.split_once(':')
            .unwrap()
            .1
            .split_ascii_whitespace()
            .collect::<String>()
            .parse::<u64>()
            .ok()
    });
    [(input.next().unwrap(), input.next().unwrap())].into_iter()
}
