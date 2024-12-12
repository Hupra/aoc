use aoc_lib::init;
use rustlind_lib::valid_positions as vp;
use std::fs::File;
use std::io::Write;

fn next(pre: (usize, usize), cur: (usize, usize), sym: char) -> (usize, usize) {
    return match sym {
        '-' => {
            if cur.1 + 1 != pre.1 {
                (cur.0, cur.1 + 1)
            } else {
                (cur.0, cur.1 - 1)
            }
        }
        '|' => {
            if cur.0 + 1 != pre.0 {
                (cur.0 + 1, cur.1)
            } else {
                (cur.0 - 1, cur.1)
            }
        }
        'L' => {
            if cur.0 - 1 != pre.0 {
                (cur.0 - 1, cur.1)
            } else {
                (cur.0, cur.1 + 1)
            }
        }
        'J' => {
            if cur.0 - 1 != pre.0 {
                (cur.0 - 1, cur.1)
            } else {
                (cur.0, cur.1 - 1)
            }
        }
        '7' => {
            if cur.0 + 1 != pre.0 {
                (cur.0 + 1, cur.1)
            } else {
                (cur.0, cur.1 - 1)
            }
        }
        'F' => {
            if cur.0 + 1 != pre.0 {
                (cur.0 + 1, cur.1)
            } else {
                (cur.0, cur.1 + 1)
            }
        }
        _ => cur,
    };
}

fn test_direction(initial_pos: (usize, usize), m: &mut Vec<Vec<char>>) -> bool {
    // vec as stack
    let mut stack: Vec<(usize, usize)> = Vec::from([initial_pos]);

    while let Some(pos) = stack.pop() {
        if "#|J7LF_-S".contains(m[pos.0][pos.1]) {
            continue;
        }

        m[pos.0][pos.1] = '#';

        stack.extend(vp(pos, &m, [(-1, 0), (1, 0), (0, -1), (0, 1)]));
    }
    return false;
}

fn p1(lines: Vec<String>) -> usize {
    let m: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();
    let mut hist: Vec<(usize, usize)> = Vec::new();

    for i in 0..lines.len() {
        for j in 0..lines.len() {
            if m[i][j] == 'S' {
                let mut pre = (i, j);
                let mut pos = (i, j + 1);
                let mut sym = m[pos.0][pos.1];
                // println!("{} {:?}", sym, pos);

                hist.push(pos);

                while sym != 'S' {
                    let tmp = next(pre, pos, sym);
                    pre = pos;
                    pos = tmp;
                    sym = m[pos.0][pos.1];
                    // println!("{} {:?}", sym, tmp);
                    hist.push(pos);
                }
            }
        }
    }

    return hist.len() / 2;
}

fn p2(lines: Vec<String>) -> usize {
    let linez: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();
    let mut m: Vec<Vec<char>> = vec![vec![' '; lines[0].len() * 2]; lines.len() * 2];
    let mut hist: Vec<(usize, usize)> = Vec::new();
    let mut s: (usize, usize) = (0, 0);

    for i in 0..linez.len() {
        for j in 0..linez[0].len() {
            m[i * 2][j * 2] = '.';

            if linez[i][j] == 'S' {
                s = (i, j);
            }
        }
    }

    let mut pre = (s.0, s.1);
    let mut pos = (pre.0, pre.1 + 1);
    let mut sym = m[pos.0][pos.1];

    m[pre.0 * 2][pre.1 * 2] = linez[pre.0][pre.1];

    let mi = ((pre.0 * 2) + (pos.0 * 2)) / 2;
    let mj = ((pre.1 * 2) + (pos.1 * 2)) / 2;
    m[mi][mj] = if pre.0 != pos.0 { '|' } else { '-' };

    hist.push(pos);

    while sym != 'S' {
        m[pos.0 * 2][pos.1 * 2] = linez[pos.0][pos.1];

        let tmp = next(pre, pos, sym);

        let mi = ((tmp.0 * 2) + (pos.0 * 2)) / 2;
        let mj = ((tmp.1 * 2) + (pos.1 * 2)) / 2;

        m[mi][mj] = if tmp.0 != pos.0 { '|' } else { '-' };

        pre = pos;
        pos = tmp;
        sym = linez[pos.0][pos.1];
    }

    for i in 0..m.len() {
        test_direction((i, 0), &mut m);
        test_direction((i, m[0].len() - 1), &mut m);
    }

    for j in 0..m[0].len() {
        test_direction((0, j), &mut m);
        test_direction((m.len() - 1, j), &mut m);
    }

    let mut sum: usize = 0;

    for i in 0..m.len() {
        for j in 0..m[0].len() {
            if m[i][j] == '.' {
                sum += 1;
            }
        }
    }

    // save output
    let content = m
        .iter()
        .map(|row| row.iter().collect::<String>()) // Convert each Vec<char> to String
        .collect::<Vec<String>>()
        .join("\n");

    let mut file = File::create("/workspace/aoc/day-2023-10/output.txt")
        .ok()
        .unwrap();
    file.write_all(content.as_bytes()).ok().unwrap();

    return sum;
}

fn main() {
    let lines = init(2023, 10);
    // stuff here
    println!("{:?}", p1(lines.clone()));
    println!("{:?}", p2(lines.clone()));
}
