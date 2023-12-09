use num::integer::lcm;
use std::{collections::HashMap, env, time::Instant};

const DAY: u32 = 9;

fn interpolate_next_val(history: Vec<i32>) -> i32 {
    return if history.iter().all(|n| *n == 0) {
        0
    } else {
        let delta: Vec<i32> = history.windows(2).map(|pair| pair[1] - pair[0]).collect();
        history.last().unwrap() + interpolate_next_val(delta)
    };
}

fn solve_a(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
        })
        .map(|nums| interpolate_next_val(nums.collect())).sum()
}

fn interpolate_prev_val(history: Vec<i32>) -> i32 {
    return if history.iter().all(|n| *n == 0) {
        0
    } else {
        let delta: Vec<i32> = history.windows(2).map(|pair| pair[0] - pair[1]).collect();
        history.first().unwrap() + interpolate_prev_val(delta)
    };
}

fn solve_b(input: &str) -> i32 {
    input
    .lines()
    .map(|line| {
        line.split_ascii_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
    })
    .map(|nums| interpolate_prev_val(nums.collect())).sum()
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
        assert_eq!(result, 114);
    }

        #[test]
        fn example_b() {
            let result = solve_b(include_str!("./example.txt"));
            assert_eq!(result, 2);
        }
}
