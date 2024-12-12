use aoc_lib::init;
use rustlind_lib::*;
use std::time::Instant;

fn p1(lines: Vec<String>) {
    drop(lines);
}

fn p2(lines: Vec<String>) {
    drop(lines);
}

fn main() {
    let lines = init(2023, 1);
    let timer = Instant::now();
    println!("{:?} | time: {:?}", p1(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p2(lines.clone()), timer.elapsed());
}
