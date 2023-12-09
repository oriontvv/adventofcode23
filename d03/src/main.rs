use regex::Regex;
use std::collections::hash_map::Entry;
use std::collections::HashSet;
use std::{cmp::min, collections::HashMap};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Number {
    y: usize, // left top position
    x: usize, // left top position
    value: usize,
    len: usize, // number of digits
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Symbol {
    y: usize,
    x: usize,
    c: char,
}

type GearsMap = HashMap<Symbol, HashSet<Number>>;

fn find_numbers(map: &Vec<&str>) -> Vec<Number> {
    let mut numbers = Vec::new();
    for (y, row) in map.iter().enumerate() {
        for (value, x, len) in find_numbers_with_positions(*row) {
            let number = Number { y, x, value, len };
            numbers.push(number);
        }
    }
    numbers
}

fn find_numbers_with_positions(s: &str) -> Vec<(usize, usize, usize)> {
    let re = Regex::new(r"(\d+)").unwrap();
    let mut numbers = Vec::new();
    for capture in re.captures_iter(s) {
        if let Ok(num) = capture[1].parse::<usize>() {
            let start_pos = capture.get(0).unwrap().start();
            numbers.push((num, start_pos, capture[1].len()));
        }
    }
    numbers
}

fn find_symbols_near_number(n: &Number, map: &Vec<&str>) -> Vec<Symbol> {
    let width = map[0].len();
    let height = map.len();

    let x_start = if n.x as i32 - 1 > 0 { n.x - 1 } else { 0 };
    let x_end = min(width, n.x + n.len + 1);

    let y_start = if n.y as i32 - 1 > 0 { n.y - 1 } else { 0 };
    let y_end = min(height, n.y + 2);

    let mut symbols = Vec::new();
    for y in y_start..y_end {
        for x in x_start..x_end {
            let c: char = map[y].chars().nth(x).unwrap();
            if !c.is_numeric() && c != '.' {
                let symbol = Symbol { y, x, c };
                symbols.push(symbol);
            }
        }
    }
    return symbols;
}

fn find_gears(map: &Vec<&str>) -> GearsMap {
    let mut gears = GearsMap::new();
    for number in find_numbers(&map) {
        for symbol in find_symbols_near_number(&number, &map) {
            if symbol.c == '*' {
                match gears.entry(symbol) {
                    Entry::Occupied(mut numbers) => {
                        numbers.get_mut().insert(number);
                    }
                    Entry::Vacant(numbers) => {
                        let mut set = HashSet::with_capacity(1);
                        set.insert(number);
                        numbers.insert(set);
                    }
                }
            }
        }
    }
    gears
}

fn main() {
    let input = include_str!("input");
    let map: Vec<&str> = input.lines().into_iter().collect();

    // 1
    // let mut total: usize = 0;
    // for number in find_numbers(&map) {
    //     let symbols = find_symbols_near_number(&number, &map);
    //     if !symbols.is_empty() {
    //         println!("{number:?}");
    //         total += number.value;
    //     }
    // }
    // println!("total: {total}");

    // 2
    let mut total: usize = 0;
    for (gear, numbers) in find_gears(&map).iter() {
        if numbers.len() == 2 {
            total += numbers.iter().fold(1, |acc, n| acc * n.value);
            println!("{gear:?}: {numbers:?}");
        }
    }
    println!("total: {total}");
}
