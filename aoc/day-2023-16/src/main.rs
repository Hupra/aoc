use std::collections::HashSet;

use aoc_lib::init;

fn movex(
    i: i32,
    j: i32,
    oi: i32,
    oj: i32,
    m: &Vec<Vec<char>>,
    r: &mut Vec<Vec<char>>,
    memo: &mut HashSet<(i32, i32, i32, i32)>,
) {
    if !memo.insert((i, j, oi, oj)) {
        return;
    }

    if i < 0 || j < 0 || i >= m.len() as i32 || j >= m[0].len() as i32 {
        return;
    }

    r[i as usize][j as usize] = '#';

    if m[i as usize][j as usize] == '.' {
        movex(i + i - oi, j + j - oj, i, j, m, r, memo);
        return;
    }

    let ic = i - oi;
    let jc = j - oj;

    // move horizontal
    if j != oj {
        match m[i as usize][j as usize] {
            '|' => {
                movex(i - 1, j, i, j, m, r, memo);
                movex(i + 1, j, i, j, m, r, memo);
                return;
            }
            '-' => {
                movex(i + ic, j + jc, i, j, m, r, memo);
                return;
            }
            '\\' => {
                movex(i + jc, j, i, j, m, r, memo);
                return;
            }
            '/' => {
                movex(i - jc, j, i, j, m, r, memo);
                return;
            }
            _ => {}
        }
    }
    // move vertical
    if i != oi {
        match m[i as usize][j as usize] {
            '-' => {
                movex(i, j - 1, i, j, m, r, memo);
                movex(i, j + 1, i, j, m, r, memo);
                return;
            }
            '|' => {
                movex(i + ic, j + jc, i, j, m, r, memo);
                return;
            }
            '\\' => {
                movex(i, j + ic, i, j, m, r, memo);
                return;
            }
            '/' => {
                movex(i, j - ic, i, j, m, r, memo);
                return;
            }
            _ => {}
        }
    }
}

fn sum_matrix(m: &Vec<Vec<char>>) -> usize {
    let mut sum: usize = 0;
    for i in 0..m.len() {
        for j in 0..m[0].len() {
            if m[i][j] == '#' {
                sum += 1;
            }
        }
    }
    sum
}

fn p1(lines: Vec<String>) -> usize {
    let m: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let mut r: Vec<Vec<char>> = vec![vec!['.'; lines[0].len()]; lines.len()];
    let mut memo: HashSet<(i32, i32, i32, i32)> = HashSet::new();

    movex(0, 0, 0, -1, &m, &mut r, &mut memo);

    return sum_matrix(&r);
}

fn p2(lines: Vec<String>) -> usize {
    let mut bestsum: usize = 0;

    let m: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let rows: i32 = m.len() as i32;
    let cols: i32 = m[0].len() as i32;

    for j in 0..cols {
        // top -> bottom
        let mut memo: HashSet<(i32, i32, i32, i32)> = HashSet::new();
        let mut r: Vec<Vec<char>> = vec![vec!['.'; lines[0].len()]; lines.len()];
        movex(0, j, -1, j, &m, &mut r, &mut memo);
        bestsum = bestsum.max(sum_matrix(&r));

        // bottom -> top
        let mut r: Vec<Vec<char>> = vec![vec!['.'; lines[0].len()]; lines.len()];
        let mut memo: HashSet<(i32, i32, i32, i32)> = HashSet::new();
        movex(rows - 1, j, rows, j, &m, &mut r, &mut memo);
        bestsum = bestsum.max(sum_matrix(&r));
    }

    for i in 0..rows {
        // left -> right
        let mut r: Vec<Vec<char>> = vec![vec!['.'; lines[0].len()]; lines.len()];
        let mut memo: HashSet<(i32, i32, i32, i32)> = HashSet::new();
        movex(i, 0, i, -1, &m, &mut r, &mut memo);
        bestsum = bestsum.max(sum_matrix(&r));

        // right -> left
        let mut r: Vec<Vec<char>> = vec![vec!['.'; lines[0].len()]; lines.len()];
        let mut memo: HashSet<(i32, i32, i32, i32)> = HashSet::new();
        movex(i, cols - 1, i, cols, &m, &mut r, &mut memo);
        bestsum = bestsum.max(sum_matrix(&r));
    }

    return bestsum;
}

fn main() {
    let lines = init(2023, 16);
    // stuff here
    println!("{:?}", p1(lines.clone()));
    println!("{:?}", p2(lines.clone()));
}
