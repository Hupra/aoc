use aoc_lib::init;
use rustlind_lib::*;
use std::{collections::HashSet, time::Instant};

fn add_edge_wrapper(
    a: (i32, i32, char),
    b: (i32, i32, char),
    c: i32,
    g: &mut Graph<(i32, i32, char), i32>,
    m: &Vec<Vec<char>>,
) {
    if get_or((b.0, b.1), m, '#') != '#' {
        g.add_edge(a, b, c);
    }
}

fn solve(lines: Vec<String>) -> (usize, usize) {
    let m: Vec<Vec<char>> = lines
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();

    let mut g: Graph<(i32, i32, char), i32> = Graph::new();

    let mut s = (0, 0);
    let mut t = (0, 0);

    for i in 0..m.len() as i32 {
        for j in 0..m[i as usize].len() as i32 {
            match get_or((i, j), &m, '#') {
                'S' => s = (i, j),
                'E' => t = (i, j),
                '#' => continue,
                _ => {}
            }
            add_edge_wrapper((i, j, '|'), (i + 1, j, '|'), 1, &mut g, &m);
            add_edge_wrapper((i, j, '|'), (i - 1, j, '|'), 1, &mut g, &m);
            add_edge_wrapper((i, j, '|'), (i, j + 1, '-'), 1_001, &mut g, &m);
            add_edge_wrapper((i, j, '|'), (i, j - 1, '-'), 1_001, &mut g, &m);

            add_edge_wrapper((i, j, '-'), (i + 1, j, '|'), 1_001, &mut g, &m);
            add_edge_wrapper((i, j, '-'), (i - 1, j, '|'), 1_001, &mut g, &m);
            add_edge_wrapper((i, j, '-'), (i, j + 1, '-'), 1, &mut g, &m);
            add_edge_wrapper((i, j, '-'), (i, j - 1, '-'), 1, &mut g, &m);
        }
    }

    add_edge_wrapper((s.0, s.1, 'S'), (s.0, s.1, '-'), 0, &mut g, &m);
    add_edge_wrapper((s.0, s.1, 'S'), (s.0, s.1, '|'), 0, &mut g, &m);
    add_edge_wrapper((t.0, t.1, '-'), (t.0, t.1, 'T'), 0, &mut g, &m);
    add_edge_wrapper((t.0, t.1, '|'), (t.0, t.1, 'T'), 0, &mut g, &m);

    let (s_to_t_distance, paths) = g.shortest_paths((s.0, s.1, 'S'), (t.0, t.1, 'T')).unwrap();

    // part 2
    let places_to_sit: HashSet<(i32, i32)> = paths
        .iter()
        .flat_map(|path| path)
        .map(|trip| (trip.0, trip.1))
        .collect();

    (s_to_t_distance as usize, places_to_sit.len())
}

fn main() {
    let lines = init(2024, 16);
    let timer = Instant::now();
    println!("{:?} | time: {:?}", solve(lines), timer.elapsed());
}
