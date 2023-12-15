use crate::Input;

pub fn solve1(input: Input) -> usize {
    parse_input(input)
        .into_iter()
        .map(|p| {
            let vert = reflection(p.cols).unwrap_or(0);
            let hori = reflection(p.rows).unwrap_or(0);
            vert + hori * 100
        })
        .sum()
}
pub fn solve2(input: Input) -> usize {
    todo!()
}

fn reflection(v: Vec<String>) -> Option<usize> {
    (1..v.len()).find(|i| {
        let (left, right) = v.split_at(*i);
        if left.len() <= right.len() {
            left.iter().rev().zip(right.iter()).all(|(a, b)| a == b)
        } else {
            right.iter().zip(left.iter().rev()).all(|(a, b)| a == b)
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
