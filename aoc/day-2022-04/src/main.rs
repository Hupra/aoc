use aoc_lib::init;
use itertools::Itertools;
use rustlind_lib::*;
use std::time::Instant;

fn p1(lines: Vec<String>) -> Option<usize> {
    lines
        .iter()
        .map(|line| re_nums(&line).iter().copied().collect_tuple().unwrap_or_default())
        .map(|(a1, b1, a2, b2)| Some(((a1 >= a2 && b1 <= b2) || (a1 <= a2 && b1 >= b2)) as usize))
        .sum()
}

fn p2(lines: Vec<String>) -> Option<usize> {
    // lines
    //     .iter()
    //     .map(|line| re_nums(&line).iter().copied().collect_tuple().unwrap_or_default())
    //     .map(|(a1, b1, a2, b2)| {
    //         let r1: HashSet<i32> = (a1..=b1).collect();
    //         let r2: HashSet<i32> = (a2..=b2).collect();
    //         Some((r1.intersection(&r2).collect::<HashSet<&i32>>().len() > 0) as usize)
    //     })
    //     .sum()

    lines
        .iter()
        .map(|line| re_nums(&line).iter().copied().collect_tuple().unwrap_or_default())
        .map(|(a1, b1, a2, b2)| Some(((a1 <= a2 && b1 >= a2) || (a2 <= a1 && b2 >= a1)) as usize))
        .sum()
}

fn main() {
    let lines = init(2022, 4);
    let timer = Instant::now();
    println!("{:?} | time: {:?}", p1(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p2(lines.clone()), timer.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let expected = Some(2);
        let actual = p1(include_str!("test.txt").lines().map(String::from).collect());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_p2() {
        let expected = Some(4);
        let actual = p2(include_str!("test.txt").lines().map(String::from).collect());
        assert_eq!(actual, expected);
    }
}
