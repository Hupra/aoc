use aoc_lib::init;
use std::{collections::HashMap, time::Instant};

fn check_pattern(s: &str, parts: &Vec<&str>, memo: &mut HashMap<String, usize>) -> usize {
    if s.is_empty() {
        return 1;
    }
    if let Some(&res) = memo.get(s) {
        return res;
    }
    let res = parts
        .iter()
        .filter(|&part| s.starts_with(part))
        .map(|part| check_pattern(&s[part.len()..], parts, memo))
        .sum();

    memo.insert(s.to_string(), res);

    res
}

fn p1(lines: Vec<String>) -> usize {
    let parts: Vec<&str> = lines[0].split(", ").collect();
    let patterns = lines[2..].to_owned();
    let mut memo: HashMap<String, usize> = HashMap::new();
    patterns
        .iter()
        .map(|pattern| (check_pattern(&pattern, &parts, &mut memo) > 0) as usize)
        .sum()
}

fn p2(lines: Vec<String>) -> usize {
    let parts: Vec<&str> = lines[0].split(", ").collect();
    let patterns: Vec<String> = lines[2..].to_owned();
    let mut memo: HashMap<String, usize> = HashMap::new();
    patterns
        .iter()
        .map(|pattern| check_pattern(&pattern, &parts, &mut memo))
        .sum()
}

fn main() {
    let lines = init(2024, 19);
    let timer = Instant::now();
    println!("{:?} | time: {:?}", p1(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p2(lines.clone()), timer.elapsed());
}
