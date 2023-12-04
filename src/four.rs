use std::collections::HashSet;

#[derive(Debug)]
struct Card {
    winning_nums: HashSet<u64>,
    have_nums: HashSet<u64>,
}

impl Card {
    fn matching(&self) -> u64 {
        self.winning_nums.intersection(&self.have_nums).count() as _
    }
}

pub fn solve1(input: crate::Input) -> u64 {
    parse_cards(input)
        .map(|card| card.matching())
        .filter_map(|matching| matching.gt(&0).then(|| 1 << (matching - 1)))
        .sum()
}

pub fn solve2(input: crate::Input) -> u64 {
    let mut table = [0u64; 213];
    parse_cards(input)
        .map(|card| card.matching())
        .enumerate()
        .for_each(|(n, matching)| {
            table[n] += 1;
            let n_matches = table[n];
            table[n + 1..=n + matching as usize]
                .iter_mut()
                .for_each(|m| *m += n_matches);
        });
    table.into_iter().sum()
}

fn parse_cards(input: crate::Input) -> impl Iterator<Item = Card> {
    fn parse_line(line: &str) -> Card {
        fn parse_set(nums: &str) -> HashSet<u64> {
            nums.split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        }
        let (_number, lists) = line.split_once(": ").unwrap();
        let (winning, have) = lists.split_once(" | ").unwrap();
        Card {
            winning_nums: parse_set(winning),
            have_nums: parse_set(have),
        }
    }
    input.map(|line| parse_line(&line))
}
