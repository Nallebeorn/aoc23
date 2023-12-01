use std::{env, time::Instant};

const DAY: u32 = 1;

fn extract_coordinates(line: &str) -> i32 {
    let digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();
    let numstr = format!("{}{}", digits.iter().next().unwrap(), digits.iter().rev().next().unwrap());
    numstr.parse().unwrap()
}

fn solve_a(input: &str) -> i32 {
    let lines = input.lines().map(|line| extract_coordinates(line));
    lines.sum()
}

fn extract_coordinates_b(line: &str) -> u32 {

    let digit1 = find_digit1(line);
    let digit2 = find_digit2(line);
    digit1 * 10 + digit2
}

fn find_digit1(word: &str) -> u32 {
    let word_digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let digits = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let mut first_match = usize::MAX;
    let mut found_digit: Option<u32> = None;
    for i in 0..word_digits.len() {
        match word.find(word_digits[i]) {
            Some(index) => {
                if index < first_match {
                    first_match = index;
                    found_digit = Some(i as u32 + 1);
                }
            }
            None => {}
        }

        match word.find(digits[i]) {
            Some(index) => {
                if index < first_match {
                    first_match = index;
                    found_digit = Some(i as u32 + 1);
                }
            }
            None => {}
        }
    }

    found_digit.unwrap()
}

fn find_digit2(word: &str) -> u32 {
    let word_digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let digits = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let mut first_match = 0;
    let mut found_digit: Option<u32> = None;
    for i in 0..word_digits.len() {
        match word.rfind(word_digits[i]) {
            Some(index) => {
                if index >= first_match {
                    first_match = index;
                    found_digit = Some(i as u32 + 1);
                }
            }
            None => {}
        }

        match word.rfind(digits[i]) {
            Some(index) => {
                if index >= first_match {
                    first_match = index;
                    found_digit = Some(i as u32 + 1);
                }
            }
            None => {}
        }
    }

    found_digit.unwrap()
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
mod tests01 {
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
    fn test_find_digit1() {
        assert_eq!(find_digit1("abcfour2threexyz"), 4);
        assert_eq!(find_digit1("77seven"), 7);

    }

    #[test]
    fn test_find_digit2() {
        assert_eq!(find_digit2("abcone2threexyz"), 3);
        assert_eq!(find_digit2("77eight"), 8);

    }

    #[test]
    fn example_b() {
        let result = solve_b(include_str!("./example_b.txt"));
        assert_eq!(result, 281);
    }
}
