#[derive(Clone, Copy)]
enum NextValue {
    First,
    Last,
}

pub fn solve1(input: crate::Input) -> i64 {
    solve(input, NextValue::Last)
}

pub fn solve2(input: crate::Input) -> i64 {
    solve(input, NextValue::First)
}

fn solve(input: crate::Input, nv: NextValue) -> i64 {
    parse_input(input).map(|v| predict_next_value(v, nv)).sum()
}

fn predict_next_value(v: Vec<i64>, nv: NextValue) -> i64 {
    let mut vals = vec![];
    let mut prev = v;
    loop {
        let (diffs, val) = compute_diffs(prev, nv);
        vals.push(val);
        if diffs.iter().all(|n| *n == 0) {
            break;
        }
        prev = diffs;
    }
    extrapolate(vals.into_iter().rev(), nv)
}

fn compute_diffs(v: Vec<i64>, nv: NextValue) -> (Vec<i64>, i64) {
    let (first, v) = v.split_first().unwrap();
    let (diffs, last) = v.iter().cloned().fold(
        (Vec::with_capacity(v.len()), *first),
        |(mut diffs, a), b| {
            diffs.push(b - a);
            (diffs, b)
        },
    );
    let n = match nv {
        NextValue::First => *first,
        NextValue::Last => last,
    };
    (diffs, n)
}

fn extrapolate(vs: impl Iterator<Item = i64>, nv: NextValue) -> i64 {
    let op = match nv {
        NextValue::First => std::ops::Sub::<i64>::sub,
        NextValue::Last => std::ops::Add::<i64>::add,
    };
    [0].into_iter().chain(vs).reduce(|a, b| op(b, a)).unwrap()
}

fn parse_input(input: crate::Input) -> impl Iterator<Item = Vec<i64>> {
    input.map(|line| {
        line.split_ascii_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect::<Vec<_>>()
    })
}
