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

#[derive(Debug, PartialEq, Ord, Eq)]
struct Hand(HandType, Cards, usize);

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.0.partial_cmp(&other.0) {
            Some(Ordering::Equal) => self.1.partial_cmp(&other.1),
            ord => return ord,
        }
    }
}

// 1
// #[derive(Debug, PartialEq, Ord, Eq)]
// struct Cards(String);

// impl PartialOrd for Cards {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         let values: Vec<_> = self.0.chars().map(card_value).collect();
//         let other_values: Vec<_> = other.0.chars().map(card_value).collect();
//         Some(values.cmp(&other_values))
//     }
// }
// fn card_value(card: char) -> usize {
//     "23456789TJQKA".find(card).unwrap()
// }

// impl FromStr for Hand {
//     type Err = ();
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let (cards, bet) = s.split_once(" ").unwrap();
//         Ok(Self(
//             detect_hand_type(cards),
//             Cards(cards.to_string()),
//             bet.parse().unwrap(),
//         ))
//     }
// }

// fn detect_hand_type(cards: &str) -> HandType {
//     let counter: Counter<_> = cards.chars().collect();
//     let mut counts: Vec<_> = counter.values().copied().collect();
//     counts.sort_by(|a, b| b.cmp(a));
//     match_counts(&counts)
// }

// 2
#[derive(Debug, PartialEq, Ord, Eq)]
struct Cards(String);

impl PartialOrd for Cards {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let values: Vec<_> = self.0.chars().map(card_value2).collect();
        let other_values: Vec<_> = other.0.chars().map(card_value2).collect();
        Some(values.cmp(&other_values))
    }
}
fn card_value2(card: char) -> usize {
    "J23456789TQKA".find(card).unwrap()
}

impl FromStr for Hand {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bet) = s.split_once(" ").unwrap();
        Ok(Self(
            detect_hand_type2(cards),
            Cards(cards.to_string()),
            bet.parse().unwrap(),
        ))
    }
}

fn detect_hand_type2(cards: &str) -> HandType {
    let cards_without_jokers = cards.replace("J", "");
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

fn main() {
    let mut cards: Vec<Hand> = read_to_string("input")
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    cards.sort();
    cards.reverse();
    println!("{cards:?}");

    let total: usize = cards
        .iter()
        .rev()
        .enumerate()
        .map(|(index, hand)| (index + 1) * hand.2)
        .sum();
    println!("{total}");
}
