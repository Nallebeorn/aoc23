use std::{
    cmp::max,
    env,
    str::{self, from_utf8_unchecked},
    time::Instant,
};

const DAY: u32 = 3;

fn solve_a(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut sum = 0;
    for (y, line) in lines.iter().enumerate() {
        let width = line.len();
        let mut number = Vec::new();
        let mut has_adjacent_symbol = false;
        let bytes = line.as_bytes();
        let mut x = 0;
        while x < width {
            while x < width && bytes[x].is_ascii_digit() {
                let c = bytes[x];
                number.push(c);
                if !has_adjacent_symbol && find_if_has_adjacent_symbol(x, y, width, &lines) {
                    has_adjacent_symbol = true;
                }

                x += 1;

            }

            x += 1;

            if number.len() > 0 {
                let numeric_number: u32 = str::from_utf8(&number).unwrap().parse().unwrap();
                
                if has_adjacent_symbol {
                    sum += numeric_number;
                }

                number.clear();
                has_adjacent_symbol = false;
            }
        }
    }

    sum
}

fn find_if_has_adjacent_symbol(x: usize, y: usize, width: usize, lines: &Vec<&str>) -> bool {
    for (ofsx, ofsy) in [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ] {
        let checkx: i32 = x as i32 + ofsx;
        let checky: i32 = y as i32 + ofsy;
        if checkx >= 0
            && checky >= 0
            && checkx < width as i32
            && checky < lines.len() as i32
        {
            let adjacent = lines
                .get(checky as usize)
                .unwrap()
                .as_bytes()
                .get(checkx as usize)
                .unwrap();

            if !adjacent.is_ascii_digit() && *adjacent != '.' as u8 {
                return true;
            }
        }
    }

    false
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
        assert_eq!(result, 4361);
    }

    //     #[test]
    //     fn example_b() {
    //         let result = solve_b(include_str!("./example.txt"));
    //         assert_eq!(result, 2286);
    //     }
}
