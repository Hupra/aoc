use aoc_lib::init;
use rustlind_lib::*;
use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

fn find_area_perimeter_fences(
    start: (usize, usize),
    m: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
) -> (usize, usize, usize) {
    let mut area = 0;
    let mut perimeter = 0;
    let mut fences = 0;
    let mut stack: Vec<(usize, usize)> = Vec::from([start]);

    let zone = m[start.0][start.1];

    while let Some(point) = stack.pop() {
        if visited[point.0][point.1] {
            continue;
        }

        visited[point.0][point.1] = true;

        let neigbors = valid_positions(point, &m, [(1, 0), (-1, 0), (0, 1), (0, -1)]);
        let mut perimeter_increase = 4 - neigbors.len();

        for neighbor in neigbors {
            if m[neighbor.0][neighbor.1] != zone {
                perimeter_increase += 1;
            } else {
                stack.push(neighbor);
            }
        }

        area += 1;
        perimeter += perimeter_increase;

        // for part 2

        // side, side
        let top_left = (-1, -1);
        let top_right = (-1, 1);
        let down_right = (1, 1);
        let down_left = (1, -1);
        let side_side_diag = [top_left, top_right, down_right, down_left].map(|(di, dj)| {
            (
                (point.0 as i32 + di, point.1 as i32),
                (point.0 as i32, point.1 as i32 + dj),
                (point.0 as i32 + di, point.1 as i32 + dj),
            )
        });

        for sides in side_side_diag {
            let side_0 = get_or(sides.0, m, char::MAX);
            let side_1 = get_or(sides.1, m, char::MAX);
            let diag = get_or(sides.2, m, char::MAX);

            // simple case point(1,1) = A ->
            //  ...
            //  AA. <-
            //  AA.
            if zone != side_0 && zone != side_1 {
                fences += 1;
            }

            // other case point(2,2) = A ->
            //  ..A
            //  ..A
            //  AAA <-
            if zone == side_0 && zone == side_1 && zone != diag {
                fences += 1;
            }
        }
    }

    (area, perimeter, fences)
}

fn solve(lines: Vec<String>) -> (usize, usize) {
    let m: Vec<Vec<char>> = lines
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();

    let mut p1 = 0;
    let mut p2 = 0;

    let mut visited: Vec<Vec<bool>> = vec![vec![false; m[0].len()]; m.len()];

    for i in 0..m.len() {
        for j in 0..m[i].len() {
            if visited[i][j] == false {
                let (area, perimeter, fences) =
                    find_area_perimeter_fences((i, j), &m, &mut visited);
                p1 += area * perimeter;
                p2 += area * fences;
            }
        }
    }

    (p1, p2)
}

fn main() {
    let lines = init(2024, 12);
    let timer = Instant::now();
    println!("{:?} | time: {:?}", solve(lines.clone()), timer.elapsed());
}
