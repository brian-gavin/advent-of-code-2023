use std::str::Chars;

pub fn solve1(input: Vec<String>) -> u64 {
    input
        .into_iter()
        .map(|line| {
            let digits = line
                .chars()
                .filter_map(|c| c.to_digit(10).map(|d| d as u64));
            combine_digits(digits)
        })
        .sum()
}

fn combine_digits(mut digits: impl Iterator<Item = u64>) -> u64 {
    let first = digits.nth(0).unwrap();
    let last = digits.last().unwrap_or(first);
    first * 10 + last
}

pub fn solve2(input: Vec<String>) -> u64 {
    input
        .into_iter()
        .map(|line| {
            let digits = parse_line(&line);
            combine_digits(digits)
        })
        .sum()
}

fn parse_line(line: &str) -> impl Iterator<Item = u64> + '_ {
    (0..line.len()).filter_map(|i| parse_digit(line[i..].chars()))
}

fn parse_digit(mut s: Chars) -> Option<u64> {
    let Some(c) = s.next() else {
        return None;
    };
    match c {
        'o' => check_suffix(s, ("ne", 1)),
        't' => check_suffix2(s, ("wo", 2), ("hree", 3)),
        'f' => check_suffix2(s, ("our", 4), ("ive", 5)),
        's' => check_suffix2(s, ("ix", 6), ("even", 7)),
        'e' => check_suffix(s, ("ight", 8)),
        'n' => check_suffix(s, ("ine", 9)),
        'z' => check_suffix(s, ("ero", 0)),
        _ => c.to_digit(10).map(|n| n as u64),
    }
}

fn check_suffix(s: Chars, (suffix, value): (&str, u64)) -> Option<u64> {
    s.take(suffix.len())
        .cmp(suffix.chars())
        .is_eq()
        .then_some(value)
}

fn check_suffix2(s: Chars, p1: (&str, u64), p2: (&str, u64)) -> Option<u64> {
    check_suffix(s.clone(), p1).or_else(|| check_suffix(s, p2))
}

#[allow(dead_code)]
fn parse_digit_slow(d: &str) -> Option<u64> {
    let p = || {
        let d = match d {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            "zero" => 0,
            _ => return None,
        };
        Some(d)
    };
    d.parse::<u64>().ok().or_else(|| p())
}

#[allow(dead_code)]
fn parse_line_slow(line: &str) -> impl Iterator<Item = u64> {
    let mut digits = vec![];
    for i in 0..line.len() {
        for j in i..line.len() {
            let cur = &line[i..=j];
            // dbg!(cur);
            if let Some(d) = parse_digit_slow(cur) {
                if d < 10 {
                    digits.push(d);
                }
            }
        }
    }
    // dbg!(line, &digits);
    digits.into_iter()
}
