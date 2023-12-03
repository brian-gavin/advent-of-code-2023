use std::{iter::Peekable, ops::RangeInclusive};

#[derive(Debug, PartialEq, Eq)]
struct Coord {
    kind: CoordKind,
    x: RangeInclusive<usize>,
}

impl Coord {
    fn adjacent(&self, other: &Coord) -> bool {
        let (large, small) = match (&self.kind, &other.kind) {
            (CoordKind::Number(_), _) => (&self.x, &other.x),
            (_, CoordKind::Number(_)) => (&other.x, &self.x),
            _ => unreachable!("have to compare something to a number"),
        };
        // range of self must contain the start or end of the other x
        // ....#..
        // ..123..
        // ...*...
        if large.contains(small.start()) || large.contains(small.end()) {
            return true;
        }
        // other x must be within 1 from the start or end
        // .#....
        // ..123.
        // .....$
        large.start().abs_diff(*small.end()) <= 1 || large.end().abs_diff(*small.start()) <= 1
    }

    fn is_part(&self) -> bool {
        matches!(self.kind, CoordKind::Symbol(_))
    }

    fn is_gear_symbol(&self) -> bool {
        matches!(self.kind, CoordKind::Symbol('*'))
    }

    fn is_number(&self) -> bool {
        matches!(self.kind, CoordKind::Number(_))
    }

    fn number(&self) -> u64 {
        let CoordKind::Number(n) = self.kind else {
            panic!("not a number")
        };
        n
    }
}

#[derive(Debug, PartialEq, Eq)]
enum CoordKind {
    Nil,
    Number(u64),
    Symbol(char),
}

pub fn solve1(input: crate::Input) -> u64 {
    let coords = parse_coords(input);
    let mut adjacents = vec![];
    let check_adjacent_row = |number: &Coord, y: usize| {
        coords
            .get(y)
            .map(|row| {
                row.iter()
                    .filter(|c| c.is_part())
                    .any(|c| number.adjacent(c))
            })
            .unwrap_or_default()
    };
    for (y, xs) in coords.iter().enumerate() {
        adjacents.extend(
            xs.iter()
                .enumerate()
                .filter(|(_, c)| matches!(c.kind, CoordKind::Number(_)))
                .filter(|(i, number)| {
                    let check_neighbor = |x| xs.get(x).map(Coord::is_part).unwrap_or_default();
                    check_neighbor(i.wrapping_sub(1))
                        || check_neighbor(i.wrapping_add(1))
                        || check_adjacent_row(number, y.wrapping_sub(1))
                        || check_adjacent_row(number, y.wrapping_add(1))
                })
                .map(|(_, c)| c),
        );
    }
    adjacents
        .into_iter()
        .map(|c| match c.kind {
            CoordKind::Number(x) => x,
            _ => unreachable!(),
        })
        .inspect(|n| println!("{}", n))
        .sum()
}

pub fn solve2(input: crate::Input) -> u64 {
    let coords = parse_coords(input);
    let find_adjacent_numbers = |gear: &Coord, y: usize| {
        coords.get(y).map_or_else(
            || vec![],
            |row| {
                row.iter()
                    .filter_map(|c2| (c2.is_number() && gear.adjacent(c2)).then(|| c2.number()))
                    .collect()
            },
        )
    };
    let mut ratios = vec![];
    for (y, row) in coords.iter().enumerate() {
        ratios.extend(
            row.iter()
                .enumerate()
                .filter(|(_, c)| c.is_gear_symbol())
                .filter_map(|(i, gear)| {
                    let check_neighbor =
                        |i: usize| row.get(i).and_then(|c| c.is_number().then(|| c.number()));
                    let up = find_adjacent_numbers(gear, y.wrapping_sub(1));
                    let down = find_adjacent_numbers(gear, y + 1);
                    let left = check_neighbor(i.wrapping_sub(1));
                    let right = check_neighbor(i + 1);
                    let all = up
                        .into_iter()
                        .chain(down.into_iter())
                        .chain(left.into_iter())
                        .chain(right.into_iter())
                        .collect::<Vec<_>>();
                    if all.len() != 2 {
                        None
                    } else {
                        Some(all[0] * all[1])
                    }
                }),
        )
    }
    ratios.into_iter().sum()
}

fn parse_coords(input: crate::Input) -> Vec<Vec<Coord>> {
    let mut vv = vec![];
    for (_, line) in input.enumerate() {
        let mut v = vec![];
        let mut chars = line.chars().enumerate().peekable();
        while let Some((x, c)) = chars.next() {
            let coord = match c {
                '.' => Coord {
                    kind: CoordKind::Nil,
                    x: x..=x,
                },
                c if c.is_ascii_punctuation() => Coord {
                    kind: CoordKind::Symbol(c),
                    x: x..=x,
                },
                c if c.is_ascii_digit() => consume_number(&mut chars, x, &line),
                _ => unreachable!(),
            };
            v.push(coord);
        }
        vv.push(v);
    }
    vv
}

fn consume_number(
    chars: &mut Peekable<impl Iterator<Item = (usize, char)>>,
    start: usize,
    line: &str,
) -> Coord {
    let parse_number = |end| CoordKind::Number(line[start..=end].parse::<u64>().unwrap());
    match chars.peek() {
        None => {
            let end = line.len() - 1;
            return Coord {
                kind: parse_number(line.len() - 1),
                x: start..=end,
            };
        }
        Some((x, c)) if !c.is_ascii_digit() => {
            let end = *x - 1;
            return Coord {
                kind: parse_number(end),
                x: start..=end,
            };
        }
        Some((_, c)) if c.is_ascii_digit() => {
            let _ = chars.next();
            consume_number(chars, start, line)
        }
        _ => unreachable!(),
    }
}
