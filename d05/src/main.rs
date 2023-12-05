use std::fs::read_to_string;

type Num = usize;
type Map = Vec<(Num, Num, Num)>;

fn parse_numbers(s: &str) -> Vec<Num> {
    s.split(' ')
        .filter_map(|num| num.trim().parse::<Num>().ok())
        .collect()
}

fn parse_map(s: &str) -> Map {
    let (_, ranges) = s.split_once("\n").unwrap();
    ranges
        .lines()
        .map(|line| {
            let range = parse_numbers(line);
            (range[0], range[1], range[2])
        })
        .collect()
}

fn parse_maps(s: &str) -> Vec<Map> {
    s.trim()
        .split("\n\n")
        .map(|map| parse_map(map.trim()))
        .collect()
}

fn process_seed(seed: Num, maps: &Vec<Map>) -> Num {
    maps.iter().fold(seed, |cur_seed, ranges| {
        for (dst_start, src_start, len) in ranges {
            if cur_seed >= *src_start && cur_seed < *src_start + len {
                return cur_seed - src_start + dst_start;
            }
        }
        cur_seed
    })
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (seeds, maps) = input.split_once("\n").unwrap();
    let seeds = parse_numbers(seeds);
    let maps = parse_maps(maps);
    // 1
    let min = seeds
        .iter()
        .map(|&seed| process_seed(seed, &maps))
        .min()
        .unwrap();
    println!("min: {min}");

    // 2
    // slowest cheating solution =(
    // let min = seeds
    //     .chunks(2)
    //     .map(|range| {
    //         (range[0]..(range[0] + range[1]))
    //             .map(|seed| process_seed(seed, &maps))
    //             .min()
    //             .unwrap()
    //     })
    //     .min()
    //     .unwrap();
    // println!("min: {min}");
}
