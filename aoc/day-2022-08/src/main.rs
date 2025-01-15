use aoc_lib::init;
use cached::proc_macro::cached;
use itertools::Itertools;
use rustlind_lib::*;
use std::time::Instant;

fn p1(lines: Vec<String>) -> Option<usize> {
    let m = lines
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap_or_default() as usize)
                .collect_vec()
        })
        .collect_vec();

    let mut tree_count = m.len() * 2 + m[0].len() * 2 - 4;
    dbg!(tree_count);

    for i in 1..m.len() - 1 {
        for j in 1..m[0].len() - 1 {
            let x = m[i][j];
            let mut i_minus_valid = true;
            let mut i_plus_valid = true;
            let mut j_minus_valid = true;
            let mut j_plus_valid = true;
            for ii in 0..i {
                if m[ii][j] >= x {
                    i_minus_valid = false;
                    break;
                }
            }
            for ii in i + 1..m.len() {
                if m[ii][j] >= x {
                    i_plus_valid = false;
                    break;
                }
            }
            for jj in 0..j {
                if m[i][jj] >= x {
                    j_minus_valid = false;
                    break;
                }
            }
            for jj in j + 1..m[0].len() {
                if m[i][jj] >= x {
                    j_plus_valid = false;
                    break;
                }
            }
            if i_minus_valid || i_plus_valid || j_minus_valid || j_plus_valid {
                tree_count += 1;
            }
        }
    }

    Some(tree_count)
}

fn p2(lines: Vec<String>) -> Option<usize> {
    let m = lines
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap_or_default() as usize)
                .collect_vec()
        })
        .collect_vec();

    let mut best: usize = 0;

    for i in 0..m.len() {
        for j in 0..m[0].len() {
            let x = m[i][j];
            let mut i_minus_valid = 0;
            let mut i_plus_valid = 0;
            let mut j_minus_valid = 0;
            let mut j_plus_valid = 0;
            for ii in (0..i).rev() {
                i_minus_valid += 1;
                if m[ii][j] >= x {
                    break;
                }
            }
            for ii in i + 1..m.len() {
                i_plus_valid += 1;
                if m[ii][j] >= x {
                    break;
                }
            }
            for jj in (0..j).rev() {
                j_minus_valid += 1;
                if m[i][jj] >= x {
                    break;
                }
            }
            for jj in j + 1..m[0].len() {
                j_plus_valid += 1;
                if m[i][jj] >= x {
                    break;
                }
            }
            best = best.max(i_minus_valid * i_plus_valid * j_minus_valid * j_plus_valid);
        }
    }

    Some(best)
}

fn main() {
    let lines = init(2022, 8);
    let timer = Instant::now();
    println!("{:?} | time: {:?}", p1(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p2(lines.clone()), timer.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let expected = Some(21);
        let actual = p1(include_str!("test.txt").lines().map(String::from).collect());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_p2() {
        let expected = Some(8);
        let actual = p2(include_str!("test.txt").lines().map(String::from).collect());
        assert_eq!(actual, expected);
    }
}
