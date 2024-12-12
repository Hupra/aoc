use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
};

use aoc_lib::init;

fn north(m: &mut Vec<Vec<char>>) {
    let rows = m.len();
    let cols = m[0].len();

    for i in (0..rows).rev() {
        for j in 0..cols {
            if m[i][j] == 'O' {
                let mut b = i;
                for k in (0..=i).rev() {
                    if m[k][j] == '#' {
                        break;
                    }
                    if m[k][j] == '.' {
                        b = k;
                    }
                }
                m[i][j] = m[b][j];
                m[b][j] = 'O';
            }
        }
    }
}

fn south(m: &mut Vec<Vec<char>>) {
    let rows = m.len();
    let cols = m[0].len();

    for i in 0..rows {
        for j in 0..cols {
            if m[i][j] == 'O' {
                let mut b = i;
                for k in i..rows {
                    if m[k][j] == '#' {
                        break;
                    }
                    if m[k][j] == '.' {
                        b = k;
                    }
                }
                m[i][j] = m[b][j];
                m[b][j] = 'O';
            }
        }
    }
}

fn east(m: &mut Vec<Vec<char>>) {
    let rows = m.len();
    let cols = m[0].len();

    for j in 0..cols {
        for i in 0..rows {
            if m[i][j] == 'O' {
                let mut b = j;
                for k in j..cols {
                    if m[i][k] == '#' {
                        break;
                    }
                    if m[i][k] == '.' {
                        b = k;
                    }
                }
                m[i][j] = m[i][b];
                m[i][b] = 'O';
            }
        }
    }
}

fn west(m: &mut Vec<Vec<char>>) {
    let rows = m.len();
    let cols = m[0].len();

    for j in 0..cols {
        for i in 0..rows {
            if m[i][j] == 'O' {
                let mut b = j;
                for k in (0..=j).rev() {
                    if m[i][k] == '#' {
                        break;
                    }
                    if m[i][k] == '.' {
                        b = k;
                    }
                }
                m[i][j] = m[i][b];
                m[i][b] = 'O';
            }
        }
    }
}

fn p1(lines: Vec<String>) -> usize {
    let mut sum: usize = 0;
    let mut m: Vec<Vec<char>> = lines.into_iter().map(|s| s.chars().collect()).collect();
    let rows = m.len();
    let cols = m[0].len();

    north(&mut m);

    for i in 0..rows {
        for j in 0..cols {
            if m[i][j] == 'O' {
                sum += rows - i;
            }
        }
    }

    sum
}

fn p2(lines: Vec<String>) -> usize {
    let mut sum = 0;
    let mut m: Vec<Vec<char>> = lines.into_iter().map(|s| s.chars().collect()).collect();
    let rows = m.len();
    let cols = m[0].len();

    let mut rot: usize = 0;
    let max: usize = 1_000_000_000;

    let mut memo: HashMap<u64, usize> = HashMap::new();

    for i in 0..max {
        north(&mut m);
        west(&mut m);
        south(&mut m);
        east(&mut m);

        let mut hasher = DefaultHasher::new();
        m.hash(&mut hasher);
        let hash: u64 = hasher.finish();

        if let Some(val) = memo.get(&hash) {
            rot = i - val;
            rot = i + ((max - 1 - i) / rot) * rot + 1;
            println!("i: {} | rot: {}", i, rot);
            break;
        }

        memo.insert(hash, i);
    }

    for _ in rot..max {
        north(&mut m);
        west(&mut m);
        south(&mut m);
        east(&mut m);
    }

    for i in 0..rows {
        for j in 0..cols {
            if m[i][j] == 'O' {
                sum += rows - i;
            }
        }
    }

    sum
}

fn main() {
    let lines = init(2023, 14);
    // stuff here
    println!("{:?}", p1(lines.clone()));
    println!("{:?}", p2(lines.clone()));
}
