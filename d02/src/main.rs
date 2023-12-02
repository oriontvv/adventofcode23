use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct Load {
    red: usize,
    green: usize,
    blue: usize,
}

impl FromStr for Load {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for pair in s.trim().split(',') {
            let (num, color) = pair.trim().split_once(' ').unwrap();
            let num: usize = num.trim().parse().expect(pair);
            match color {
                "red" => red += num,
                "green" => green += num,
                "blue" => blue += num,
                _ => return Err(()),
            }
        }
        Ok(Self { red, green, blue })
    }
}

type GameId = usize;
type IsGamePossbile = bool;
type GamePower = usize;

fn solution_1(s: &str) -> Option<(GameId, IsGamePossbile)> {
    let (id, loads) = s.split_once(':')?;
    let (_, id) = id.split_once(' ')?;
    let id: usize = id.trim().parse().ok()?;
    let is_possible = loads.split(';').all(|load_str| {
        let load: Load = load_str.parse().expect(load_str);
        load.red <= 12 && load.green <= 13 && load.blue <= 14
    });
    Some((id, is_possible))
}

fn solution_2(s: &str) -> Option<GamePower> {
    let (_, loads) = s.split_once(':')?;
    let loads: Vec<Load> = loads.split(';').filter_map(|s| s.parse().ok()).collect();
    let max_red = loads.iter().max_by(|a, b| a.red.cmp(&b.red))?.red;
    let max_green = loads.iter().max_by(|a, b| a.green.cmp(&b.green))?.green;
    let max_blue = loads.iter().max_by(|a, b| a.blue.cmp(&b.blue))?.blue;
    Some(max_red * max_green * max_blue)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_load() {
        let load: Load = " 1 red, 2 green, 6 blue ".parse().unwrap();
        assert_eq!(
            load,
            Load {
                red: 1,
                green: 2,
                blue: 6
            }
        );
    }

    #[test]
    fn test_parse_game() {
        let (n, is_possible) =
            solution_1("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue").unwrap();
        assert_eq!(n, 2);
        assert!(is_possible);
    }
}

fn main() {
    // 1
    // let mut total: usize = 0;
    // for line in read_to_string("input").unwrap().lines() {
    //     if let Some((n, true)) = solution_1(line) {
    //         total += n
    //     }
    // }
    // println!("total: {total}");

    // 2
    let mut total: usize = 0;
    for line in read_to_string("input").unwrap().lines() {
        if let Some(power) = solution_2(line) {
            total += power
        }
    }
    println!("total: {total}");
}
