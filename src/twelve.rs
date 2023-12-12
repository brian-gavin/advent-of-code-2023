use crate::Input;

pub fn solve1(input: Input) -> usize {
    let input = parse_input(input);
    input
        .into_iter()
        .map(|(mut states, targets)| arrangements(&mut states, &targets))
        .sum()
}

fn combinatorial_explosion_bait(
    (states, targets): (Vec<SpringState>, Vec<u64>),
) -> (Vec<SpringState>, Vec<u64>) {
    fn explode<T: Copy>(v: Vec<T>, sep: Option<T>) -> Vec<T> {
        let mut v2 = v.clone();
        for _ in 0..5 {
            if let Some(sep) = sep {
                v2.push(sep);
            }
            v2.extend(v.clone().into_iter());
        }
        v2
    }
    (explode(states, Some(Unknown)), explode(targets, None))
}

pub fn solve2(input: Input) -> usize {
    let input = parse_input(input);
    input
        .into_iter()
        .map(combinatorial_explosion_bait)
        .map(|(mut states, targets)| arrangements(&mut states, &targets))
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

fn arrangements(states: &mut [SpringState], targets: &[u64]) -> usize {
    compute_states(states, 0, targets)
}

fn compute_states(states: &mut [SpringState], mut idx: usize, targets: &[u64]) -> usize {
    let mut valid_states = 0;
    while idx < states.len() && states[idx] != Unknown {
        idx += 1
    }
    if idx >= states.len() {
        return is_solution(states, targets)
            .then_some(1)
            .unwrap_or_default();
    }
    states[idx] = Damaged;
    valid_states += compute_states(states, idx + 1, targets);
    states[idx] = Operational;
    valid_states += compute_states(states, idx + 1, targets);
    states[idx] = Unknown;
    valid_states
}

fn is_solution(states: &[SpringState], targets: &[u64]) -> bool {
    runs_of_damaged(states) == targets
}

fn runs_of_damaged(states: &[SpringState]) -> Vec<u64> {
    states
        .split(|s| *s != Damaged)
        .map(|ss| ss.len() as u64)
        .filter(|l| l.gt(&0))
        .collect()
}

fn parse_input(input: Input) -> Vec<(Vec<SpringState>, Vec<u64>)> {
    input
        .map(|l| {
            let (states, targets) = l.split_once(" ").unwrap();
            let states = states.chars().map(SpringState::from).collect();
            let targets = targets.split(',').filter_map(|n| n.parse().ok()).collect();
            (states, targets)
        })
        .collect()
}
