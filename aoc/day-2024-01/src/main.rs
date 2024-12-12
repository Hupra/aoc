use aoc_lib::init;
use day_2024_01::loop_match;
use std::{iter::zip, time::Instant, usize};

fn p1(lines: Vec<String>) -> i32 {
    let (mut a, mut b): (Vec<i32>, Vec<i32>) = lines
        .iter()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let l = parts.next().unwrap().parse::<i32>().unwrap();
            let r = parts.next().unwrap().parse::<i32>().unwrap();
            (l, r)
        })
        .unzip();

    a.sort();
    b.sort();

    zip(a, b).map(|(x, y)| (x - y).abs()).sum()
}

fn p2(lines: Vec<String>) -> usize {
    let (mut a, mut b): (Vec<usize>, Vec<usize>) = lines
        .iter()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let l = parts.next().unwrap().parse::<usize>().unwrap();
            let r = parts.next().unwrap().parse::<usize>().unwrap();
            (l, r)
        })
        .unzip();

    a.sort();
    b.sort();

    loop_match(&a, &b)
}

fn main() {
    let lines = init(2024, 1);
    let timer = Instant::now();
    println!("{:?} | time: {:?}", p1(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p2(lines.clone()), timer.elapsed());
}
