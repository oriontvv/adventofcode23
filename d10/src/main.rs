use std::collections::VecDeque;

type Num = usize;
type SignedNum = isize;
type Map = Vec<String>;
type Pos = (Num, Num);

#[derive(Debug, Clone, Default, PartialEq, Eq)]
enum PrevPos {
    #[default]
    Uninited,
    Start,
    Pos(Num, Num),
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Node {
    prev: PrevPos,
    color: Num,
}

fn bfs_floodfill(map: &Map) -> usize {
    let start_pos = find_start(map).expect("No start found");
    let mut queue = VecDeque::new();
    queue.push_back(start_pos);

    let height = map.len();
    let width = map[0].len();
    let mut nodes = vec![vec![Node::default(); width]; height];
    nodes[start_pos.0][start_pos.1] = Node {
        prev: PrevPos::Start,
        color: 1,
    };

    while !queue.is_empty() {
        let (y, x) = queue.pop_front().unwrap();
        let node = &nodes[y][x].clone();

        for (next_y, next_x) in find_neighbours(x, y, map) {
            let next_node = &nodes[next_y][next_x];
            if node.prev == PrevPos::Pos(next_y, next_x) {
                // we've arrived here from this position, don't need to check it again
                continue;
            }
            if next_node.color == 0 {
                // unvisited
                nodes[next_y][next_x] = Node {
                    prev: PrevPos::Pos(y, x),
                    color: node.color + 1,
                };
                queue.push_back((next_y, next_x));
            } else {
                // finish. come to visited node - cycle is done
                return node.color;
            }
        }
    }
    unreachable!()
}

fn find_start(map: &Map) -> Option<Pos> {
    for (y, row) in map.iter().enumerate() {
        if let Some(x) = row.find('S') {
            return Some((y as Num, x as Num));
        }
    }
    None
}

fn find_neighbours(x: Num, y: Num, map: &Map) -> Vec<Pos> {
    let height = map.len();
    let width = map[0].len();

    let mut neighbours = vec![];
    for (dx, dy) in [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)] {
        let (next_x, next_y) = (x as SignedNum + dx, y as SignedNum + dy);
        if let (Some(next_x), Some(next_y)) =
            (between(next_x, 0, width), between(next_y, 0, height))
        {
            if is_connected(map, x, y, dx, dy) && is_connected(map, next_x, next_y, -dx, -dy) {
                neighbours.push((next_y, next_x));
            }
        }
    }
    neighbours
}

fn between(value: SignedNum, lower: Num, upper: Num) -> Option<Num> {
    if value >= (lower as SignedNum) && value < (upper as SignedNum) {
        Some(value as Num)
    } else {
        None
    }
}
fn is_connected(map: &Map, x: Num, y: Num, dx: SignedNum, dy: SignedNum) -> bool {
    let char = map[y].chars().nth(x).unwrap();
    match (char, dy, dx) {
        ('.', _, _) => false,
        ('S', _, _) => true,
        ('|', -1, _) => true,
        ('|', 1, _) => true,
        ('-', _, -1) => true,
        ('-', _, 1) => true,
        ('L', -1, 0) => true,
        ('L', 0, 1) => true,
        ('J', -1, 0) => true,
        ('J', 0, -1) => true,
        ('7', 1, 0) => true,
        ('7', 0, -1) => true,
        ('F', 1, 0) => true,
        ('F', 0, 1) => true,
        _ => false,
    }
}

fn main() {
    let map: Map = include_str!("input")
        .lines()
        .map(|line| line.to_string())
        .collect();

    let first: Num = bfs_floodfill(&map);
    println!("{first}");
}
