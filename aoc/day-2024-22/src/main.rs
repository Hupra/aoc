use aoc_lib::init;
use cached::proc_macro::cached;
use itertools::Itertools;
use rustlind_lib::*;
use std::time::Instant;

fn p1(lines: Vec<String>) -> Option<usize> {
    drop(lines);
    None
}

fn p2(lines: Vec<String>) -> Option<usize> {
    drop(lines);
    None
}

fn main() {
    let lines = init(2024, 22);
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
        let expected = Some(8685429 + 4700978 + 15273692 + 8667524);
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
