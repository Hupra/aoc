use aoc_lib::init;
use itertools::Itertools;
use rustlind_lib::*;
use std::{collections::HashMap, i32, time::Instant, usize};

fn p1(lines: Vec<String>) -> usize {
    let mut m: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    let mut s = (0, 0);
    let mut t = (0, 0);

    for i in 0..m.len() {
        for j in 0..m[i].len() {
            match m[i][j] {
                'S' => s = (i as i32, j as i32),
                'E' => t = (i as i32, j as i32),
                _ => {}
            }
        }
    }

    let mut g: Graph<(i32, i32), i32> = Graph::new();

    for i in 0..m.len() as i32 {
        for j in 0..m[i as usize].len() as i32 {
            if get_or((i, j), &m, '#') == '#' {
                continue;
            }
            for delta in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let point = tadd!((i, j), delta);
                if get_or(point, &m, '#') == '#' {
                    continue;
                }
                g.add_edge((i, j), point, 1);
            }
        }
    }

    let (from_s, _) = g.dijkstra(s);
    let (from_t, _) = g.dijkstra(t);

    let normal_distance = *from_s.get(&t).unwrap();

    let mut sum = 0;

    for i in 0..m.len() {
        for j in 0..m[i].len() {
            if m[i][j] == '#' {
                continue;
            }
            let ij = (i as i32, j as i32);
            for delta in [(2, 0), (-2, 0), (0, 2), (0, -2)] {
                let poi = tadd!(ij, delta);
                if get_or(poi, &m, '#') == '#' {
                    continue;
                }
                let dist = from_s.get(&ij).unwrap() + from_t.get(&poi).unwrap() + 2;
                if dist < normal_distance && normal_distance - dist >= 100 {
                    sum += 1;
                }
            }
        }
    }
    sum
}

fn p2_old(lines: Vec<String>) -> usize {
    let m: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    let pico = 20;
    let min_save = 100;

    let mut s = (0, 0);
    let mut t = (0, 0);

    for i in 0..m.len() {
        for j in 0..m[i].len() {
            match m[i][j] {
                'S' => s = (i as i32, j as i32),
                'E' => t = (i as i32, j as i32),
                _ => {}
            }
        }
    }

    let mut g: Graph<(i32, i32), i32> = Graph::new();

    for i in 0..m.len() as i32 {
        for j in 0..m[i as usize].len() as i32 {
            if get_or((i, j), &m, '#') == '#' {
                continue;
            }
            for delta in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let point = tadd!((i, j), delta);
                if get_or(point, &m, '#') == '#' {
                    continue;
                }
                g.add_edge((i, j), point, 1);
            }
        }
    }

    let (from_s, _) = g.dijkstra(s);
    let (from_t, _) = g.dijkstra(t);

    let normal_distance = *from_s.get(&t).unwrap();

    let mut sum = 0;

    // only used to check against example
    let mut tracker: HashMap<i32, i32> = HashMap::new();

    for i in 0..m.len() {
        for j in 0..m[i].len() {
            if m[i][j] == '#' {
                continue;
            }
            let ij = (i as i32, j as i32);

            for di in 0..=pico {
                for dj in 0..=pico {
                    if di + dj > pico {
                        continue;
                    }

                    // we must check if its horizontal so we dont double count
                    // from the "normal" dirs ie. (1, 1) and (1, -1) would
                    // yield the same result for (i,j) = (5,0) so we avoid that here
                    // by setting the element 2 and 3 to (0,0)
                    let dirs = if di == 0 || dj == 0 {
                        [(1, 1), (-1, -1), (0, 0), (0, 0)]
                    } else {
                        [(1, 1), (-1, -1), (1, -1), (-1, 1)]
                    };

                    for delta in dirs {
                        let delta_mul = (delta.0 * di, delta.1 * dj);

                        let poi = tadd!(ij, delta_mul);
                        if get_or(poi, &m, '#') == '#' {
                            continue;
                        }
                        let dist = from_s.get(&ij).unwrap() + from_t.get(&poi).unwrap() + (di + dj);
                        if dist < normal_distance && normal_distance - dist >= min_save {
                            *tracker.entry(normal_distance - dist).or_default() += 1;
                            sum += 1;
                        }
                    }
                }
            }
        }
    }

    // println!("{:?}", tracker);
    // for key in tracker.keys().sorted() {
    //     println!("{} {}", tracker.get(key).unwrap(), key);
    // }
    sum
}

fn p2(lines: Vec<String>) -> usize {
    let m: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    let pico: i32 = 20;
    let min_save: i32 = 100;

    let mut s = (0, 0);
    let mut t = (0, 0);

    // find s and t
    for i in 0..m.len() {
        for j in 0..m[i].len() {
            match m[i][j] {
                'S' => s = (i as i32, j as i32),
                'E' => t = (i as i32, j as i32),
                _ => {}
            }
        }
    }

    let mut g: Graph<(i32, i32), i32> = Graph::new();

    // build graph
    for i in 0..m.len() as i32 {
        for j in 0..m[i as usize].len() as i32 {
            if get_or((i, j), &m, '#') == '#' {
                continue;
            }
            for delta in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let point = tadd!((i, j), delta);
                if get_or(point, &m, '#') == '#' {
                    continue;
                }
                g.add_edge((i, j), point, 1);
            }
        }
    }

    // run dijstra from both sides such that
    // we can get the a nodes distance to both s and t
    let (from_s, _) = g.dijkstra(s);
    let (from_t, _) = g.dijkstra(t);

    let normal_distance = *from_s.get(&t).unwrap();

    let mut sum = 0;

    for i in 0..m.len() {
        for j in 0..m[i].len() {
            if m[i][j] == '#' {
                continue;
            }

            // look pico seconds around the current position
            for di in -pico..=pico {
                for dj in -pico..=pico {
                    let ij = (i as i32, j as i32);
                    let poi = tadd!(ij, (di, dj));
                    let moves = di.abs() + dj.abs();

                    if moves > pico {
                        continue;
                    };
                    if get_or(poi, &m, '#') == '#' {
                        continue;
                    }

                    // check the new dist using the shortcut
                    let dist = from_s.get(&ij).unwrap() + from_t.get(&poi).unwrap() + moves;
                    if dist < normal_distance && normal_distance - dist >= min_save {
                        sum += 1;
                    }
                }
            }
        }
    }

    sum
}

fn main() {
    let lines = init(2024, 20);
    let timer = Instant::now();
    println!("{:?} | time: {:?}", p1(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p2(lines.clone()), timer.elapsed());
}
