#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandStrength {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
enum JokerCount {
    Zero,
    One,
    Two,
    Three,
    Four,
}

impl From<usize> for JokerCount {
    fn from(value: usize) -> Self {
        use JokerCount::*;
        match value {
            0 => Zero,
            1 => One,
            2 => Two,
            3 => Three,
            4 => Four,
            _ => unreachable!(),
        }
    }
}

impl HandStrength {
    fn add_jokers(self, jokers: usize) -> HandStrength {
        use HandStrength::*;
        use JokerCount::*;
        let jokers = JokerCount::from(jokers);
        match (jokers, self) {
            (Zero, s) => s,
            (One, HighCard) => OnePair,
            (One, OnePair) => ThreeOfAKind,
            (One, TwoPair) => FullHouse,
            (One, ThreeOfAKind) => FourOfAKind,
            (One, FourOfAKind) => FiveOfAKind,
            (Two, HighCard) => ThreeOfAKind,
            (Two, OnePair) => FourOfAKind,
            (Two, ThreeOfAKind) => FiveOfAKind,
            (Three, HighCard) => FourOfAKind,
            (Three, OnePair) => FiveOfAKind,
            (Four, HighCard) => FiveOfAKind,
            (n, s) => panic!("{:?} + {:?} is too many cards", n, s),
        }
    }
}

trait Card: Ord + Clone + From<char> {
    fn hand_strength(cards: Vec<Self>) -> HandStrength;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand<C: Card> {
    strength: HandStrength,
    cards: Vec<C>,
    bid: u64,
}

impl<C: Card> Hand<C> {
    fn of(cards: Vec<C>, bid: u64) -> Hand<C> {
        Hand {
            strength: C::hand_strength(cards.clone()),
            cards,
            bid,
        }
    }
}

pub fn solve1(input: crate::Input) -> u64 {
    solve::<CardOne>(input)
}

pub fn solve2(input: crate::Input) -> u64 {
    solve::<CardTwo>(input)
}

fn solve<C: Card>(input: crate::Input) -> u64 {
    let mut hands = parse_input::<C>(input)
        .map(|(cards, bid)| Hand::of(cards, bid))
        .collect::<Vec<_>>();
    hands.sort();
    hands
        .into_iter()
        .enumerate()
        .map(|(rank, hand)| (rank as u64 + 1) * hand.bid)
        .sum()
}

fn parse_input<C: Card>(input: crate::Input) -> impl Iterator<Item = (Vec<C>, u64)> {
    input.map(|line| {
        let (cards, bid) = line.split_once(" ").unwrap();
        let cards = cards.chars().map(|c| C::from(c)).collect();
        let bid = bid.parse().unwrap();
        (cards, bid)
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct CardOne(base::Card);

impl From<char> for CardOne {
    fn from(value: char) -> Self {
        Self(base::Card::from(value))
    }
}

impl Card for CardOne {
    fn hand_strength(mut cards: Vec<CardOne>) -> HandStrength {
        use HandStrength::*;
        cards.sort();
        let (first, hand) = cards.split_first().unwrap();
        let (_, mut runs, _) =
            hand.iter()
                .fold((first, [1, 0, 0, 0, 0], 0), |(prev, mut runs, idx), cur| {
                    let idx = if prev == cur { idx } else { idx + 1 };
                    runs[idx] += 1;
                    (cur, runs, idx)
                });
        debug_assert_eq!(runs.iter().sum::<u8>(), 5);
        runs.sort();
        match runs {
            [.., 1] => HighCard,
            [.., 1, 2] => OnePair,
            [.., 2, 2] => TwoPair,
            [.., 1, 3] => ThreeOfAKind,
            [.., 2, 3] => FullHouse,
            [.., 4] => FourOfAKind,
            [.., 5] => FiveOfAKind,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct CardTwo(base::Card);

impl From<char> for CardTwo {
    fn from(value: char) -> Self {
        use base::Card::{Jack, Joker};
        let c = base::Card::from(value);
        Self(if c == Jack { Joker } else { c })
    }
}

impl Card for CardTwo {
    fn hand_strength(mut cards: Vec<CardTwo>) -> HandStrength {
        use base::Card::*;
        use HandStrength::*;
        cards.sort();
        let (jokers, cards) = cards.split_at(cards.partition_point(|card| card.0 == Joker));
        if jokers.len() == 5 {
            return FiveOfAKind;
        }
        let (first, cards) = cards.split_first().unwrap();
        let (_, mut runs, _) =
            cards
                .iter()
                .fold((first, [1, 0, 0, 0, 0], 0), |(prev, mut runs, idx), cur| {
                    let idx = if prev == cur { idx } else { idx + 1 };
                    runs[idx] += 1;
                    (cur, runs, idx)
                });
        debug_assert_eq!(jokers.len() as u8 + runs.iter().sum::<u8>(), 5);
        runs.sort();
        let strength = match runs.as_slice() {
            [.., 1] => HighCard,
            [.., 2, 2] => TwoPair,
            [.., _, 2] => OnePair,
            [.., 2, 3] => FullHouse,
            [.., _, 3] => ThreeOfAKind,
            [.., 4] => FourOfAKind,
            [.., 5] => FiveOfAKind,
            a => unreachable!("{:?}", a),
        };
        strength.add_jokers(jokers.len())
    }
}

mod base {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Card {
        Joker,
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Ten,
        Jack,
        Queen,
        King,
        Ace,
    }

    impl From<char> for Card {
        fn from(value: char) -> Self {
            use Card::*;
            match value {
                '2' => Two,
                '3' => Three,
                '4' => Four,
                '5' => Five,
                '6' => Six,
                '7' => Seven,
                '8' => Eight,
                '9' => Nine,
                'T' => Ten,
                'J' => Jack,
                'Q' => Queen,
                'K' => King,
                'A' => Ace,
                c @ _ => unreachable!("{}", c),
            }
        }
    }
}
