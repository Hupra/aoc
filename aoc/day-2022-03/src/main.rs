use aoc_lib::init;
use cached::proc_macro::cached;
use itertools::Itertools;
use rustlind_lib::*;
use std::{ collections::HashSet, time::Instant };

fn char_to_priority(c: &char) -> usize {
    match c {
        'a'..='z' => (*c as usize) - ('a' as usize) + 1,
        'A'..='Z' => (*c as usize) - ('A' as usize) + 27,
        _ => 0,
    }
}

fn p1(lines: Vec<String>) -> Option<usize> {
    lines
        .iter()
        .map(|line| {
            let (a, b) = line.split_at(line.len() / 2);
            a.chars()
                .collect::<HashSet<char>>()
                .intersection(&b.chars().collect::<HashSet<char>>())
                .next()
                .map(char_to_priority)
        })
        .sum()
}

fn p2(lines: Vec<String>) -> Option<usize> {
    lines
        .iter()
        .chunks(3)
        .into_iter()
        .map(|mut chunk| {
            let a = chunk.next().unwrap_or(&String::new()).chars().collect::<HashSet<char>>();
            let b = chunk.next().unwrap_or(&String::new()).chars().collect::<HashSet<char>>();
            let c = chunk.next().unwrap_or(&String::new()).chars().collect::<HashSet<char>>();
            a.intersection(&b)
                .copied()
                .collect::<HashSet<_>>()
                .intersection(&c)
                .next()
                .map(char_to_priority)
        })
        .sum()
}

fn main() {
    let lines = init(2022, 3);
    let timer = Instant::now();
    println!("{:?} | time: {:?}", p1(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p2(lines.clone()), timer.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let expected = Some(157);
        let actual = p1(include_str!("test.txt").lines().map(String::from).collect());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_p2() {
        let expected = Some(70);
        let actual = p2(include_str!("test.txt").lines().map(String::from).collect());
        assert_eq!(actual, expected);
    }
}
