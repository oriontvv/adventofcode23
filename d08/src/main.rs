use std::collections::HashMap;

type Num = usize;
type Node = String;
type Map = HashMap<Node, (Node, Node)>;

fn parse_map(s: &str) -> Option<Map> {
    let mut map = Map::new();
    for line in s.lines() {
        let (key, values) = line.split_once(" = ")?;
        let (left, right) = values
            .strip_prefix('(')?
            .strip_suffix(')')?
            .split_once(", ")?;
        map.insert(key.to_string(), (left.to_string(), right.to_string()));
    }
    Some(map)
}

fn get_next_node(node: &Node, map: &Map, dir: char) -> Node {
    let next = map.get(node).unwrap();
    match dir {
        'L' => (*next.0).to_owned(),
        'R' => (*next.1).to_owned(),
        _ => unreachable!(),
    }
}

fn find_cycle(mut cur_node: Node, path: &str, map: &Map, is_finish: impl Fn(&Node) -> bool) -> Num {
    for (step, dir) in path.chars().cycle().enumerate() {
        cur_node = get_next_node(&cur_node, map, dir);
        if is_finish(&cur_node) {
            return step + 1; // steps starts from zero
        }
    }
    unreachable!()
}

fn gcd(a: Num, b: Num) -> Num {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}

fn lcm(a: Num, b: Num) -> Num {
    a * b / gcd(a, b)
}

fn part1(path: &str, map: &Map) -> Num {
    find_cycle("AAA".to_string(), path, &map, |node| node == "ZZZ")
}

fn part2(path: &str, map: &Map) -> Num {
    let is_finish = |node: &Node| node.ends_with('Z');
    map.keys()
        .filter(|&node| node.ends_with('A'))
        .map(|node| find_cycle(node.to_owned(), path, map, is_finish))
        .fold(1 as Num, |acc, num| lcm(acc, num))
}

fn main() {
    let input = include_str!("input");
    let (path, map) = input.split_once("\n\n").unwrap();
    let map = parse_map(map).unwrap();

    let (first, second) = (part1(path, &map), part2(path, &map));
    println!("{first} {second}");
}
