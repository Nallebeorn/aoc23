use std::{collections::HashMap, env, time::Instant};
use num::integer::lcm;

const DAY: u32 = 8;

fn solve_a(input: &str) -> usize {
    let mut lines = input.lines();
    let mut directions = lines.next().unwrap().bytes().cycle();

    let nodes_input = lines.skip(1);
    let mut nodes = HashMap::new();
    for node in nodes_input {
        let mut words = node.split_ascii_whitespace();
        let name = words.next().unwrap();
        let left = words
            .nth(1)
            .unwrap()
            .trim_start_matches('(')
            .trim_end_matches(',');
        let right = words.next().unwrap().trim_end_matches(')');
        nodes.insert(name, (left, right));
    }

    let mut num_steps = 0;
    let mut curr_node = nodes["AAA"];
    loop {
        let direction = directions.next().unwrap();
        let next_node = match direction {
            b'L' => curr_node.0,
            b'R' => curr_node.1,
            _ => panic!(),
        };

        num_steps += 1;

        if next_node == "ZZZ" {
            return num_steps;
        }

        curr_node = *nodes.get(next_node).unwrap();
    }
}

fn solve_b(input: &str) -> usize {
    let mut lines = input.lines();
    let directions = lines.next().unwrap().bytes().cycle();

    let nodes_input = lines.skip(1);
    let mut nodes = HashMap::new();
    for node in nodes_input {
        let mut words = node.split_ascii_whitespace();
        let name = words.next().unwrap();
        let left = words
            .nth(1)
            .unwrap()
            .trim_start_matches('(')
            .trim_end_matches(',');
        let right = words.next().unwrap().trim_end_matches(')');
        nodes.insert(name, (left, right));
    }

    let starting_nodes = nodes.keys().filter(|key| key.ends_with('A'));
    let cycle_step_counts = starting_nodes.map(|start| {
        let mut directions = directions.clone();
        let mut num_steps = 0;
        let mut curr_node = nodes[start];
        loop {
            let direction = directions.next().unwrap();
            let next_node = match direction {
                b'L' => curr_node.0,
                b'R' => curr_node.1,
                _ => panic!(),
            };

            num_steps += 1;

            if next_node.ends_with('Z') {
                break num_steps;
            }

            curr_node = *nodes.get(next_node).unwrap();
        }
    });

    cycle_step_counts.reduce(|a, b| lcm(a, b)).unwrap()
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
        assert_eq!(result, 2);
    }

    #[test]
    fn example_a_2() {
        let result = solve_a(include_str!("./example2.txt"));
        assert_eq!(result, 6);
    }

    #[test]
    fn example_b() {
        let result = solve_b(include_str!("./example_b.txt"));
        assert_eq!(result, 6);
    }
}
