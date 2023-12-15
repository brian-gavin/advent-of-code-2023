use crate::Input;

type Eq = fn((&String, &String)) -> bool;

pub fn solve1(input: Input) -> usize {
    parse_input(input)
        .into_iter()
        .map(|p| {
            let vert = reflection(&p.cols, eq).unwrap_or(0);
            let hori = reflection(&p.rows, eq).unwrap_or(0);
            dbg!(vert) + dbg!(hori) * 100
        })
        .sum()
}

fn eq((a, b): (&String, &String)) -> bool {
    a == b
}

fn eq_off_by_one((a, b): (&String, &String)) -> bool {
    let mut diff = 0;
    a.chars().zip(b.chars()).for_each(|(a, b)| {
        if a != b {
            diff += 1;
        }
    });
    diff <= 1
}

pub fn solve2(input: Input) -> usize {
    parse_input(input)
        .into_iter()
        .map(|ref p| {
            let vert = reflection(&p.cols, eq_off_by_one)
                .and_then(|i| reflection(&p.cols, eq).and_then(|j| (i != j).then_some(j)))
                .unwrap_or(0);
            let hori = reflection(&p.rows, eq_off_by_one)
                .and_then(|i| reflection(&p.rows, eq).and_then(|j| (i != j).then_some(j)))
                .unwrap_or(0);
            vert + hori * 100
        })
        .sum()
}

fn reflection(v: &[String], eq: Eq) -> Option<usize> {
    (1..v.len()).find(|i| {
        let (left, right) = v.split_at(*i);
        if left.len() <= right.len() {
            left.iter().rev().zip(right.iter()).all(eq)
        } else {
            right.iter().zip(left.iter().rev()).all(eq)
        }
    })
}

#[derive(Default, Debug)]
struct Pattern {
    rows: Vec<String>,
    cols: Vec<String>,
}

fn parse_input(input: Input) -> Vec<Pattern> {
    let builders = input.fold(vec![Pattern::default()], |mut v, l| {
        if l.is_empty() {
            v.push(Pattern::default());
            return v;
        }
        let p = v.last_mut().unwrap();
        let row = l;
        if p.cols.len() < row.len() {
            for _ in 0..(row.len() - p.cols.len()) {
                p.cols.push(String::new())
            }
        }
        row.char_indices().for_each(|(col, c)| p.cols[col].push(c));
        p.rows.push(row);
        v
    });
    builders
        .into_iter()
        .map(|b| Pattern {
            rows: b.rows,
            cols: b.cols.into_iter().map(|col| col.into()).collect(),
        })
        .collect()
}
