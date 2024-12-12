use aoc_lib::init;
use day_2024_06::{p1gpt, p2gpt};
use rustlind_lib::*;
use std::{collections::HashSet, time::Instant};

fn p1(lines: Vec<String>) -> usize {
    let mut m: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();

    let mut start: (i32, i32) = (0, 0);

    'outer: for i in 0..m.len() {
        for j in 0..m[i].len() {
            if m[i][j] == '^' {
                start = (i as i32, j as i32);
                break 'outer;
            }
        }
    }

    let mut pos: (i32, i32) = start;
    'freedom: loop {
        //
        // find dir
        let dir: (i32, i32) = match m[pos.0 as usize][pos.1 as usize] {
            '^' => (-1, 0),
            'v' => (1, 0),
            '<' => (0, -1),
            '>' => (0, 1),
            _ => break,
        };

        // move in dir
        loop {
            let np = (pos.0 + dir.0, pos.1 + dir.1);

            if np.0 < 0 || np.0 >= m.len() as i32 || np.1 < 0 || np.1 >= m[0].len() as i32 {
                break 'freedom;
            }

            if m[np.0 as usize][np.1 as usize] != '#' {
                m[np.0 as usize][np.1 as usize] = m[pos.0 as usize][pos.1 as usize];
                m[pos.0 as usize][pos.1 as usize] = 'X';
                pos = np;
            } else {
                break;
            }
        }

        match m[pos.0 as usize][pos.1 as usize] {
            '^' => m[pos.0 as usize][pos.1 as usize] = '>',
            'v' => m[pos.0 as usize][pos.1 as usize] = '<',
            '<' => m[pos.0 as usize][pos.1 as usize] = '^',
            '>' => m[pos.0 as usize][pos.1 as usize] = 'v',
            _ => break,
        };
    }
    m[pos.0 as usize][pos.1 as usize] = 'X';

    m.into_iter()
        .flat_map(|vc| vc)
        .filter(|&c| c == 'X')
        .count()
}

fn p2(lines: Vec<String>) -> usize {
    let mut m: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();

    let om = m.clone();

    let mut start: (i32, i32) = (0, 0);

    'outer: for i in 0..m.len() {
        for j in 0..m[i].len() {
            if m[i][j] == '^' {
                start = (i as i32, j as i32);
                break 'outer;
            }
        }
    }

    let mut sum: usize = 0;

    for i in 0..m.len() {
        for j in 0..m[0].len() {
            m = om.clone();
            if m[i][j] == '.' {
                m[i][j] = '#';
                sum += p2_helper(start, &mut m);
            }
        }
    }

    // for x in m {
    //     println!("{:?}", x);
    // }

    sum
}

fn p2_helper(start: (i32, i32), m: &mut Vec<Vec<char>>) -> usize {
    let mut memo: HashSet<(usize, usize, char)> = HashSet::new();
    let mut pos: (i32, i32) = start;

    loop {
        //
        // find dir

        let symbol = m[pos.0 as usize][pos.1 as usize];

        let next_symbol = match symbol {
            '^' => '>',
            'v' => '<',
            '<' => '^',
            '>' => 'v',
            _ => return 0,
        };

        let dir: (i32, i32) = match symbol {
            '^' => (-1, 0),
            'v' => (1, 0),
            '<' => (0, -1),
            '>' => (0, 1),
            _ => return 0,
        };

        // move in dir
        loop {
            let np = (pos.0 + dir.0, pos.1 + dir.1);

            if np.0 < 0 || np.0 >= m.len() as i32 || np.1 < 0 || np.1 >= m[0].len() as i32 {
                return 0;
            }

            // check if we are in a loop
            memo.insert((pos.0 as usize, pos.1 as usize, symbol));
            if memo.contains(&(np.0 as usize, np.1 as usize, symbol)) {
                m[np.0 as usize][np.1 as usize] = '.';
                return 1;
            }
            // check if next would be wall
            if m[np.0 as usize][np.1 as usize] != '#' {
                m[pos.0 as usize][pos.1 as usize] = '.';
                m[np.0 as usize][np.1 as usize] = symbol;
                pos = np;
            } else {
                // stop moving! time to check whats next
                break;
            }
        }

        m[pos.0 as usize][pos.1 as usize] = next_symbol;
    }
}

fn main() {
    let lines = init(2024, 6);

    let timer = Instant::now();
    println!("{:?} | time: {:?}", p1(lines.clone()), timer.elapsed());
    let timer = Instant::now();
    println!("{:?} | time: {:?}", p2(lines.clone()), timer.elapsed());
    let timer = Instant::now();
    println!("{:?} | time: {:?}", p1gpt(lines.clone()), timer.elapsed());
    let timer = Instant::now();
    println!("{:?} | time: {:?}", p2gpt(lines.clone()), timer.elapsed());
}
