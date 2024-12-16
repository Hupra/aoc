use aoc_lib::init;
use rustlind_lib::*;
use std::i32;
use std::{collections::HashSet, time::Instant, usize};

fn p1(lines: Vec<String>) -> usize {
    let m: Vec<Vec<char>> = lines.into_iter().map(|l| l.chars().collect()).collect();
    let d = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut g: Graph<(usize, usize), i32> = Graph::new();

    for i in 0..m.len() {
        for j in 0..m[i].len() {
            let cc = m[i][j];

            match cc {
                '.' => {
                    for (ni, nj) in valid_positions((i, j), &m, d) {
                        // '.' -> '.'
                        if m[ni][nj] == cc {
                            g.add_edge((i, j), (ni, nj), 1);
                        }
                    }
                }
                '>' => {
                    g.add_edge((i, j - 1), (i, j), 1);
                    g.add_edge((i, j), (i, j + 1), 1);
                }
                '<' => {
                    g.add_edge((i, j + 1), (i, j), 1);
                    g.add_edge((i, j), (i, j - 1), 1);
                }
                '^' => {
                    g.add_edge((i + 1, j), (i, j), 1);
                    g.add_edge((i, j), (i - 1, j), 1);
                }
                'v' => {
                    g.add_edge((i - 1, j), (i, j), 1);
                    g.add_edge((i, j), (i + 1, j), 1);
                }
                _ => {}
            }
        }
    }

    let s: (usize, usize) = (0, 1);
    let t: (usize, usize) = (m.len() - 1, m[0].len() - 2);

    fn dfs(
        a: (usize, usize),
        mut v: HashSet<(usize, usize)>,
        g: &Graph<(usize, usize), i32>,
        t: (usize, usize),
    ) -> usize {
        if a == t {
            return 0;
        }
        v.insert(a);

        let mut best: usize = 0;

        for e in g.adj.get(&a).unwrap() {
            if !v.contains(&e.b) {
                best = best.max(e.c as usize + dfs(e.b, v.clone(), g, t))
            }
        }
        best
    }

    dfs(s, HashSet::new(), &g, t)
}

fn p2(lines: Vec<String>) -> i32 {
    let m: Vec<Vec<char>> = lines.into_iter().map(|l| l.chars().collect()).collect();
    let d = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut g: Graph<(usize, usize), i32> = Graph::new();

    for i in 0..m.len() {
        for j in 0..m[i].len() {
            if m[i][j] != '#' {
                for (ni, nj) in valid_positions((i, j), &m, d) {
                    // '.' -> '.'
                    if m[ni][nj] != '#' {
                        g.add_edge((i, j), (ni, nj), 1);
                    }
                }
            }
        }
    }

    let s: (usize, usize) = (0, 1);
    let t: (usize, usize) = (m.len() - 1, m[0].len() - 2);

    return g.longest_path(s, t).0;

    // really fast longest path algorithm, only works on <= 64 nodes

    // fn longest_path_64(a: u64, mut v: u64, g: &Graph<u64, i32>, t: u64) -> i32 {
    //     if a == t {
    //         return 0;
    //     }
    //     v += a;

    //     let mut best: i32 = i32::MIN;

    //     let adj = g.adj.get(&a);
    //     if let Some(bs) = adj {
    //         for e in bs {
    //             if e.b & v == 0 {
    //                 best = best.max(e.c + longest_path_64(e.b, v, g, t))
    //             }
    //         }
    //     }
    //     best
    // }

    // let speed = g.shrink(s).speedify();
    // let ng = speed.0;
    // let i_t = speed.1;
    // let t_i = speed.2;
    // dbg!(i_t.len());

    // let g_u64 = ng.map_graph(|x| 1u64 << x);

    // let ns = 1u64 << t_i.get(&s).unwrap();
    // let nt = 1u64 << t_i.get(&t).unwrap();

    // let res = longest_path_64(ns, 0, &g_u64, nt);

    // res
}

fn main() {
    let lines = init(2023, 23);
    let timer = Instant::now();
    println!("{:?} | time: {:?}", p1(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p2(lines.clone()), timer.elapsed());
}
