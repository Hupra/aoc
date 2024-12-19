use aoc_lib::{init, print_matrix};
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

fn p1(lines: Vec<String>) -> usize {
    let wide: i32 = 101;
    let tall: i32 = 103;
    let seconds = 100;
    let mut quads = [0, 0, 0, 0];
    let mut m: Vec<Vec<usize>> = vec![vec![0; wide as usize]; tall as usize];

    for line in lines {
        let input = re_nums_neg(&line);
        let p = (input[1], input[0]);
        let q = (input[3], input[2]);

        let q_100 = tmul!(q, seconds);
        let p_100 = tadd!(p, q_100);

        // put them inbound-ish
        let p_new = (p_100.0 % tall, p_100.1 % wide);

        // handle negatives
        let p_final = (
            ((tall + p_new.0) % tall) as usize,
            ((wide + p_new.1) % wide) as usize,
        );

        let half_wide = wide as usize / 2;
        let half_tall = tall as usize / 2;

        // add result to correct quadrant
        match p_final {
            (i, j) if i < half_tall && j < half_wide as usize => {
                quads[0] += 1;
            }
            (i, j) if i < half_tall && j > half_wide as usize => {
                quads[1] += 1;
            }
            (i, j) if i > half_tall && j < half_wide as usize => {
                quads[2] += 1;
            }
            (i, j) if i > half_tall && j > half_wide as usize => {
                quads[3] += 1;
            }
            _ => {}
        }

        m[p_final.0][p_final.1] += 1;
    }
    dbg!(quads);

    quads.into_iter().reduce(|a, c| a * c).unwrap() as usize
}

fn p2(lines: Vec<String>) -> usize {
    let wide: i32 = 101;
    let tall: i32 = 103;

    let mut hs: HashSet<(usize, usize)> = HashSet::new();

    let items: Vec<((i32, i32), (i32, i32))> = lines
        .into_iter()
        .map(|line| {
            let input = re_nums_neg(&line);
            let p = (input[1], input[0]);
            let q = (input[3], input[2]);
            (p, q)
        })
        .collect();

    for seconds in 0..100_000_000 {
        hs.clear();
        for (p, q) in &items {
            let q_100 = tmul!(q, seconds);
            let p_100 = tadd!(p, q_100);

            // put them inbound-ish
            let p_new = (p_100.0 % tall, p_100.1 % wide);

            // handle negatives
            let p_final = (
                ((tall + p_new.0) % tall) as usize,
                ((wide + p_new.1) % wide) as usize,
            );

            hs.insert(p_final);
        }

        if hs.len() == items.len() {
            return seconds as usize;
        }
    }

    return 0;
}

fn main() {
    let lines = init(2024, 14);
    let timer = Instant::now();
    println!("{:?} | time: {:?}", p1(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p2(lines.clone()), timer.elapsed());
}
