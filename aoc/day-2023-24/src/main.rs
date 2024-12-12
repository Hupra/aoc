use aoc_lib::init;
use rustlind_lib::*;
use std::{time::Instant, usize};

fn find_intersection(m1: f64, b1: f64, m2: f64, b2: f64) -> (f64, f64) {
    if m1 == m2 {
        return (0., 0.);
    }

    let x = (b2 - b1) / (m1 - m2);
    let y = m1 * x + b1;
    return (x, y);
}

fn p1(lines: Vec<String>) -> usize {
    let data: Vec<(f64, f64, f64, f64, f64, f64)> = lines
        .iter()
        .map(|l| {
            let ab = fsplit(l, " @ ");
            let a: Vec<f64> = ab[0].split(", ").map(|s| s.parse().unwrap()).collect();
            let b: Vec<f64> = ab[1].split(", ").map(|s| s.parse().unwrap()).collect();
            (a[0], a[1], a[2], b[0], b[1], b[2])
        })
        .collect();

    let minn: f64 = 200000000000000.0;
    let maxx: f64 = 400000000000000.0;
    let mut counter: usize = 0;

    for i in 0..data.len() {
        for j in i..data.len() {
            if i == j {
                continue;
            }
            let (x1, y1, _, dx1, dy1, _) = data[i];
            let m1 = dy1 / dx1;
            let b1 = y1 - m1 * x1;

            let (x2, y2, _, dx2, dy2, _) = data[j];
            let m2 = dy2 / dx2;
            let b2 = y2 - m2 * x2;

            let (x, y) = find_intersection(m1, b1, m2, b2);

            if maxx >= x && x >= minn && maxx >= y && y >= minn {
                if ((x1 <= x && dx1 >= 0.) || (x1 >= x && dx1 <= 0.))
                    && ((x2 <= x && dx2 >= 0.) || (x2 >= x && dx2 <= 0.))
                {
                    counter += 1
                }
            }
        }
    }

    counter
}

fn p2(lines: Vec<String>) -> usize {
    let data: Vec<(f64, f64, f64, f64, f64, f64)> = lines
        .iter()
        .map(|l| {
            let ab = fsplit(l, " @ ");
            let a: Vec<f64> = ab[0].split(", ").map(|s| s.parse().unwrap()).collect();
            let b: Vec<f64> = ab[1].split(", ").map(|s| s.parse().unwrap()).collect();
            (a[0], a[1], a[2], b[0], b[1], b[2])
        })
        .collect();

    let mut x: Vec<f64> = (-1000..1000).map(|r| r as f64).collect();
    let mut y: Vec<f64> = (-1000..1000).map(|r| r as f64).collect();
    let mut z: Vec<f64> = (-1000..1000).map(|r| r as f64).collect();

    for i in 0..data.len() {
        let (x1, y1, z1, dx1, dy1, dz1) = data[i];
        for j in (i + 1)..data.len() {
            let (x2, y2, z2, dx2, dy2, dz2) = data[j];

            if dx1 == dx2 {
                x = x
                    .into_iter()
                    .filter(|&r| r != dx1 && (x2 - x1) % (r - dx1) == 0.)
                    .collect();
            }
            if dy1 == dy2 {
                y = y
                    .into_iter()
                    .filter(|&r| r != dy1 && (y2 - y1) % (r - dy1) == 0.)
                    .collect();
            }
            if dz1 == dz2 {
                z = z
                    .into_iter()
                    .filter(|&r| r != dz1 && (z2 - z1) % (r - dz1) == 0.)
                    .collect();
            }
        }
    }

    println!("{:?}, {:?}, {:?}", &x, &y, &z);
    let (r_dx, r_dy, r_dz) = (x[0], y[0], z[0]);

    // slope and bias for a point moved one back
    let (x1, y1, z1, dx1, dy1, dz1) = data[0];
    let m1 = (dy1 - r_dy) / (dx1 - r_dx);
    let b1 = y1 - m1 * x1;

    // slope and bias for a point moved one back
    let (x2, y2, _z2, dx2, dy2, _dz2) = data[1];
    let m2 = (dy2 - r_dy) / (dx2 - r_dx);
    let b2 = y2 - m2 * x2;

    let (x, y) = find_intersection(m1, b1, m2, b2);
    let t = (x - x1) / (dx1 - r_dx);
    let z = z1 + (dz1 - r_dz) * t;

    (x + y + z) as usize
}

fn main() {
    let lines = init(2023, 24);
    let timer = Instant::now();
    println!("{:?} | time: {:?}", p1(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p2(lines.clone()), timer.elapsed());
}
