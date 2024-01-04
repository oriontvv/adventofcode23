use std::collections::{HashMap, HashSet};

type Map = Vec<String>;
type Num = isize;
type Pos = (Num, Num);

const DOWN: Pos = (1, 0);
const UP: Pos = (-1, 0);
const LEFT: Pos = (0, -1);
const RIGHT: Pos = (0, 1);

fn find_loop(
    map: &Map,
    start: Pos,
    mut dir: Pos,
    next: &HashMap<char, Vec<Pos>>,
) -> Option<(Num, HashSet<Pos>)> {
    let width = map.len() as isize;
    let height = map[0].len() as isize;

    let mut visited: HashSet<Pos> = HashSet::new();
    let mut distance = 0;
    let (mut x, mut y) = start;
    let (mut dx, mut dy);

    while distance == 0 || (x, y) != start {
        visited.insert((x, y));
        (dx, dy) = dir;
        x += dx;
        y += dy;

        if x < 0 || x >= width || y < 0 || y >= height {
            return None;
        }

        // move next and keep backtrack
        dx *= -1;
        dy *= -1;
        let ch = map[x as usize].chars().nth(y as usize).unwrap();
        let next_dirs = next.get(&ch).unwrap();
        if !next_dirs.contains(&(dx, dy)) {
            // wrong loop
            return None;
        }

        for &next_dir in next_dirs {
            if next_dir != (dx, dy) {
                // other one is the correct next direction
                dir = next_dir;
            }
        }
        distance += 1;
    }

    Some((distance, visited))
}

fn find_start(map: &Map) -> Option<Pos> {
    for (x, row) in map.iter().enumerate() {
        if let Some(y) = row.find('S') {
            return Some((x as Num, y as Num));
        }
    }
    None
}

fn solve(map: &Map) -> (Num, Num) {
    let mut next: HashMap<char, Vec<Pos>> = HashMap::from([
        ('.', vec![]),
        ('S', vec![UP, DOWN, RIGHT, LEFT]),
        ('|', vec![UP, DOWN]),
        ('-', vec![LEFT, RIGHT]),
        ('L', vec![UP, RIGHT]),
        ('J', vec![UP, LEFT]),
        ('7', vec![DOWN, LEFT]),
        ('F', vec![DOWN, RIGHT]),
    ]);

    let start = find_start(map).expect("No start found");

    let mut updated_start_dirs = vec![];
    let mut first = 0;
    let mut visited = HashSet::new();

    for dir in [UP, DOWN, LEFT, RIGHT] {
        if let Some((distance, _visited)) = find_loop(map, start, dir, &next) {
            first = distance / 2;
            updated_start_dirs.push(dir);
            visited = _visited;
        }
    }

    next.insert('S', updated_start_dirs);

    // 2
    // highly inspired by reddit discussions and raycast algorithm
    // https://en.wikipedia.org/wiki/Point_in_polygon#Ray_casting_algorithm
    let mut second = 0;
    let mut cur_hand: Option<Pos> = None;
    for (x, row) in map.iter().enumerate() {
        let mut inside = false;
        for (y, ch) in row.chars().enumerate() {
            let pos = (x as Num, y as Num);
            if visited.contains(&pos) {
                let next_dirs = next.get(&ch).unwrap();
                if next_dirs.contains(&LEFT) && next_dirs.contains(&RIGHT) {
                    continue;
                }
                inside = !inside;
                if next_dirs.contains(&UP) && next_dirs.contains(&DOWN) {
                    continue;
                }
                let next_hand = find_up_or_down(&next_dirs).unwrap();
                cur_hand = match cur_hand {
                    Some(hand) => {
                        if next_hand != hand {
                            inside = !inside;
                        }
                        None
                    }
                    None => Some(next_hand),
                }
            } else {
                if inside {
                    second += 1;
                }
            }
        }
    }

    (first, second)
}

fn find_up_or_down(next_positions: &Vec<Pos>) -> Option<Pos> {
    for &dir in next_positions {
        if dir != LEFT && dir != RIGHT {
            return Some(dir);
        }
    }
    None
}

fn main() {
    let map: Map = include_str!("input")
        .lines()
        .map(|line| line.to_string())
        .collect();
    let (first, second) = solve(&map);
    println!("{first}, {second}");
}
