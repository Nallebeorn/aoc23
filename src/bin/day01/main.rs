use std::env;

const DAY: u32 = 1;

fn solve_a(input: &str) -> i32 {
    let lines: Vec<Option<i32>> = input.lines().map(|line| line.parse().ok()).collect();
    let mut max = 0;
    let mut acc = 0;
    for line in lines {
        match line {
            Some(number) => acc += number,
            None => {
                if acc > max {
                    max = acc
                }
                acc = 0;
            }
        }
    }

    max
}

fn solve_b(input: &str) -> i32 {
    let lines: Vec<Option<i32>> = input.lines().map(|line| line.parse().ok()).collect();
    let mut elf_calories = Vec::new();
    let mut acc = 0;
    for line in lines {
        match line {
            Some(number) => acc += number,
            None => {
                elf_calories.push(acc);
                acc = 0;
            }
        }
    }

    elf_calories.sort_unstable();
    elf_calories.iter().rev().take(3).sum()
}

fn main() {
    let args: Vec<String> = env::args().skip(1).map(|arg| arg.to_lowercase()).collect();

    if args.is_empty() || args.iter().any(|arg| arg == "a") {
        println!("**** DECEMBER {} (a) ****", DAY);
        let result = solve_a(include_str!("./input.txt"));
        println!("{}\n", result)
    }

    if args.is_empty() || args.iter().any(|arg| arg == "b") {
        println!("**** DECEMBER {} (b) ****", DAY);
        let result = solve_b(include_str!("./input.txt"));
        println!("{}\n", result)
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
        assert_eq!(result, 24000);
    }

    #[test]
    fn example_b() {
        let result = solve_b(include_str!("./example.txt"));
        assert_eq!(result, 45000);
    }
}
