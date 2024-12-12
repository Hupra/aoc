use aoc_lib::init;
use rustlind_lib::*;
use std::time::Instant;

fn p1(lines: Vec<String>) -> i32 {
    let m: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();
    let mut sum = 0;
    let mut dirss: Vec<Vec<(i32, i32)>> = vec![Vec::new(); 8];
    let xmas = ['X', 'M', 'A', 'S'];

    for x in 0..=3 as i32 {
        dirss[0].push((x, 0));
        dirss[1].push((-x, 0));
        dirss[2].push((0, x));
        dirss[3].push((0, -x));
        dirss[4].push((x, x));
        dirss[5].push((-x, x));
        dirss[6].push((x, -x));
        dirss[7].push((-x, -x));
    }

    for i in 0..m.len() {
        for j in 0..m[i].len() {
            for dirs in dirss.clone() {
                let p = valid_positions((i, j), &m, dirs);
                if p.len() == xmas.len() {
                    if (0..p.len()).all(|x| m[p[x].0][p[x].1] == xmas[x]) {
                        sum += 1
                    }
                }
            }
        }
    }

    sum
}

fn p1v2(lines: Vec<String>) -> usize {
    let m: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();
    let mut sum: usize = 0;
    let mut dirss: Vec<Vec<(i32, i32)>> = Vec::new();
    let xmas: Vec<char> = "XMAS".chars().collect();

    for i in [-1, 0, 1] {
        for j in [-1, 0, 1] {
            if !(i == 0 && j == 0) {
                dirss.push((0..xmas.len() as i32).map(|x| (i * x, j * x)).collect());
            }
        }
    }

    for i in 0..m.len() {
        for j in 0..m[i].len() {
            sum += dirss
                .iter()
                .filter(|&dirs| {
                    let p = valid_positions((i, j), &m, dirs.clone());
                    p.len() == xmas.len() && (0..p.len()).all(|x| m[p[x].0][p[x].1] == xmas[x])
                })
                .count();
        }
    }
    sum
}

fn p2(lines: Vec<String>) -> i32 {
    let m: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();
    let mut sum = 0;

    let mas = ['M', 'A', 'S'];

    let dirss = [
        [(1, 1), (0, 0), (-1, -1)],
        [(-1, -1), (0, 0), (1, 1)],
        [(1, -1), (0, 0), (-1, 1)],
        [(-1, 1), (0, 0), (1, -1)],
    ];

    for i in 0..m.len() {
        for j in 0..m[i].len() {
            let mut valid = 0;

            for dirs in dirss {
                let p = valid_positions((i, j), &m, dirs);
                if p.len() == mas.len() {
                    let is_match = (0..p.len()).all(|x| m[p[x].0][p[x].1] == mas[x]);
                    if is_match {
                        {
                            valid += 1;
                        }
                    }
                }
            }
            if valid == 2 {
                sum += 1;
            }
        }
    }
    sum
}

fn p2v2(lines: Vec<String>) -> i32 {
    let m: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();
    let mas = ['M', 'A', 'S'];
    let dirss = [
        [(1, 1), (0, 0), (-1, -1)],
        [(-1, -1), (0, 0), (1, 1)],
        [(1, -1), (0, 0), (-1, 1)],
        [(-1, 1), (0, 0), (1, -1)],
    ];

    let mut sum = 0;
    for i in 0..m.len() {
        for j in 0..m[i].len() {
            if dirss
                .iter()
                .filter(|&&dirs| {
                    let p = valid_positions((i, j), &m, dirs);
                    p.len() == mas.len() && (0..p.len()).all(|x| m[p[x].0][p[x].1] == mas[x])
                })
                .count()
                == 2
            {
                sum += 1;
            }
        }
    }
    sum
}

fn main() {
    let lines: Vec<String> = init(2024, 4);
    let timer = Instant::now();
    println!("{:?} | time: {:?}", p1(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p1v2(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p2(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p2v2(lines.clone()), timer.elapsed());
}
