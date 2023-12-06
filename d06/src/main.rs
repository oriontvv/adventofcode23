use std::fs::read_to_string;

type Num = usize;

fn parse_numbers(s: &str, split_char: char) -> Vec<Num> {
    s.split(split_char)
        .filter_map(|num| num.trim().parse::<Num>().ok())
        .collect()
}

fn number_of_ways(time: Num, distance: Num) -> Num {
    (1..=time)
        .map(|speed| (time - speed) * speed)
        .filter(|&dst| dst > distance)
        .count()
}

fn main() {
    // 1
    let input = read_to_string("input").unwrap();
    let (times, distances) = input.split_once("\n").unwrap();
    let times = parse_numbers(times, ' ');
    let distances = parse_numbers(distances, ' ');
    let answer: Num = times
        .iter()
        .zip(distances.iter())
        .map(|(&time, &distance)| number_of_ways(time, distance))
        .product();
    println!("answer: {answer}");

    // 2
    let input = read_to_string("input").unwrap().replace(" ", "");
    let (times, distances) = input.split_once("\n").unwrap();
    let times = parse_numbers(times, ':');
    let distances = parse_numbers(distances, ':');

    let answer = number_of_ways(times[0], distances[0]);
    println!("answer: {answer}");
}
