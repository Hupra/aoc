use aoc_lib::init;
use rustlind_lib::*;
use std::{collections::HashSet, time::Instant};

macro_rules! tadd {
    ($tuple1:expr, $tuple2:expr) => {
        ($tuple1.0 + $tuple2.0, $tuple1.1 + $tuple2.1)
    };
}

macro_rules! tmul {
    ($t:expr, $mul:expr) => {
        ($t.0 * $mul, $t.1 * $mul)
    };
}

fn inbound(i: i32, j: i32, m: &Vec<Vec<char>>) -> bool {
    i >= 0 && j >= 0 && i < m.len() as i32 && j < m[i as usize].len() as i32
}

fn antinodes(a: (i32, i32), b: (i32, i32), m: &Vec<Vec<char>>) -> HashSet<(i32, i32)> {
    let offset = (a.0 - b.0, a.1 - b.1);

    let mut nodes: HashSet<(i32, i32)> = HashSet::new();

    let i = a.0 - offset.0;
    let j = a.1 - offset.1;
    let ij = (i, j);
    if inbound(i, j, m) && ij != a && ij != b {
        nodes.insert(ij);
    }

    let i = a.0 + offset.0;
    let j = a.1 + offset.1;
    let ij = (i, j);
    if inbound(i, j, m) && ij != a && ij != b {
        nodes.insert(ij);
    }

    let i = b.0 - offset.0;
    let j = b.1 - offset.1;
    let ij = (i, j);
    if inbound(i, j, m) && ij != a && ij != b {
        nodes.insert(ij);
    }

    let i = b.0 + offset.0;
    let j = b.1 + offset.1;
    let ij = (i, j);
    if inbound(i, j, m) && ij != a && ij != b {
        nodes.insert(ij);
    }

    nodes
}

fn antinodes_diag(a: (i32, i32), b: (i32, i32), m: &Vec<Vec<char>>) -> HashSet<(i32, i32)> {
    let offset = (a.0 - b.0, a.1 - b.1);

    let mut nodes: HashSet<(i32, i32)> = HashSet::new();

    //pos loop
    let mut x = 1;
    loop {
        let pos = tadd!(a, tmul!(offset, x));
        if !inbound(pos.0, pos.1, m) {
            break;
        }
        if pos != a && pos != b {
            nodes.insert(pos);
        }
        x += 1;
    }

    //neg loop
    let mut x = -1;
    loop {
        let pos = tadd!(a, tmul!(offset, x));
        if !inbound(pos.0, pos.1, m) {
            break;
        }
        if pos != a && pos != b {
            nodes.insert(pos);
        }
        x -= 1;
    }

    nodes
}
fn p1(lines: Vec<String>) -> usize {
    let m: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let mut signals: Vec<(i32, i32)> = Vec::new();

    for i in 0..m.len() {
        for j in 0..m[i].len() {
            if m[i][j] != '.' {
                signals.push((i as i32, j as i32));
            }
        }
    }

    let mut noize: HashSet<(i32, i32)> = HashSet::new();

    for a in 0..signals.len() {
        for b in a + 1..signals.len() {
            let a = signals[a];
            let b = signals[b];
            if m[a.0 as usize][a.1 as usize] == m[b.0 as usize][b.1 as usize] {
                noize.extend(antinodes(a, b, &m));
            }
        }
    }

    noize.len()
}

fn p2(lines: Vec<String>) -> usize {
    let mut m: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let mut signals: Vec<(i32, i32)> = Vec::new();

    for i in 0..m.len() {
        for j in 0..m[i].len() {
            if m[i][j] != '.' {
                signals.push((i as i32, j as i32));
            }
        }
    }

    let mut noize: HashSet<(i32, i32)> = HashSet::new();

    for a in 0..signals.len() {
        for b in a + 1..signals.len() {
            let a = signals[a];
            let b = signals[b];
            if m[a.0 as usize][a.1 as usize] == m[b.0 as usize][b.1 as usize] {
                noize.extend(antinodes_diag(a, b, &m));
            }
        }
    }
    noize.extend(signals);
    noize.len()
}

fn main() {
    let lines = init(2024, 8);
    let timer = Instant::now();
    println!("{:?} | time: {:?}", p1(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p2(lines.clone()), timer.elapsed());
}
