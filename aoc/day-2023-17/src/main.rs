use std::time::Instant;

use aoc_lib::init;
use rustlind_lib::Graph;

fn w(i: i32, j: i32, m: &Vec<Vec<i32>>) -> i32 {
    if i < 0 || j < 0 || i >= m.len() as i32 || j >= m[0].len() as i32 {
        return 0;
    }
    return m[i as usize][j as usize];
}

fn p1(lines: Vec<String>) -> i32 {
    let m: Vec<Vec<i32>> = lines
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();

    let rows = m.len();
    let cols = m[0].len();

    let mut g: Graph<String, i32> = Graph::new();

    g.add_edge("S".to_string(), "v0,0".to_string(), 0);
    g.add_edge("S".to_string(), "h0,0".to_string(), 0);

    let a1 = format!("v{},{}", rows - 1, cols - 1);
    let a2 = format!("h{},{}", rows - 1, cols - 1);
    g.add_edge(a1, "E".to_string(), 0);
    g.add_edge(a2, "E".to_string(), 0);

    for i in 0..rows as i32 {
        for j in 0..cols as i32 {
            let mut w_u = 0;
            let mut w_d = 0;
            let mut w_l = 0;
            let mut w_r = 0;
            let av = format!("v{},{}", i, j);
            let ah = format!("h{},{}", i, j);

            for x in 1..=3 as i32 {
                w_u += w(i - x, j, &m);
                w_d += w(i + x, j, &m);
                w_l += w(i, j - x, &m);
                w_r += w(i, j + x, &m);
                g.add_edge(av.clone(), format!("h{},{}", i - x, j), w_u);
                g.add_edge(av.clone(), format!("h{},{}", i + x, j), w_d);
                g.add_edge(ah.clone(), format!("v{},{}", i, j - x), w_l);
                g.add_edge(ah.clone(), format!("v{},{}", i, j + x), w_r);
            }
        }
    }

    let (res, _path) = g.dijkstra("S".to_string());

    return *res.get("E").unwrap_or(&0);
}

fn p2(lines: Vec<String>) -> i32 {
    let mut num_edges: usize = 0;

    let m: Vec<Vec<i32>> = lines
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();

    let rows = m.len();
    let cols = m[0].len();

    let mut g: Graph<String, i32> = Graph::new();

    g.add_edge("S".to_string(), "v0,0".to_string(), 0);
    g.add_edge("S".to_string(), "h0,0".to_string(), 0);

    let a1 = format!("v{},{}", rows - 1, cols - 1);
    let a2 = format!("h{},{}", rows - 1, cols - 1);
    g.add_edge(a1, "E".to_string(), 0);
    g.add_edge(a2, "E".to_string(), 0);

    for i in 0..rows as i32 {
        for j in 0..cols as i32 {
            let mut w_u = 0;
            let mut w_d = 0;
            let mut w_l = 0;
            let mut w_r = 0;
            let av = format!("v{},{}", i, j);
            let ah = format!("h{},{}", i, j);

            for x in 1..=10 as i32 {
                w_u += w(i - x, j, &m);
                w_d += w(i + x, j, &m);
                w_l += w(i, j - x, &m);
                w_r += w(i, j + x, &m);
                if x < 4 {
                    continue;
                }
                g.add_edge(av.clone(), format!("h{},{}", i - x, j), w_u);
                g.add_edge(av.clone(), format!("h{},{}", i + x, j), w_d);
                g.add_edge(ah.clone(), format!("v{},{}", i, j - x), w_l);
                g.add_edge(ah.clone(), format!("v{},{}", i, j + x), w_r);
                num_edges += 4;
            }
        }
    }

    println!("Number of edges: {}", num_edges);
    let (res, _path) = g.dijkstra("S".to_string());
    return *res.get("E").unwrap_or(&0);
}

fn main() {
    let lines = init(2023, 17);
    let timer = Instant::now();
    println!("{:?} | time: {:?}", p1(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p2(lines.clone()), timer.elapsed());
}
