type Num = isize;

fn parse_numbers(s: &str) -> Vec<Num> {
    s.split(' ')
        .filter_map(|num| num.trim().parse::<Num>().ok())
        .collect()
}

fn get_track(mut history: Vec<Num>, item_getter: impl Fn(&Vec<Num>) -> Num) -> Vec<Num> {
    let mut track = vec![];
    loop {
        track.push(item_getter(&history));
        history = history
            .iter()
            .zip(history[1..].iter())
            .map(|(a, b)| b - a)
            .collect();
        if history.iter().all(|&num| num == 0) {
            return track;
        }
    }
}

fn first(history: Vec<Num>) -> Num {
    let getter = |history: &Vec<Num>| *history.last().unwrap();
    let last_items = get_track(history, getter);
    last_items.iter().sum()
}

fn second(history: Vec<Num>) -> Num {
    let getter = |history: &Vec<Num>| history[0];
    let first_items = get_track(history, getter);
    // backtracking from bottom to top
    first_items.iter().rev().fold(0, |acc, num| num - acc)
}

fn main() {
    let input: Vec<Vec<Num>> = include_str!("input")
        .lines()
        .map(|line| parse_numbers(line))
        .collect();

    // let first: Num = input.into_iter().map(|history| first(history)).sum();
    // println!("{first}");
    let second: Num = input.into_iter().map(|history| second(history)).sum();
    println!("{second}");
}
