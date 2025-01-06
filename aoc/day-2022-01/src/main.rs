use aoc_lib::init;
use cached::proc_macro::cached;
use itertools::Itertools;
use rustlind_lib::*;
use std::time::Instant;

fn p1(lines: Vec<String>) -> Option<usize> {
    None
}

fn p2(lines: Vec<String>) -> Option<usize> {
    None
}

fn main() {
    let lines = init(2024, 1);
    let timer = Instant::now();
    println!("{:?} | time: {:?}", p1(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p2(lines.clone()), timer.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test_lines() -> Vec<String> {
        include_str!("test.txt").lines().map(String::from).collect()
    }

    #[test]
    fn test_p1() {
        let expected = None;
        let actual = p1(test_lines());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_p2() {
        let expected = None;
        let actual = p2(test_lines());
        assert_eq!(actual, expected);
    }
}
