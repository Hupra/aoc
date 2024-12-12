use std::usize;

use aoc_lib::init;

pub fn print_matrix<T: std::fmt::Display>(matrix: &[Vec<T>]) {
    for row in matrix {
        let row_str = row
            .iter()
            .map(|item| format!("{}", item))
            .collect::<Vec<_>>()
            .join(" ");
        println!("{}", row_str);
    }
}

fn flip_char(matrix: &mut Vec<Vec<char>>, i: usize, j: usize) -> &mut Vec<Vec<char>> {
    if let Some(row) = matrix.get_mut(i) {
        if let Some(cell) = row.get_mut(j) {
            *cell = match *cell {
                '.' => '#',
                '#' => '.',
                _ => *cell,
            };
        }
    }
    return matrix;
}

fn turn90<T: Clone>(m: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let row_count = m.len();
    let col_count = m[0].len();

    let mut flipped: Vec<Vec<T>> = Vec::with_capacity(col_count);

    for j in 0..col_count {
        // convert col to row
        let mut new_row: Vec<T> = Vec::with_capacity(row_count);

        for i in (0..row_count).rev() {
            new_row.push(m[i][j].clone());
        }

        flipped.push(new_row);
    }

    return flipped;
}

fn solve_puzzle(puzzle: &Vec<Vec<char>>, num: usize) -> Option<usize> {
    for i in 1..puzzle.len() {
        if puzzle[i - 1] == puzzle[i] {
            let mut valid = true;

            let mut u: i32 = i as i32 - 2;
            let mut d: i32 = i as i32 + 1;

            while u >= 0 && d < puzzle.len() as i32 {
                if puzzle[u as usize] != puzzle[d as usize] {
                    valid = false;
                    break;
                }
                u -= 1;
                d += 1;
            }
            if valid && i - 1 != num {
                return Some(i - 1);
            }
        }
    }
    return None;
}

fn p1(lines: Vec<String>) -> usize {
    let mut sum: usize = 0;

    let puzzles: Vec<Vec<Vec<char>>> = lines
        .split(|line| line.is_empty())
        .map(|group| {
            group
                .into_iter()
                .map(|s| s.chars().collect::<Vec<char>>())
                .collect()
        })
        .collect();

    for puzzle in puzzles {
        match solve_puzzle(&puzzle, usize::MAX) {
            Some(r) => {
                // println!("H: {}", r);
                sum += (r + 1) * 100
            }
            _ => {
                let r = solve_puzzle(&turn90(&puzzle), usize::MAX).unwrap();
                // println!("V: {}", r);
                sum += r + 1;
            }
        }
    }

    sum
}

fn p2(lines: Vec<String>) -> usize {
    let mut sum: usize = 0;

    let puzzles: Vec<Vec<Vec<char>>> = lines
        .split(|line| line.is_empty())
        .map(|group| {
            group
                .into_iter()
                .map(|s| s.chars().collect::<Vec<char>>())
                .collect()
        })
        .collect();

    for puzzle in puzzles {
        let mut opuzzle: Vec<Vec<char>> = puzzle;
        let mut fpuzzle: Vec<Vec<char>> = turn90(&opuzzle);

        // print_matrix(&opuzzle);
        // println!("-----------");
        // print_matrix(&fpuzzle);

        let original_correct = solve_puzzle(&opuzzle, usize::MAX).is_some();
        let old_r = if original_correct {
            solve_puzzle(&opuzzle, usize::MAX)
        } else {
            solve_puzzle(&fpuzzle, usize::MAX)
        }
        .unwrap();

        let (o_comp, f_comp) = if original_correct {
            (old_r, usize::MAX)
        } else {
            (usize::MAX, old_r)
        };

        for i in 0..opuzzle.len() {
            for j in 0..opuzzle[0].len() {
                let or = solve_puzzle(&flip_char(&mut opuzzle, i, j), o_comp);
                let fr = solve_puzzle(&flip_char(&mut fpuzzle, j, i), f_comp);

                if let Some(r) = or {
                    sum += (r + 1) * 100
                }

                if let Some(r) = fr {
                    sum += r + 1
                }

                flip_char(&mut opuzzle, i, j);
                flip_char(&mut fpuzzle, j, i);
            }
        }
    }

    return sum / 2;
}

fn main() {
    let lines = init(2023, 13);
    // stuff here
    println!("{:?}", p1(lines.clone()));
    println!("{:?}", p2(lines.clone()));
}
