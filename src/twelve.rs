use std::{cell::RefCell, collections::HashMap};

use crate::Input;
use rayon::prelude::*;

pub fn solve1(input: Input) -> usize {
    let mut input = parse_input(input);
    input
        .par_iter_mut()
        .map(|(states, targets)| arrangements(states, &targets))
        .sum()
}

fn combinatorial_explosion_bait(
    (states, targets): &(Vec<SpringState>, Vec<usize>),
) -> (Vec<SpringState>, Vec<usize>) {
    fn explode<T: Copy>(v: &Vec<T>, sep: Option<T>) -> Vec<T> {
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
        .par_iter()
        .map(|t| combinatorial_explosion_bait(t))
        .enumerate()
        .map(|(i, (mut states, targets))| (i, arrangements(&mut states, &targets)))
        .inspect(|(i, _)| println!("finished line: {}", i))
        .map(|t| t.1)
        .sum()
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

fn arrangements(states: &mut [SpringState], targets: &[usize]) -> usize {
    let unknowns = states
        .iter()
        .enumerate()
        .filter(|t| *t.1 == Unknown)
        .map(|t| t.0)
        .collect::<Vec<_>>();
    let total_damaged: usize = targets.iter().sum();
    let current_damaged = states.iter().filter(|s| **s == Damaged).count();
    // eprintln!("{:?}", targets);
    let mut solutions = 0;
    compute_states(
        &mut solutions,
        states,
        &unknowns,
        current_damaged,
        total_damaged,
        targets,
    );
    solutions
}

fn compute_states(
    solutions: &mut usize,
    states: &mut [SpringState],
    unknowns: &[usize],
    current_damaged: usize,
    target_damaged: usize,
    targets: &[usize],
) {
    // eprintln!( "{} {:?} {} {}", debug_string(states), unknowns, current_damaged, target_damaged);
    if is_solution(states, unknowns, targets) {
        *solutions += 1;
    } else {
        let Some((idx, unknowns)) = unknowns.split_first().map(|t| (*t.0, t.1)) else {
            return;
        };
        // dbg!(idx, unknowns);
        for s in candidates(unknowns, current_damaged, target_damaged) {
            // dbg!(s);
            states[idx] = s;
            let current_damaged = if s == Damaged {
                current_damaged + 1
            } else {
                current_damaged
            };
            compute_states(
                solutions,
                states,
                unknowns,
                current_damaged,
                target_damaged,
                targets,
            );
        }
    }
}

fn candidates(
    unknowns: &[usize],
    current_damaged: usize,
    target_damaged: usize,
) -> Vec<SpringState> {
    if current_damaged + unknowns.len() < target_damaged - 1 || current_damaged > target_damaged {
        vec![]
    } else if current_damaged == target_damaged {
        vec![Operational]
    } else {
        vec![Damaged, Operational]
    }
}

fn is_solution(states: &[SpringState], unknowns: &[usize], targets: &[usize]) -> bool {
    unknowns.is_empty() && eq_runs_of_damaged(states, targets)
}

fn eq_runs_of_damaged(states: &[SpringState], targets: &[usize]) -> bool {
    type Map = HashMap<Vec<SpringState>, Vec<usize>>;
    thread_local! {
        static MEMOIZE: RefCell<Map> = RefCell::new(HashMap::from([
                (vec![], vec![]),
                (vec![Unknown], vec![]),
                (vec![Damaged], vec![1]),
                (vec![Operational], vec![]),
            ]));
    }
    fn memoize_prefixes(memoize: &mut Map, states: &[SpringState]) {
        // dbg!(debug_string(states));
        let (cur, prefix) = states.split_last().unwrap();
        if !memoize.contains_key(prefix) {
            memoize_prefixes(memoize, prefix);
        }
        let prefix_run = memoize.get(prefix).unwrap();
        memoize.insert(
            states.to_vec(),
            extend_prefix_run(prefix_run, *prefix.last().unwrap(), *cur),
        );
    }
    MEMOIZE.with_borrow_mut(|memoize| {
        if !memoize.contains_key(states) {
            memoize_prefixes(memoize, states);
        }
        let run = memoize.get(states).unwrap();
        run.eq(targets)
    })
}

fn extend_prefix_run(prefix_run: &[usize], before: SpringState, cur: SpringState) -> Vec<usize> {
    // dbg!(prefix_run, before, cur);
    match (prefix_run.split_last(), before, cur) {
        (Some((n, prefix_run)), Damaged, Damaged) => {
            let mut v = prefix_run.to_vec();
            v.push(*n + 1);
            v
        }
        (_, _, Damaged) => {
            let mut v = prefix_run.to_vec();
            v.push(1);
            v
        }
        _ => prefix_run.to_vec(),
    }
    /*
    states
        .split(|s| *s != Damaged)
        .map(|ss| ss.len() as usize)
        .filter(|l| l.gt(&0))
        .collect::<Vec<_>>()
        */
}

fn parse_input(input: Input) -> Vec<(Vec<SpringState>, Vec<usize>)> {
    input
        .map(|l| {
            let (states, targets) = l.split_once(" ").unwrap();
            let states = states.chars().map(SpringState::from).collect();
            let targets = targets.split(',').filter_map(|n| n.parse().ok()).collect();
            (states, targets)
        })
        .collect()
}
