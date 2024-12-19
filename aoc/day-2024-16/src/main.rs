use aoc_lib::init;
use rustlind_lib::*;
use std::{collections::HashSet, path, time::Instant};

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
            let char = get_or((i, j), &m, '#');

            if char == 'S' {
                s = (i, j);
            }
            if char == 'E' {
                t = (i, j);
            }
            if char == '#' {
                continue;
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

    let (s_to_t_distance, path) = g.shortest_path((s.0, s.1, 'S'), (t.0, t.1, 'T'));

    // part 2

    // optimize the following 2 things:
    // shrink graph
    // dont add bad stuff to graph like edges to #
    // dont run dijsktra from a vertex that has already been identified as a good place to sit!

    let (hm, _) = g.dijkstra((s.0, s.1, 'S'));

    let mut places_to_sit: HashSet<(i32, i32, char)> = HashSet::new();
    places_to_sit.extend(path);

    for (spot, distance_to_spot) in hm {
        if distance_to_spot > s_to_t_distance
            || get_or((spot.0, spot.1), &m, '#') == '#'
            || places_to_sit.contains(&spot)
        {
            continue;
        }

        let (distance_to_t, path_to_t) = g.shortest_path(spot, (t.0, t.1, 'T'));

        if distance_to_spot + distance_to_t == s_to_t_distance {
            places_to_sit.extend(path_to_t);
        }
    }

    (s_to_t_distance as usize, places_to_sit.len())
}

fn main() {
    let lines = init(2024, 16);
    let timer = Instant::now();
    println!("{:?} | time: {:?}", solve(lines.clone()), timer.elapsed());
}
