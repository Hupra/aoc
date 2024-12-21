use aoc_lib::{init, print_matrix};
use rustlind_lib::*;
use std::time::Instant;

fn p1(lines: Vec<String>) -> i32 {
    let size = 71;
    let mut m: Vec<Vec<char>> = vec![vec!['.'; size]; size];

    lines
        .iter()
        .take(1024)
        .map(|line| re_nums(&line))
        .for_each(|v| m[v[1] as usize][v[0] as usize] = '#');

    let mut g: Graph<(usize, usize), i32> = Graph::new();
    for i in 0..m.len() {
        for j in 0..m[i].len() {
            if m[i][j] == '#' {
                continue;
            }
            valid_positions((i, j), &m, [(1, 0), (-1, 0), (0, 1), (0, -1)])
                .into_iter()
                .for_each(|pos| g.add_edge((i, j), pos, 1));
        }
    }
    let (dist, _path) = g.shortest_path((0, 0), (size - 1, size - 1));
    dist.unwrap()
}

fn p2(lines: Vec<String>) -> Option<String> {
    let size = 71;

    let mut low = 0;
    let mut high = lines.len();

    while low < high {
        let mid = (low + high) / 2;

        let i = mid;

        println!("{} {} {}, {:?}", low, mid, high, &lines[i]);
        let mut m: Vec<Vec<char>> = vec![vec!['.'; size]; size];

        lines
            .iter()
            .take(i)
            .map(|line| re_nums(&line))
            .for_each(|v| m[v[1] as usize][v[0] as usize] = '#');

        let mut g: Graph<(usize, usize), i32> = Graph::new();
        for i in 0..m.len() {
            for j in 0..m[i].len() {
                if m[i][j] == '#' {
                    continue;
                }
                valid_positions((i, j), &m, [(1, 0), (-1, 0), (0, 1), (0, -1)])
                    .into_iter()
                    .for_each(|pos| g.add_edge((i, j), pos, 1));
            }
        }
        let (dist, _path) = g.shortest_path((0, 0), (size - 1, size - 1));

        if dist.is_none() {
            // Potential candidate found. Search the left half for a lower index.
            high = mid;
        } else {
            // Current element doesn't satisfy the predicate. Search the right half.
            low = mid + 1;
        }
    }

    if low < lines.len() {
        Some(lines[low - 1].clone())
    } else {
        None
    }
}

fn has_path(i: usize, lines: &Vec<String>, size: usize) -> bool {
    let mut m: Vec<Vec<char>> = vec![vec!['.'; size]; size];

    lines
        .iter()
        .take(i)
        .map(|line| re_nums(&line))
        .for_each(|v| m[v[1] as usize][v[0] as usize] = '#');

    let mut g: Graph<(usize, usize), i32> = Graph::new();
    for i in 0..m.len() {
        for j in 0..m[i].len() {
            if m[i][j] == '#' {
                continue;
            }
            valid_positions((i, j), &m, [(1, 0), (-1, 0), (0, 1), (0, -1)])
                .into_iter()
                .for_each(|pos| g.add_edge((i, j), pos, 1));
        }
    }
    let (dist, _path) = g.shortest_path((0, 0), (size - 1, size - 1));
    dist.is_some()
}

fn p22(lines: Vec<String>) -> String {
    let pp = (0..lines.len())
        .collect::<Vec<usize>>()
        .partition_point(|&i| has_path(i, &lines, 71));
    return lines[pp - 1].clone();
}

fn main() {
    let lines = init(2024, 18);
    let timer = Instant::now();
    println!("{:?} | time: {:?}", p1(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p2(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p22(lines.clone()), timer.elapsed());
}
