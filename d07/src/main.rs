use counter::Counter;
use std::cmp::Ordering;
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

fn match_counts(counts: &Vec<usize>) -> HandType {
    match counts {
        _ if counts[0] == 5 => HandType::FiveKind,
        _ if counts[0] == 4 => HandType::FourKind,
        _ if counts[0] == 3 && counts.len() > 1 && counts[1] == 2 => HandType::FullHouse,
        _ if counts[0] == 3 => HandType::ThreeKind,
        _ if counts[0] == 2 && counts.len() > 1 && counts[1] == 2 => HandType::TwoPair,
        _ if counts[0] == 2 => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

type Bet = usize;

#[derive(Debug, Clone, PartialEq, Ord, Eq)]
struct Cards(String);

#[derive(Debug, PartialEq, Ord, Eq)]
struct Hand(HandType, Cards, Bet);

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.0.partial_cmp(&other.0) {
            Some(Ordering::Equal) => self.1.partial_cmp(&other.1),
            ord => return ord,
        }
    }
}

impl FromStr for Hand {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bet) = s.split_once(" ").unwrap();
        let cards = Cards(cards.to_string());
        Ok(Self(cards.clone().into(), cards, bet.parse().unwrap()))
    }
}

// 1
// impl PartialOrd for Cards {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         let index = |card: char| "23456789TJQKA".find(card).unwrap();
//         let values = self.0.chars().map(index);
//         let other_values = other.0.chars().map(index);
//         Some(values.cmp(other_values))
//     }
// }

// impl From<Cards> for HandType {
//     fn from(value: Cards) -> Self {
//         let counter: Counter<_> = value.0.chars().collect();
//         let mut counts: Vec<_> = counter.values().copied().collect();
//         counts.sort_by(|a, b| b.cmp(a));
//         match_counts(&counts)
//     }
// }

// 2
impl PartialOrd for Cards {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let index = |card: char| "J23456789TQKA".find(card).unwrap();
        let values = self.0.chars().map(index);
        let other_values = other.0.chars().map(index);
        Some(values.cmp(other_values))
    }
}

impl From<Cards> for HandType {
    fn from(value: Cards) -> Self {
        let cards_without_jokers = value.0.replace("J", "");
        let non_jokers = cards_without_jokers.len();
        if non_jokers == 0 {
            return HandType::FiveKind;
        }
        let counter: Counter<_> = cards_without_jokers.chars().collect();
        let mut counts: Vec<_> = counter.values().copied().collect();
        counts.sort_by(|a, b| b.cmp(a));
        // add jokers to first combination
        counts[0] += 5 - non_jokers;
        match_counts(&counts)
    }
}

fn main() {
    let mut cards: Vec<Hand> = read_to_string("input")
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    cards.sort();
    println!("{cards:?}");

    let total: usize = cards
        .iter()
        .enumerate()
        .map(|(index, hand)| (index + 1) * hand.2)
        .sum();
    println!("{total}");
}
