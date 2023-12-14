use std::collections::HashMap;

use crate::Input;

pub fn solve1(input: Input) -> usize {
    solve(parse_input(input))
}

fn combinatorial_explosion_bait(
    (states, targets): (Vec<SpringState>, Vec<usize>),
) -> (Vec<SpringState>, Vec<usize>) {
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

fn clone_append<T: Clone>(v: &Vec<T>, t: T) -> Vec<T> {
    let mut v = v.clone();
    v.push(t);
    v
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
        let Some((cur, states)) = states.split_first() else {
            return if targets.is_empty() { 1 } else { 0 };
        };
        let Some((target_run, rest_targets)) = targets.split_first().map(|t| (*t.0, t.1)) else {
            return if *cur != Damaged && states.iter().find(|s| **s == Damaged).is_none() {
                1
            } else {
                0
            };
        };
        fn damaged<'a>(
            memoize: &mut Map<'a>,
            states: &'a [SpringState],
            target_run: usize,
            rest_targets: &'a [usize],
        ) -> usize {
            if target_run >= states.len() {
                return 0;
            }
            if states.iter().take(target_run).any(|s| *s == Operational) {
                return 0;
            }
            if states[target_run] == Damaged {
                return 0;
            }
            arrangements(memoize, &states[target_run..], rest_targets)
        }
        match cur {
            Damaged => damaged(memoize, states, target_run - 1, rest_targets),
            Operational => arrangements(memoize, states, targets),
            Unknown => {
                damaged(memoize, states, target_run - 1, rest_targets)
                    + arrangements(memoize, states, targets)
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
    eprintln!("{} {:?} = {}", debug_string(states), targets, x);
    x
}

fn parse_input(input: Input) -> impl Iterator<Item = (Vec<SpringState>, Vec<usize>)> {
    input.map(|l| {
        let (states, targets) = l.split_once(" ").unwrap();
        let states = states.chars().map(SpringState::from).collect();
        let targets = targets.split(',').filter_map(|n| n.parse().ok()).collect();
        (states, targets)
    })
}
