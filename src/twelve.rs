use std::collections::HashMap;

use crate::Input;

pub fn solve1(input: Input) -> usize {
    solve(parse_input(input))
}

fn combinatorial_explosion_bait(
    (states, ref targets): (Vec<SpringState>, Vec<usize>),
) -> (Vec<SpringState>, Vec<usize>) {
    let states = states.as_slice();
    let states = [states, states, states, states, states].join(&Unknown);
    let targets = [targets, targets, targets, targets, targets]
        .into_iter()
        .flatten()
        .copied()
        .collect::<Vec<_>>();
    (states, targets)
}

pub fn solve2(input: Input) -> usize {
    solve(
        parse_input(input)
            .into_iter()
            .map(|t| combinatorial_explosion_bait(t)),
    )
}

fn solve(input: impl Iterator<Item = (Vec<SpringState>, Vec<usize>)>) -> usize {
    let mut memoize = Map::new();
    let input = input.collect::<Vec<_>>();
    let mut sum = 0;
    input
        .iter()
        .for_each(|(states, targets)| sum += arrangements(&mut memoize, states, targets));
    sum
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

use SpringState::*;

impl From<char> for SpringState {
    fn from(c: char) -> Self {
        match c {
            '.' => Operational,
            '#' => Damaged,
            '?' => Unknown,
            _ => unreachable!(),
        }
    }
}

#[allow(dead_code)]
fn debug_string(states: &[SpringState]) -> String {
    states.iter().fold(String::new(), |mut s, ss| {
        match ss {
            Operational => s.push('.'),
            Damaged => s.push('#'),
            Unknown => s.push('?'),
        }
        s
    })
}

type K<'a> = (&'a [SpringState], &'a [usize]);
type Map<'a> = HashMap<K<'a>, usize>;

fn arrangements<'a>(
    memoize: &mut Map<'a>,
    states: &'a [SpringState],
    targets: &'a [usize],
) -> usize {
    fn f<'a>(memoize: &mut Map<'a>, states: &'a [SpringState], targets: &'a [usize]) -> usize {
        if (states.len() as isize)
            < (targets.len() as isize - 1 + targets.iter().sum::<usize>() as isize)
        {
            return 0;
        }
        if states.is_empty() {
            return if targets.is_empty() { 1 } else { 0 };
        }
        if targets.is_empty() {
            return if states.iter().any(|s| *s == Damaged) {
                0
            } else {
                1
            };
        }
        match states[0] {
            Damaged => damaged(memoize, states, targets.split_first().unwrap()),
            Operational => arrangements(memoize, &states[1..], targets),
            Unknown => {
                damaged(memoize, states, targets.split_first().unwrap())
                    + arrangements(memoize, &states[1..], targets)
            }
        }
    }
    let k = &(states, targets);
    let x = if memoize.contains_key(k) {
        *memoize.get(k).unwrap()
    } else {
        let v = f(memoize, states, targets);
        memoize.insert(*k, v);
        *memoize.get(&k).unwrap()
    };
    x
}

fn damaged<'a>(
    memoize: &mut Map<'a>,
    states: &'a [SpringState],
    (target_run, rest_targets): (&usize, &'a [usize]),
) -> usize {
    let (run, states) = states.split_at(*target_run);
    if run.iter().any(|s| *s == Operational) {
        return 0;
    }
    match states.split_first() {
        Some((&Damaged, _)) => 0,
        Some((_, states)) => arrangements(memoize, states, rest_targets),
        None => arrangements(memoize, states, rest_targets),
    }
}

fn parse_input(input: Input) -> impl Iterator<Item = (Vec<SpringState>, Vec<usize>)> {
    input.map(|l| {
        let (states, targets) = l.split_once(" ").unwrap();
        let states = states.chars().map(SpringState::from).collect();
        let targets = targets.split(',').filter_map(|n| n.parse().ok()).collect();
        (states, targets)
    })
}
