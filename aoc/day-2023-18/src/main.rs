use aoc_lib::init;
use rustlind_lib::poly_area;
use std::{cmp, iter::zip, time::Instant};

fn test_direction(inipos: (i32, i32), m: &mut Vec<Vec<char>>) {
    let mut stack: Vec<(i32, i32)> = Vec::new();
    stack.push(inipos);

    while let Some(pos) = stack.pop() {
        if pos.0 < 0 || pos.1 < 0 || pos.0 >= m.len() as i32 || pos.1 >= m[0].len() as i32 {
            continue;
        }

        if "#|J7LF_-S".contains(m[pos.0 as usize][pos.1 as usize]) {
            continue;
        }

        m[pos.0 as usize][pos.1 as usize] = '_';

        rustlind_lib::valid_positions(
            (pos.0 as usize, pos.1 as usize),
            &m,
            [(1, 0), (-1, 0), (0, 1), (0, -1)],
        )
        .iter()
        .for_each(|new_pos| stack.push((new_pos.0 as i32, new_pos.1 as i32)));
    }
}

fn p1(lines: Vec<String>) -> i32 {
    let directions: Vec<char> = lines.iter().map(|s| s.chars().next().unwrap()).collect();
    let lengths: Vec<i32> = lines
        .iter()
        .map(|s| s[2..4].trim().parse::<i32>().unwrap())
        .collect();
    let _colors: Vec<String> = lines
        .iter()
        .map(|s| {
            s.split('(')
                .last()
                .unwrap()
                .split(')')
                .next()
                .unwrap()
                .to_string()
        })
        .collect();

    let mut imin = 0;
    let mut imax = 0;
    let mut jmin = 0;
    let mut jmax = 0;
    let mut i = 0;
    let mut j = 0;

    zip(&directions, &lengths).for_each(|p| {
        match p.0 {
            'L' => j -= p.1,
            'R' => j += p.1,
            'U' => i -= p.1,
            'D' => i += p.1,
            _ => {}
        };

        imin = cmp::min(imin, i);
        jmin = cmp::min(jmin, j);
        imax = cmp::max(imax, i);
        jmax = cmp::max(jmax, j);
    });

    let rows = imin.abs() + imax + 1;
    let cols = jmin.abs() + jmax + 1;

    let mut m = vec![vec!['.'; cols as usize]; rows as usize];

    let mut i = imin.abs() as usize;
    let mut j = jmin.abs() as usize;

    m[i][j] = '#';

    zip(&directions, &lengths).for_each(|p| {
        match p.0 {
            'L' => {
                for _ in 1..=*p.1 {
                    m[i][j] = '#';
                    j -= 1;
                }
            }
            'R' => {
                for _ in 1..=*p.1 {
                    m[i][j] = '#';
                    j += 1;
                }
            }
            'U' => {
                for _ in 1..=*p.1 {
                    m[i][j] = '#';
                    i -= 1;
                }
            }
            'D' => {
                for _ in 1..=*p.1 {
                    m[i][j] = '#';
                    i += 1;
                }
            }

            _ => {}
        };
    });

    for i in 0..rows {
        test_direction((i, 0), &mut m);
        test_direction((i, m[0].len() as i32 - 1), &mut m);
    }

    for j in 0..cols {
        test_direction((0, j), &mut m);
        test_direction((m.len() as i32 - 1, j), &mut m);
    }

    let mut sum = 0;

    for i in 0..rows {
        for j in 0..cols {
            if "#.".contains(m[i as usize][j as usize]) {
                sum += 1;
            }
        }
    }

    sum
}

fn p2(lines: Vec<String>) -> usize {
    let colors: Vec<String> = lines
        .iter()
        .map(|s| {
            s.split('#')
                .last()
                .unwrap()
                .split(')')
                .next()
                .unwrap()
                .to_string()
        })
        .collect();

    // Convert hexadecimal string to decimal integer
    let lengths: Vec<i32> = colors
        .iter()
        .map(|s| i32::from_str_radix(&s[0..5], 16).unwrap())
        .collect();

    let directions: Vec<i32> = colors
        .iter()
        // .map(|s| (s.chars().nth(5).unwrap().to_digit(10).unwrap() as i32))
        .map(|s| (s.chars().nth(5).unwrap() as i32) - 48)
        .collect();

    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut i = 0;
    let mut j = 0;
    let mut lenpts = 0;

    let points: Vec<(i32, i32)> = zip(&directions, &lengths)
        .into_iter()
        .map(|(&d, &len)| {
            let dir = dirs[d as usize];
            lenpts += len;
            i += len * dir.0;
            j += len * dir.1;
            (j, i)
        })
        .collect();

    let a = poly_area(points);
    let b = lenpts as f64;

    return (a + (b / 2.0) + 1.0) as usize;
}

fn main() {
    let lines = init(2023, 18);
    let timer = Instant::now();
    println!("{:?} | time: {:?}", p1(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p2(lines.clone()), timer.elapsed());
}
