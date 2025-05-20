use aoc_lib::{ init, print_matrix };
use cached::proc_macro::cached;
use itertools::Itertools;
use rustlind_lib::valid_positions;
use rustlind_lib::utils::simple_graph::Graph;
use std::collections::{ vec_deque, VecDeque };
use std::time::Instant;
use std::{ fs, usize };

fn p1(lines: Vec<String>) -> Option<usize> {
    let mut m = lines
        .iter()
        .map(|line|
            line
                .chars()
                .map(|c| c as usize)
                .collect_vec()
        )
        .collect_vec();

    let d = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
    ];

    let mut g = Graph::<(usize, usize)>::new();

    let mut s = (0, 0);
    let mut t = (0, 0);

    for i in 0..m.len() {
        for j in 0..m[i].len() {
            match m[i][j] {
                u if u == ('S' as usize) => {
                    s = (i, j);
                    m[i][j] = 'a' as usize;
                }
                u if u == ('E' as usize) => {
                    t = (i, j);
                    m[i][j] = 'z' as usize;
                }
                _ => {}
            }
        }
    }

    g.add_edge(s, s);
    g.add_edge(t, t);

    for i in 0..m.len() {
        for j in 0..m[i].len() {
            for (ni, nj) in valid_positions((i, j), &m, d) {
                if m[i][j] + 1 >= m[ni][nj] {
                    g.add_edge((i, j), (ni, nj));
                }
            }
        }
    }

    let distances = g.bfs_distances(s).unwrap();

    // dbg!(g.add_edge(s.clone(), t.clone()));
    // dbg!(g.add_edge(s.clone(), t.clone()));

    distances.get(&t).and_then(|&u| Some(u))
}

fn p2(lines: Vec<String>) -> Option<usize> {
    let mut m = lines
        .iter()
        .map(|line|
            line
                .chars()
                .map(|c| c as usize)
                .collect_vec()
        )
        .collect_vec();

    let d = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
    ];

    let mut g = Graph::<(usize, usize)>::new();

    let mut s = (0, 0);
    let mut t = (0, 0);

    for i in 0..m.len() {
        for j in 0..m[i].len() {
            match m[i][j] {
                u if u == ('S' as usize) => {
                    s = (i, j);
                    m[i][j] = 'a' as usize;
                }
                u if u == ('E' as usize) => {
                    t = (i, j);
                    m[i][j] = 'z' as usize;
                }
                _ => {}
            }
        }
    }

    g.add_edge(s, s);
    g.add_edge(t, t);

    for i in 0..m.len() {
        for j in 0..m[i].len() {
            for (ni, nj) in valid_positions((i, j), &m, d) {
                if m[i][j] + 1 >= m[ni][nj] {
                    g.add_edge((ni, nj), (i, j));
                }
            }
        }
    }

    let distances = g.bfs_distances(t).unwrap();

    distances
        .iter()
        .filter(|&(&(i, j), &_dist)| m[i][j] == ('a' as usize))
        .map(|(_key, &val)| val)
        .min()
}

fn main() {
    let lines = init(2022, 12);
    let timer = Instant::now();
    println!("{:?} | time: {:?}", p1(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p2(lines.clone()), timer.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let expected = Some(31);
        let actual = p1(include_str!("test.txt").lines().map(String::from).collect());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_p2() {
        let expected = Some(29);
        let actual = p2(include_str!("test.txt").lines().map(String::from).collect());
        assert_eq!(actual, expected);
    }
}
