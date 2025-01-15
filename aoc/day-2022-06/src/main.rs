use aoc_lib::init;
use cached::proc_macro::cached;
use itertools::Itertools;
use rustlind_lib::*;
use std::{ collections::HashSet, time::Instant };

fn p1(lines: Vec<String>) -> Option<usize> {
    lines.get(0).and_then(|line|
        line
            .chars()
            .collect_vec()
            .windows(4)
            .position(|window| window.iter().collect::<HashSet<_>>().len() == 4)
            .and_then(|i| Some(i + 4))
    )
}

fn p2(lines: Vec<String>) -> Option<usize> {
    lines.get(0).and_then(|line|
        line
            .chars()
            .collect_vec()
            .windows(14)
            .position(|window| window.iter().collect::<HashSet<_>>().len() == 14)
            .and_then(|i| Some(i + 14))
    )
}

fn main() {
    let lines = init(2022, 6);
    let timer = Instant::now();
    println!("{:?} | time: {:?}", p1(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p2(lines.clone()), timer.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let expected = Some(11);
        let actual = p1(include_str!("test.txt").lines().map(String::from).collect());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_p2() {
        let expected = Some(26);
        let actual = p2(include_str!("test.txt").lines().map(String::from).collect());
        assert_eq!(actual, expected);
    }
}
