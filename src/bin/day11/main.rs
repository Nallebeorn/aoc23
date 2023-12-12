use std::{env, time::Instant};

const DAY: u32 = 11;

fn solve_a(input: &str) -> i64 {
    solve(input, 2)
}

fn solve_b(input: &str) -> i64 {
    solve(input, 1_000_000)
}

fn solve(input: &str, expansion: i64) -> i64 {
    let mut galaxy_locations: Vec<(i64, i64)> = Vec::new();

    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();

    let mut y = 0;
    for line in &lines {
        let mut x = 0;
        for (column, char) in line.bytes().enumerate() {
            if char == b'#' {
                galaxy_locations.push((x, y));
            }

            x += if (0..height).all(|check_y| lines[check_y].bytes().nth(column).unwrap() == b'.') {
                expansion
            } else {
                1
            };
        }

        y += if line.bytes().all(|c| c == b'.') {
            expansion
        } else {
            1
        };
    }

    let mut sum = 0;

    for i in 0..galaxy_locations.len() {
        for j in (i + 1)..galaxy_locations.len() {
            let (x1, y1) = galaxy_locations[i];
            let (x2, y2) = galaxy_locations[j];
            sum += (x1 - x2).abs() + (y1 - y2).abs()
        }
    }

    sum
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
        assert_eq!(result, 374);
    }

    #[test]
    fn example_b_10() {
        let result = solve(include_str!("./example.txt"), 10);
        assert_eq!(result, 1030);
    }

    #[test]
    fn example_b_100() {
        let result = solve(include_str!("./example.txt"), 100);
        assert_eq!(result, 8410);
    }
}
