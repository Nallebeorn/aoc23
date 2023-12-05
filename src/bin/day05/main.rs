use std::{env, time::Instant};

const DAY: u32 = 5;

#[derive(Debug)]
struct RangeMapping {
    src_start: i64,
    src_end_exclusive: i64,
    src_to_dest_offset: i64,
}

type Map = Vec<RangeMapping>;

fn parse_input(input: &str) -> (Vec<u64>, Vec<Map>) {
    let mut lines = input.lines();
    let seeds = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    let mut maps = Vec::new();

    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        
        if line.ends_with(':') {
            maps.push(Vec::new());
        } else {
            let mut numbers = line.split_ascii_whitespace().map(|n| n.parse().unwrap());
            let dest_start = numbers.next().unwrap();
            let src_start = numbers.next().unwrap();
            let range_len = numbers.next().unwrap();

            let mapping = RangeMapping {
                src_start: src_start,
                src_end_exclusive: src_start + range_len,
                src_to_dest_offset: dest_start - src_start,
            };
            maps.last_mut().unwrap().push(mapping);
        }
    }

    return (seeds, maps);
}

fn trace_seed_to_location(seed: u64, maps: &Vec<Map>) -> i64 {
    let mut n = seed as i64;
    for map in maps {
        for range in map {
            if n >= range.src_start && n < range.src_end_exclusive {
                n = n + range.src_to_dest_offset;
                break;
            }
        }
    }

    n
}

fn solve_a(input: &str) -> i64 {
    let (seeds, maps) = parse_input(input);
    seeds.iter().map(|seed| trace_seed_to_location(*seed, &maps)).min().unwrap()
}

fn solve_b(input: &str) -> u64 {
    0
}

fn main() {
    let args: Vec<String> = env::args().skip(1).map(|arg| arg.to_lowercase()).collect();

    if args.is_empty() || args.iter().any(|arg| arg == "a") {
        println!("**** DECEMBER {} (a) ****", DAY);

        let timer = Instant::now();
        let result = solve_a(include_str!("./input.txt"));
        let elapsed = timer.elapsed();

        println!("{}", result);
        println!("({:?})\n", elapsed);
    }

    if args.is_empty() || args.iter().any(|arg| arg == "b") {
        println!("**** DECEMBER {} (b) ****", DAY);

        let timer = Instant::now();
        let result = solve_b(include_str!("./input.txt"));
        let elapsed = timer.elapsed();

        println!("{}", result);
        println!("({:?})\n", elapsed);
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn example_a() {
        let result = solve_a(include_str!("./example.txt"));
        assert_eq!(result, 35);
    }

    // #[test]
    // fn example_b() {
    //     let result = solve_b(include_str!("./example.txt"));
    //     assert_eq!(result, 30);
    // }
}
