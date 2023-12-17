use std::{env, time::Instant};

const DAY: u32 = 12;

#[derive(Debug)]
struct Line<'a> {
    conditions: &'a str,
    summary: Vec<i32>,
}

fn get_summary(conditions: &str) -> Vec<i32> {
    let mut summary = Vec::new();

    let mut chars = conditions.bytes();
    let mut counter = 0;
    loop {
        let c = chars.next();

        match c {
            Some(b'#') => counter += 1,
            None | Some(b'.') => {
                if counter > 0 {
                    summary.push(counter);
                    counter = 0;
                }
            }
            _ => panic!(),
        }

        if c.is_none() {
            break;
        }
    }

    summary
}

fn parse_line(line: &str) -> Line {
    let mut split = line.split_ascii_whitespace();

    Line {
        conditions: split.next().unwrap(),
        summary: split
            .next()
            .unwrap()
            .split(',')
            .map(|c| c.parse().unwrap())
            .collect(),
    }
}

fn get_num_arrangements(conditions_with_unkowns: &str, summary: &Vec<i32>) -> usize {
    let num_unknowns = conditions_with_unkowns
        .bytes()
        .filter(|c| *c == b'?')
        .count();

    let mut counter = 0;

    if num_unknowns == 0 {
        return if are_vectors_same(&get_summary(conditions_with_unkowns), &summary) {
            1
        } else {
            0
        }
    }

    for i in 0..=num_unknowns {
        let conditions = conditions_with_unkowns.replacen('?', ".", i).replacen('?', "#", 1);
        counter += get_num_arrangements(&conditions, &summary);
    }

    counter
}

fn are_vectors_same(a: &Vec<i32>, b: &Vec<i32>) -> bool {
    return if a.len() == b.len() {
        for i in 0..a.len() {
            if a[i] != b[i] {
                return false;
            }
        }

        true
    } else {
        false
    };
}

fn solve_a(input: &str) -> usize {
    let lines = input.lines().map(|line| parse_line(line));

    lines.map(|line| get_num_arrangements(line.conditions, &line.summary)).sum()
}

fn solve_b(input: &str) -> i64 {
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
    fn example() {
        let result = solve_a(include_str!("./example.txt"));
        assert_eq!(result, 21);
    }

    #[test]
    fn test_get_summary() {
        let result = get_summary(".#.###.#.######");
        let expected = vec![1, 3, 1, 6];
        assert_eq!(
            are_vectors_same(&result, &expected),
            true,
            "{:?} (got) == {:?} (expected)",
            result,
            expected
        );
    }

    #[test]
    fn test_get_summary_2() {
        let result = get_summary("#....######..#####.");
        let expected = vec![1, 6, 5];
        assert_eq!(
            are_vectors_same(&result, &expected),
            true,
            "{:?} (got) == {:?} (expected)",
            result,
            expected
        );
    }

    #[test]
    fn test_get_num_arrangements() {
        let line = Line{conditions: "????.######..#####.", summary: vec![1, 6, 5] };
        let result = get_num_arrangements(line.conditions, &line.summary);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_b() {
        let result = solve_b(include_str!("./example.txt"));
        assert_eq!(result, 525152);
    }
}
