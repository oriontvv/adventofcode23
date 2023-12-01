use std::{collections::HashMap, fs::read_to_string};

fn calibration(s: &str) -> usize {
    let numbers: Vec<_> = s
        .matches(char::is_numeric)
        .map(|c| c.parse::<usize>().unwrap())
        .collect();
    10 * numbers.first().unwrap() + numbers.last().unwrap()
}

fn calibration_2(s: &str, map: &HashMap<&str, usize>) -> usize {
    let mut numbers = Vec::<usize>::new();
    for (index, c) in s.chars().enumerate() {
        if c.is_numeric() {
            numbers.push(c.to_string().parse::<usize>().unwrap());
            continue;
        }
        for (key, value) in map.iter() {
            if s[index..].starts_with(*key) {
                numbers.push(*value);
                break;
            }
        }
    }
    10 * numbers.first().unwrap() + numbers.last().unwrap()
}

fn main() {
    // 1
    // let mut total: usize = 0;
    // for line in read_to_string("input").unwrap().lines() {
    //     total += calibration(line);
    // }
    // println!("total: {total}");

    // 2
    let map = HashMap::from_iter([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let mut total: usize = 0;
    for line in read_to_string("input").unwrap().lines() {
        total += calibration_2(line, &map);
    }
    println!("total: {total}");
}
