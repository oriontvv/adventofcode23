use std::fs::read_to_string;

fn calibration(s: &str) -> Option<usize> {
    let numbers: Vec<_> = s
        .matches(char::is_numeric)
        .map(|c| c.parse::<usize>().unwrap())
        .collect();
    Some(10 * numbers.first()? + numbers.last()?)
}

fn calibration_2(s: &str, words: &Vec<&str>) -> Option<usize> {
    let mut numbers = Vec::<usize>::new();
    for (index, c) in s.chars().enumerate() {
        if c.is_numeric() {
            numbers.push(c.to_string().parse::<usize>().unwrap());
            continue;
        }
        for (word_index, word) in words.iter().enumerate() {
            if s[index..].starts_with(*word) {
                numbers.push(word_index + 1);
                break;
            }
        }
    }
    Some(10 * numbers.first()? + numbers.last()?)
}

fn main() {
    // 1
    // let mut total: usize = 0;
    // for line in read_to_string("input").unwrap().lines() {
    //     total += calibration(line).unwrap();
    // }
    // println!("total: {total}");

    // 2
    let words = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut total: usize = 0;
    for line in read_to_string("input").unwrap().lines() {
        total += calibration_2(line, &words).unwrap();
    }
    println!("total: {total}");
}
