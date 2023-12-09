use std::collections::HashSet;

fn find_numbers(s: &str) -> HashSet<usize> {
    s.split(' ')
        .filter_map(|num| num.trim().parse::<usize>().ok())
        .collect()
}
fn calc_common(s: &str) -> usize {
    let (win, actual) = s.split_once('|').unwrap();
    let win = find_numbers(win);
    let actual = find_numbers(actual);
    win.intersection(&actual).count()
}

fn calc_cards(common: usize) -> usize {
    if common > 0 {
        1 << (common - 1)
    } else {
        0
    }
}

fn main() {
    // 1
    // let mut total: usize = 0;
    // for line in include_str!("input").lines() {
    //     let common = calc_common(line);
    //     total += calc_cards(common);
    // }
    // println!("total: {total}");

    // 2
    let cards: Vec<usize> = include_str!("input").lines().map(calc_common).collect();
    // start indexing with 1
    let mut totals = vec![1; cards.len() + 1];
    totals[0] = 0;
    for (card_number, &common_cards) in cards.iter().enumerate() {
        let card_number = card_number + 1;
        for index in card_number..(card_number + common_cards) {
            totals[index + 1] += totals[card_number];
        }
    }
    println!("total: {}", totals.iter().sum::<usize>());
}
