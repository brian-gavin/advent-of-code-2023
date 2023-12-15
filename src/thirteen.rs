use crate::Input;

pub fn solve1(input: Input) -> usize {
    parse_input(input).into_iter().for_each(|p| {
        dbg!(&p);
        let row_reflect = reflection(p.rows);
        let col_reflect = reflection(p.cols);
        dbg!(row_reflect, col_reflect);
    });
    0
}
pub fn solve2(input: Input) -> usize {
    todo!()
}

fn reflection(v: Vec<String>) -> Option<usize> {
    (1..v.len() / 2).find(|i| {
        let (left, right) = v.split_at(*i);
        if left.len() <= right.len() {
            left.iter().zip(right.iter().rev()).all(|(a, b)| a == b)
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
