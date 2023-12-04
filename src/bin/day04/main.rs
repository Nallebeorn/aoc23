use std::{env, time::Instant};

use itertools::Itertools;

const DAY: u32 = 4;

fn score_game(game: &str) -> usize {
    let mut split = game.split(':').skip(1).next().unwrap().split('|');
    let winners = split.next().unwrap().split(' ').unique();
    let have: Vec<&str> = split.next().unwrap().split(' ').collect();

    let num_winners = winners.filter(|num| !num.is_empty() && have.contains(num)).count();
    (0x01 << num_winners) >> 1
}

fn solve_a(input: &str) -> usize {
    input.lines().map(|line| score_game(line)).sum()
}

fn solve_b(input: &str) -> u32 {
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
        assert_eq!(result, 13);
    }

    // #[test]
    // fn example_b() {
    //     let result = solve_b(include_str!("./example.txt"));
    //     assert_eq!(result, 2286);
    // }
}
