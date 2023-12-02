use std::{env, time::Instant, cmp::max};

const DAY: u32 = 2;

fn are_cubes_possible(count: u32, color: &str) -> bool {
    match color {
        "red" => count <= 12,
        "green" => count <= 13,
        "blue" => count <= 14,
        _ => false,
    }
}

fn verify_game(line: &str) -> bool {
    let results = line.split(':').skip(1).next().unwrap();
    let cubes = results.split([';', ',']).map(|cube| cube.trim());
    for cube in cubes {
        let mut split = cube.split(' ');
        let count: u32 = split.next().unwrap().parse().ok().unwrap();
        let color = split.next().unwrap();
        if !are_cubes_possible(count, color) {
            return false;
        }
    }

    true
}

fn solve_a(input: &str) -> usize {
    input.lines().enumerate().filter(|(_, line)| verify_game(line)).map(|(i, _)| i + 1).sum()
}

fn calc_cube_power(line: &str) -> u32 {
    let results = line.split(':').skip(1).next().unwrap();
    let cubes = results.split([';', ',']).map(|cube| cube.trim());
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;
    for cube in cubes {
        let mut split = cube.split(' ');
        let count: u32 = split.next().unwrap().parse().unwrap();
        let color = split.next().unwrap();
        match color {
            "red" => max_red = max(max_red, count),
            "green" => max_green = max(max_green, count),
            "blue" => max_blue = max(max_blue, count),
            _ => {}
        }
    }

    max_red * max_green * max_blue
}

fn solve_b(input: &str) -> u32 {
    input.lines().map(|line| calc_cube_power(line)).sum()
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
        assert_eq!(result, 8);
    }

    #[test]
    fn example_b() {
        let result = solve_b(include_str!("./example.txt"));
        assert_eq!(result, 2286);
    }
}
