use itertools::Itertools;
use std::collections::HashSet;

type Num = usize;
type Nums = HashSet<Num>;
type Map = Vec<String>;
type Position = (Num, Num);
type Positions = Vec<Position>;

fn find_galaxies(map: &Map, gap: Num) -> Positions {
    let big_rows = find_big_rows(map);
    let big_columns = find_big_columns(map);
    let mut shift_x;
    let mut shift_y = 0;

    let mut galaxies = vec![];
    for (y, row) in map.iter().enumerate() {
        if big_rows.contains(&y) {
            shift_y += gap;
        }
        shift_x = 0;
        for (x, char) in row.chars().enumerate() {
            if big_columns.contains(&x) {
                shift_x += gap;
            }
            if char == '#' {
                galaxies.push((y + shift_y, x + shift_x));
            }
        }
    }

    galaxies
}

fn find_big_rows(map: &Map) -> Nums {
    map.iter()
        .enumerate()
        .filter(|(_index, &ref row)| row.chars().all(|c| c == '.')) // wtf, rustc?
        .map(|(index, _)| index)
        .collect()
}

fn find_big_columns(map: &Map) -> Nums {
    let height = map.len();
    let width = map[0].len();
    (0..width)
        .filter(|&x| (0..height).all(|y| map[y].chars().nth(x).unwrap() == '.'))
        .collect()
}

fn distance(p1: &Position, p2: &Position) -> Num {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
}

fn solve(galaxies: &Positions) -> Num {
    galaxies
        .iter()
        .combinations(2)
        .map(|comb| distance(comb[0], comb[1]))
        .sum()
}

fn main() {
    let map: Map = include_str!("input")
        .lines()
        .map(|line| line.to_string())
        .collect();

    let first: Num = solve(&find_galaxies(&map, 1));
    println!("{first}");

    let second: Num = solve(&find_galaxies(&map, 999999));
    println!("{second}");
}
