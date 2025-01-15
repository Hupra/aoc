use aoc_lib::init;
use cached::proc_macro::cached;
use itertools::Itertools;
use rustlind_lib::*;
use std::time::Instant;

fn p1(lines: Vec<String>) -> Option<usize> {
    lines
        .into_iter()
        .map(|line|
            line
                .split_whitespace()
                .map(|s| s.chars().next().unwrap())
                .collect_vec()
        )
        .map(|pair| {
            let (a, b) = (&pair[0], &pair[1]);
            let bonus: usize = match b {
                'X' => 1,
                'Y' => 2,
                'Z' => 3,
                _ => 0,
            };
            let score: usize = match (a, b) {
                ('A', 'X') => 3,
                ('B', 'Y') => 3,
                ('C', 'Z') => 3,
                ('A', 'Y') => 6,
                ('B', 'Z') => 6,
                ('C', 'X') => 6,
                _ => 0,
            };
            Some(score + bonus)
        })
        .sum()
}

fn p2(lines: Vec<String>) -> Option<usize> {
    lines
        .into_iter()
        .map(|line|
            line
                .split_whitespace()
                .map(|s| s.chars().next().unwrap())
                .collect_vec()
        )
        .map(|pair| {
            // A = X = 1 ROCK     | LOSE
            // B = Y = 2 PAPER    | DRAW
            // C = Z = 3 SCISSORS | WIN
            let (a, b) = (&pair[0], &pair[1]);
            let bonus: usize = match (a, b) {
                ('A', 'X') => 3,
                ('B', 'X') => 1,
                ('C', 'X') => 2,
                ('A', 'Y') => 1,
                ('B', 'Y') => 2,
                ('C', 'Y') => 3,
                ('A', 'Z') => 2,
                ('B', 'Z') => 3,
                ('C', 'Z') => 1,
                _ => 0,
            };
            let score: usize = match b {
                'X' => 0,
                'Y' => 3,
                'Z' => 6,
                _ => 0,
            };
            Some(score + bonus)
        })
        .sum()
}

fn main() {
    let lines = init(2022, 2);
    let timer = Instant::now();
    println!("{:?} | time: {:?}", p1(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p2(lines.clone()), timer.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let expected = Some(15);
        let actual = p1(include_str!("test.txt").lines().map(String::from).collect());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_p2() {
        let expected = Some(12);
        let actual = p2(include_str!("test.txt").lines().map(String::from).collect());
        assert_eq!(actual, expected);
    }
}
