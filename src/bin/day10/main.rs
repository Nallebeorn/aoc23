use std::{env, time::Instant};

const DAY: u32 = 10;

fn solve_a(input: &str) -> i32 {
    let width = input.lines().next().unwrap().len();

    let get_idx = |x: usize, y: usize| -> usize {
        return y * width + x;
    };

    let height = input.lines().count();
    let mut grid = vec![None; width * height];
    let (mut startx, mut starty) = (0, 0);
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.bytes().enumerate() {
            grid[get_idx(x, y)] = match c {
                b'.' => None,
                b'|' => Some((get_idx(x, y - 1), get_idx(x, y + 1))),
                b'-' => Some((get_idx(x - 1, y), get_idx(x + 1, y))),
                b'L' => Some((get_idx(x, y - 1), get_idx(x + 1, y))),
                b'J' => Some((get_idx(x, y - 1), get_idx(x - 1, y))),
                b'7' => Some((get_idx(x - 1, y), get_idx(x, y + 1))),
                b'F' => Some((get_idx(x + 1, y), get_idx(x, y + 1))),
                b'S' => {
                    (startx, starty) = (x as i32, y as i32);
                    None
                }
                _ => panic!(),
            }
        }
    }

    let possible_connections: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let start_idx = get_idx(startx as usize, starty as usize);
    let mut start_con_a = None;
    let mut start_con_b = None;
    for (ofsx, ofsy) in possible_connections {
        let (x, y) = ((startx + ofsx) as usize, (starty + ofsy) as usize);
        if let Some((a, b)) = grid[get_idx(x, y)] {
            if a == start_idx || b == start_idx {
                if start_con_a.is_none() {
                    start_con_a = Some(get_idx(x, y));
                } else {
                    if get_idx(x, y) != start_con_a.unwrap() {
                        start_con_b = Some(get_idx(x, y));
                        break;
                    }
                }
            }
        }
    }

    grid[start_idx] = Some((start_con_a.unwrap(), start_con_b.unwrap()));

    let mut loop_len = 0;
    let mut index = start_idx;
    let mut prev = start_idx;
    loop {
        let (a, b) = grid[index].unwrap();
        let next = if a == prev { b } else { a };
        prev = index;
        index = next;

        loop_len += 1;

        if index == start_idx {
            break;
        }
    }

    loop_len / 2
}

fn solve_b(input: &str) -> i32 {
    let width = input.lines().next().unwrap().len();

    let get_idx = |x: i32, y: i32| -> usize {
        return (y * (width as i32) + x) as usize;
    };

    let height = input.lines().count();
    let mut src_grid = vec!['.'; width * height];
    let mut grid = vec![None; width * height];
    let (mut startx, mut starty) = (0, 0);
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.bytes().enumerate() {
            let (x, y) = (x as i32, y as i32);
            src_grid[get_idx(x, y)] = c as char;
            grid[get_idx(x, y)] = match c {
                b'.' => None,
                b'|' => Some((get_idx(x, y - 1), get_idx(x, y + 1))),
                b'-' => Some((get_idx(x - 1, y), get_idx(x + 1, y))),
                b'L' => Some((get_idx(x, y - 1), get_idx(x + 1, y))),
                b'J' => Some((get_idx(x, y - 1), get_idx(x - 1, y))),
                b'7' => Some((get_idx(x - 1, y), get_idx(x, y + 1))),
                b'F' => Some((get_idx(x + 1, y), get_idx(x, y + 1))),
                b'S' => {
                    (startx, starty) = (x as i32, y as i32);
                    None
                }
                _ => panic!(),
            }
        }
    }

    let possible_connections: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let start_idx = get_idx(startx, starty);
    let mut start_con_a = None;
    let mut start_con_b = None;
    for (ofsx, ofsy) in possible_connections {
        let (x, y) = (startx + ofsx, starty + ofsy);
        let idx = get_idx(x, y);
        if idx < grid.len() {
            if let Some((a, b)) = grid[idx] {
                if a == start_idx || b == start_idx {
                    if start_con_a.is_none() {
                        start_con_a = Some(get_idx(x, y));
                    } else {
                        if get_idx(x, y) != start_con_a.unwrap() {
                            start_con_b = Some(get_idx(x, y));
                            break;
                        }
                    }
                }
            }
        }
    }

    grid[start_idx] = Some((start_con_a.unwrap(), start_con_b.unwrap()));

    let mut is_loop = vec![false; width * height];
    let mut index = start_idx;
    let mut prev = start_idx;
    loop {
        let (a, b) = grid[index].unwrap();
        let next = if a == prev { b } else { a };
        prev = index;
        index = next;

        is_loop[index] = true;

        if index == start_idx {
            break;
        }
    }

    let mut num_inside = 0;
    let mut debug = String::new();
    for index in 0..is_loop.len() {
        if index % width == 0 {
            debug.push('\n');
        }

        if is_loop[index] {
            debug.push(src_grid[index]);
            continue;
        }

        let mut num_crossings = 0;
        let mut check_idx = index as i32 - width as i32;
        while check_idx > 0 {
            if is_loop[check_idx as usize] {
                if src_grid[check_idx as usize] != '|' {
                    num_crossings += 1;
                }
                loop {
                    let (a, b) = grid[check_idx as usize].unwrap();
                    if !(a == check_idx as usize - width || b == check_idx as usize - width) {
                        break;
                    }
                    check_idx -= width as i32;
                }
            }
            check_idx -= width as i32;
        }

        if num_crossings % 2 == 1 {
            num_inside += 1;
            debug.push('I');
        } else {
            debug.push('.');
        }
    }

    print!("{}", debug);

    num_inside
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
        let result = solve_a(include_str!("./example_a.txt"));
        assert_eq!(result, 4);
    }

    #[test]
    fn example_b() {
        let result = solve_b(include_str!("./example_b_1.txt"));
        assert_eq!(result, 4);
    }

    #[test]
    fn example_b_2() {
        let result = solve_b(include_str!("./example_b_2.txt"));
        assert_eq!(result, 8);
    }
}
