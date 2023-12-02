#[derive(Default, Debug)]
struct Set {
    red: u32,
    blue: u32,
    green: u32,
}

impl Set {
    fn power(&self) -> u32 {
        self.red * self.blue * self.green
    }
}

pub fn solve1(input: crate::Input) -> usize {
    const BAG: Set = Set {
        red: 12,
        green: 13,
        blue: 14,
    };
    input
        .filter_map(|line| {
            let (n, mut game) = parse_line(&line);
            game.all(|set| set.blue <= BAG.blue && set.green <= BAG.green && set.red <= BAG.red)
                .then_some(n)
        })
        .sum()
}

fn parse_line(line: &str) -> (usize, impl Iterator<Item = Set> + '_) {
    let (n, details) = game_number_detais(line);
    let details = details.split("; ").map(|subset| {
        subset
            .split(", ")
            .fold(Set::default(), |set, s| match s.split_once(" ").unwrap() {
                (n, "blue") => Set {
                    blue: n.parse().unwrap(),
                    ..set
                },
                (n, "red") => Set {
                    red: n.parse().unwrap(),
                    ..set
                },
                (n, "green") => Set {
                    green: n.parse().unwrap(),
                    ..set
                },
                _ => unreachable!(),
            })
    });
    (n, details)
}

fn game_number_detais(line: &str) -> (usize, &str) {
    let (game, details) = line.split_once(": ").unwrap();
    let game = game
        .strip_prefix("Game ")
        .and_then(|n| n.parse::<usize>().ok())
        .unwrap();
    (game, details)
}

pub fn solve2(input: crate::Input) -> u32 {
    input
        .map(|line| {
            let (_, sets) = parse_line(&line);
            sets.fold(Set::default(), |fewest_bag, set| Set {
                red: fewest_bag.red.max(set.red),
                blue: fewest_bag.blue.max(set.blue),
                green: fewest_bag.green.max(set.green),
            })
        })
        .map(|set| set.power())
        .sum()
}
