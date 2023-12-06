use std::{env, time::Instant};

const DAY: u32 = 6;

fn solve_a(input: &str) -> usize {
    let mut lines = input.lines().map(|line| {
        line.split_ascii_whitespace()
            .skip(1)
            .map(|n| n.parse().unwrap())
            .collect::<Vec<i32>>()
    });

    let times = lines.next().unwrap();
    let distances = lines.next().unwrap();

    let mut result = 1;
    for i in 0..times.len() {
        let time = times[i];
        let distance = distances[i];
        result *= (0..time + 1).filter(|speed| speed * (time - speed) > distance).count();
    }

    result
}

fn solve_b(input: &str) -> usize {
    let mut lines = input.lines().map(|line| {
        line.split(':')
            .nth(1)
            .unwrap()
            .replace(' ', "")
            .parse::<i64>()
            .unwrap()
    });

    let time = lines.next().unwrap();
    let distance = lines.next().unwrap();

    (0..time + 1).filter(|speed| speed * (time - speed) > distance).count()

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
        assert_eq!(result, 288);
    }

    #[test]
    fn example_b() {
        let result = solve_b(include_str!("./example.txt"));
        assert_eq!(result, 71503);
    }
}
