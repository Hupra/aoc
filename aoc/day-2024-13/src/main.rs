use aoc_lib::init;
use rustlind_lib::*;
use std::{time::Instant, usize};

fn p1(lines: Vec<String>) -> usize {
    let machines: Vec<Vec<String>> = lines
        .split(|line| line.is_empty())
        .map(|grp| grp.to_vec())
        .collect();

    let mut sum = 0;

    for machine in machines {
        let button_a: Vec<usize> = re_nums(&machine[0])
            .into_iter()
            .map(|i| i as usize)
            .collect();
        let button_b: Vec<usize> = re_nums(&machine[1])
            .into_iter()
            .map(|i| i as usize)
            .collect();
        let prize: Vec<usize> = re_nums(&machine[2])
            .into_iter()
            .map(|i| i as usize)
            .collect();

        let button_a = (button_a[0], button_a[1]);
        let button_b = (button_b[0], button_b[1]);
        let prize = (prize[0], prize[1]);

        let mut min: usize = usize::MAX;

        for a in 0..=100 as usize {
            for b in 0..=100 as usize {
                let x = a * button_a.0 + b * button_b.0;
                let y = a * button_a.1 + b * button_b.1;
                if prize == (x, y) {
                    min = min.min((a * 3) + (b * 1));
                }
            }
        }

        dbg!(min);

        if min != usize::MAX {
            sum += min;
        }
    }

    sum
}

fn solve_for_a_b(
    a: (f64, f64),
    b: (f64, f64),
    g: (f64, f64),
) -> Result<(usize, usize), &'static str> {
    let (x1, y1) = a;
    let (x2, y2) = b;
    let (gx, gy) = g;

    //  a|x1| + b|x2| = |gx|
    //   |y1|    |y2|   |gy|

    // a * (x1, y1) + b * (x2, y2) = (gx, gy)

    // [ x1   x2 ] [ a ]   [ gx ]
    // [ y1   y2 ] [ b ] = [ gy ]
    // Compute the determinant of the coefficient matrix:
    let det = x1 * y2 - y1 * x2;

    if det.abs() < 1e-12 {
        return Err("Determinant is zero, no unique solution available");
    }

    // Calculate a and b using Cramer's rule
    let num_a = gx * y2 - gy * x2;
    let num_b = x1 * gy - y1 * gx;

    // Check if we can divide evenly
    if num_a % det != 0.0 || num_b % det != 0.0 {
        return Err("No integer solution exists");
    }

    let a_val = num_a / det;
    let b_val = num_b / det;

    Ok((a_val as usize, b_val as usize))
}

fn p2(lines: Vec<String>) -> usize {
    let machines: Vec<Vec<String>> = lines
        .split(|line| line.is_empty())
        .map(|grp| grp.to_vec())
        .collect();

    let mut sum = 0;

    for machine in machines {
        let button_a: Vec<usize> = re_nums(&machine[0])
            .into_iter()
            .map(|i| i as usize)
            .collect();
        let button_b: Vec<usize> = re_nums(&machine[1])
            .into_iter()
            .map(|i| i as usize)
            .collect();
        let prize: Vec<usize> = re_nums(&machine[2])
            .into_iter()
            .map(|i| i as usize)
            .collect();

        let bonus: usize = 10_000_000_000_000;
        let button_a = (button_a[0] as f64, button_a[1] as f64);
        let button_b = (button_b[0] as f64, button_b[1] as f64);
        let prize = ((prize[0] + bonus) as f64, (prize[1] + bonus) as f64);

        if let Ok(res) = solve_for_a_b(button_a, button_b, prize) {
            dbg!(res);
            sum += (res.0 * 3) + (res.1 * 1)
        }
    }

    sum
}

fn main() {
    let lines = init(2024, 13);
    let timer = Instant::now();
    println!("{:?} | time: {:?}", p1(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p2(lines.clone()), timer.elapsed());
}
