use aoc_lib::init;
use rustlind_lib::*;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    time::Instant,
};

fn p1(lines: Vec<String>) -> usize {
    let m = lines
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let mut sum: usize = 0;
    let mut memo: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();

    for i in 0..m.len() {
        for j in 0..m[i].len() {
            if m[i][j] == 0 {
                rec((i, j), &m, &mut memo);
                let res = memo
                    .get(&(i, j))
                    .and_then(|hs| Some(hs.len()))
                    .unwrap_or_default();
                // println!("{:?} {:?}", (i, j), res);

                sum += res;
            }
        }
    }

    fn rec(
        point: (usize, usize),
        m: &Vec<Vec<usize>>,
        memo: &mut HashMap<(usize, usize), HashSet<(usize, usize)>>,
    ) {
        if memo.contains_key(&point) {
            return;
        }
        if m[point.0][point.1] == 9 {
            memo.insert(point, HashSet::from([point]));
            return;
        }

        let mut hm: HashSet<(usize, usize)> = HashSet::new();

        for neighbor_point in valid_positions(point, &m, [(1, 0), (-1, 0), (0, 1), (0, -1)]) {
            if m[point.0][point.1] + 1 == m[neighbor_point.0][neighbor_point.1] {
                rec(neighbor_point, m, memo);

                if let Some(hs) = memo.get(&neighbor_point) {
                    hm.extend(hs.clone());
                }
            }
        }

        memo.insert(point, hm);
    }

    sum
}

fn p2(lines: Vec<String>) -> usize {
    let m = lines
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    // i wanna do recursion! with memo. just loop and each point figures out how long they an go
    // then just sum the res of each 0 at the end
    // maybe we just scan the 0s but we will see

    let mut sum: usize = 0;
    let mut memo: HashMap<(usize, usize), usize> = HashMap::new();

    for i in 0..m.len() {
        for j in 0..m[i].len() {
            if m[i][j] == 0 {
                let res = rec((i, j), &m, &mut memo);
                sum += res;
            }
        }
    }

    fn rec(
        point: (usize, usize),
        m: &Vec<Vec<usize>>,
        memo: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if let Some(res) = memo.get(&point) {
            return *res;
        }
        if m[point.0][point.1] == 9 {
            memo.insert(point, 1);
            return 1;
        }

        let mut sum: usize = 0;
        for neighbor_point in valid_positions(point, &m, [(1, 0), (-1, 0), (0, 1), (0, -1)]) {
            if m[point.0][point.1] + 1 == m[neighbor_point.0][neighbor_point.1] {
                sum += rec(neighbor_point, m, memo);
            }
        }

        memo.insert(point, sum);
        sum
    }

    sum
}

fn main() {
    let lines = init(2024, 10);
    let timer = Instant::now();
    println!("{:?} | time: {:?}", p1(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p2(lines.clone()), timer.elapsed());
}
