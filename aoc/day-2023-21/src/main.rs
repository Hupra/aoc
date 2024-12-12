use aoc_lib::init;
use rustlind_lib::*;
use std::{
    collections::{HashMap, VecDeque},
    time::Instant,
    usize,
};

fn solve(lines: Vec<String>) {
    let mut g: HashMap<String, Vec<String>> = HashMap::new();
    let mut v: HashMap<String, usize> = HashMap::new();
    let mut q: VecDeque<(String, usize)> = VecDeque::new();

    let m: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();

    for i in 0..m.len() {
        for j in 0..m[i].len() {
            //
            let name = format!("{}-{}", i, j);
            let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
            for d in valid_positions((i, j), &m, dirs) {
                if m[d.0][d.1] != '#' {
                    g.entry(name.clone())
                        .or_default()
                        .push(format!("{}-{}", d.0, d.1));
                }
            }

            if m[i][j] == 'S' {
                q.push_back((name, 0));
            }
        }
    }

    while let Some((name, step)) = q.pop_front() {
        let vname = v.entry(name.clone()).or_insert(usize::MAX);
        if step < *vname {
            *vname = step;
            for neigbor in g.get(&name).unwrap() {
                q.push_back((neigbor.clone(), step + 1));
            }
        }
    }

    let p1ressult = v.values().filter(|&&val| val <= 64 && val % 2 == 0).count();

    let even_full = v.values().filter(|&&val| val % 2 == 0).count();
    let oddd_full = v.values().filter(|&&val| val % 2 == 1).count();
    let even_corn = v.values().filter(|&&val| val > 65 && val % 2 == 0).count();
    let oddd_corn = v.values().filter(|&&val| val > 65 && val % 2 == 1).count();

    let n: usize = 26501365 / 131;
    println!(
        "n:{} ef:{} of:{} ec:{} oc:{}",
        n, even_full, oddd_full, even_corn, oddd_corn
    );

    let a = even_full * n.pow(2);
    let b = oddd_full * (n + 1).pow(2);
    let c = even_corn * n;
    let d = oddd_corn * (n + 1);

    let p2ressult = a + b + c - d;

    println!("p1: {}", p1ressult);
    println!("p2: {}", p2ressult);
}

fn main() {
    let lines = init(2023, 21);
    let timer = Instant::now();
    solve(lines);
    println!("time: {:?}", timer.elapsed());
}
