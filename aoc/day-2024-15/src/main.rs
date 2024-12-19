use aoc_lib::{init, print_matrix};
use rustlind_lib::*;
use std::{mem::swap, time::Instant};

macro_rules! tmul {
    ($t:expr, $mul:expr) => {
        ($t.0 * $mul, $t.1 * $mul)
    };
}

fn direction_char_to_tuple(c: char) -> (i32, i32) {
    match c {
        '^' => (-1, 0),
        'v' => (1, 0),
        '<' => (0, -1),
        '>' => (0, 1),
        _ => (0, 0),
    }
}

fn p1(lines: Vec<String>) -> usize {
    let input: Vec<Vec<String>> = lines
        .split(|line| line.is_empty())
        .map(|sa| sa.to_vec())
        .collect();
    let directions: Vec<char> = input[1]
        .iter()
        .flat_map(|x| x.chars().collect::<Vec<char>>())
        .collect();

    let mut m: Vec<Vec<char>> = input[0].iter().map(|s| s.chars().collect()).collect();

    let mut start = (0, 0);
    for i in 0..m.len() {
        for j in 0..m[i].len() {
            if m[i][j] == '@' {
                start = (i as i32, j as i32);
            }
        }
    }

    /////

    let mut cur = start;
    for char in directions {
        let direction = direction_char_to_tuple(char);

        let cur_plus_1 = tadd!(cur, direction);
        let mut cur_plus_x = cur_plus_1;

        loop {
            match get_or(cur_plus_x, &m, '#') {
                '#' => break,
                '.' => {
                    m_swap(cur_plus_1, cur_plus_x, &mut m);
                    m_swap(cur_plus_1, cur, &mut m);
                    cur = cur_plus_1;
                    break;
                }
                _ => {}
            }
            cur_plus_x = tadd!(cur_plus_x, direction);
        }
    }

    let mut sum = 0;

    for i in 0..m.len() {
        for j in 0..m[i].len() {
            if m[i][j] == 'O' {
                sum += 100 * i + j
            }
        }
    }

    sum
}

fn p2(lines: Vec<String>) -> usize {
    let input: Vec<Vec<String>> = lines
        .split(|line| line.is_empty())
        .map(|sa| sa.to_vec())
        .collect();
    let directions: Vec<char> = input[1]
        .iter()
        .flat_map(|x| x.chars().collect::<Vec<char>>())
        .collect();

    let mut m: Vec<Vec<char>> = input[0]
        .iter()
        .map(|s| {
            s.chars()
                .flat_map(|c| match c {
                    '@' => ['@', '.'],
                    'O' => ['[', ']'],
                    '.' => ['.', '.'],
                    '#' => ['#', '#'],
                    _ => ['#', '#'],
                })
                .collect()
        })
        .collect();

    let mut start = (0, 0);
    for i in 0..m.len() {
        for j in 0..m[i].len() {
            if m[i][j] == '@' {
                start = (i as i32, j as i32);
            }
        }
    }

    /////

    let mut cur = start;
    for char in directions {
        let direction = direction_char_to_tuple(char);
        if can_push(cur, direction, &m) {
            push(cur, direction, &mut m);
            cur = tadd!(cur, direction);
        }
    }

    let mut sum = 0;

    for i in 0..m.len() {
        for j in 0..m[i].len() {
            if m[i][j] == '[' {
                sum += 100 * i + j
            }
        }
    }
    sum
}

fn can_push(cur: (i32, i32), direction: (i32, i32), m: &Vec<Vec<char>>) -> bool {
    let cur_char = get_or(cur, &m, '#');
    if cur_char == '.' {
        return true;
    }
    if cur_char == '#' {
        return false;
    }
    if cur_char == '@' {
        return can_push(tadd!(cur, direction), direction, m);
    }
    // [ ] these two only care about their left/right neighbor if direction is <- or ->
    if direction == (0, 1) || direction == (0, -1) {
        return can_push(tadd!(cur, tmul!(direction, 2)), direction, m);
    }

    // ^ and v  for [ and ] will always have a horizontal direction and then one either left or right
    let horizontal = can_push(tadd!(cur, direction), direction, m);

    if cur_char == '[' {
        let horizontal_right = can_push(tadd!(tadd!(cur, direction), (0, 1)), direction, m);
        return horizontal && horizontal_right;
    }

    if cur_char == ']' {
        let horizontal_left = can_push(tadd!(tadd!(cur, direction), (0, -1)), direction, m);
        return horizontal && horizontal_left;
    }

    false
}

fn push(cur: (i32, i32), direction: (i32, i32), m: &mut Vec<Vec<char>>) {
    let cur_char = get_or(cur, &m, '#');

    if cur_char == '.' || cur_char == '#' {
        return;
    }

    if direction == (0, 1) || direction == (0, -1) || cur_char == '@' {
        let next = tadd!(cur, direction);
        push(next, direction, m);
        m_swap(cur, next, m);
        return;
    }

    // handle [ and ] when we come from a vertical direction
    let next_vertical = tadd!(cur, direction);
    push(next_vertical, direction, m);
    m_swap(cur, next_vertical, m);

    // if [ we also push its right neighbor
    // if ] we also push its left neighbor
    let horizontal_direction = if cur_char == '[' { (0, 1) } else { (0, -1) };
    let neighbor = tadd!(cur, horizontal_direction);
    let next_vertical_neighbor = tadd!(neighbor, direction);
    push(next_vertical_neighbor, direction, m);
    m_swap(neighbor, next_vertical_neighbor, m);

    return;
}

fn main() {
    let lines = init(2024, 15);
    let timer = Instant::now();
    println!("{:?} | time: {:?}", p1(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p2(lines.clone()), timer.elapsed());
}
