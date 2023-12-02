use std::{env, time::Instant};

const DAY: u32 = 1;

fn extract_coordinates(line: &str) -> u32 {
    let digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();
    let digit1 = digits.iter().next().unwrap().to_digit(10).unwrap();
    let digit2 = digits.iter().rev().next().unwrap().to_digit(10).unwrap();
    digit1 * 10 + digit2
}

fn solve_a(input: &str) -> u32 {
    let lines = input.lines().map(|line| extract_coordinates(line));
    lines.sum()
}

fn extract_coordinates_b(line: &str) -> u32 {
    let word_digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let digits = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];

    let mut first_match = usize::MAX;
    let mut last_match = 0;

    let mut left_digit = 0;
    let mut right_digit = 0;

    for i in 0..word_digits.len() {
        if let Some(index) = line.find(word_digits[i]) {
            if index < first_match {
                first_match = index;
                left_digit = i as u32 + 1;
            }
        }

        if let Some(index) = line.find(digits[i]) {
            if index < first_match {
                first_match = index;
                left_digit = i as u32 + 1;
            }
        }

        if let Some(index) = line.rfind(word_digits[i]) {
            if index >= last_match {
                last_match = index;
                right_digit = i as u32 + 1;
            }
        }

        if let Some(index) = line.rfind(digits[i]) {
            if index >= last_match {
                last_match = index;
                right_digit = i as u32 + 1;
            }
        }
    }

    left_digit * 10 + right_digit
}


fn solve_b(input: &str) -> u32 {
    let lines = input.lines().map(|line| extract_coordinates_b(line));
    lines.sum()
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
        assert_eq!(result, 142);
    }

    #[test]
    fn test_extract_coordinates() {
        let result = extract_coordinates("a1b2c3d4e5f");
        assert_eq!(result, 15);
    }

    #[test]
    fn test_extract_coordinates_one_digit() {
        let result = extract_coordinates("treb7uchet");
        assert_eq!(result, 77);
    }

    #[test]
    fn example_b() {
        let result = solve_b(include_str!("./example_b.txt"));
        assert_eq!(result, 281);
    }
}
