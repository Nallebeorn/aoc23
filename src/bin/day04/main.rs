use std::{env, time::Instant};

const DAY: u32 = 4;

fn score_game(game: &str) -> usize {
    let mut split = game.split(':').skip(1).next().unwrap().split('|');
    let winners = split.next().unwrap().split(' ');
    let have: Vec<&str> = split.next().unwrap().split(' ').collect();

    let num_winners = winners.filter(|num| !num.is_empty() && have.contains(num)).count();
    (0x01 << num_winners) >> 1
}

fn solve_a(input: &str) -> usize {
    input.lines().map(|line| score_game(line)).sum()
}

fn score_card_b(game: &str) -> usize {
    let mut split = game.split(':').skip(1).next().unwrap().split('|');
    let winners = split.next().unwrap().split(' ');
    let have: Vec<&str> = split.next().unwrap().split(' ').collect();

    winners.filter(|num| !num.is_empty() && have.contains(num)).count()
}

fn solve_b(input: &str) -> usize {
    let card_scores: Vec<usize> = input.lines().map(|card| score_card_b(card)).collect();

    let mut cards: Vec<usize> = (0..card_scores.len()).collect();

    let mut i = 0;
    while i < cards.len() {
        let card_number = cards[i];
        let score = card_scores[card_number];
        for card_won in card_number + 1..card_number + 1 + score {
            cards.push(card_won);
        }
        i += 1;
    }

    cards.len()
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

    #[test]
    fn example_b() {
        let result = solve_b(include_str!("./example.txt"));
        assert_eq!(result, 30);
    }
}
