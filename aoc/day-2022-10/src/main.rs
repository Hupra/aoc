use aoc_lib::{ init, print_matrix };
use cached::proc_macro::cached;
use itertools::Itertools;
use rustlind_lib::*;
use std::{ time::Instant, vec };

fn gather_instructions(lines: Vec<String>) -> Vec<(String, i32)> {
    let instructions = lines
        .into_iter()
        .flat_map(|line| {
            let (a, b) = line
                .split_whitespace()
                .next_tuple()
                .unwrap_or_else(|| ("noop", "0"));
            let b = b.parse::<i32>().unwrap_or_default();
            if a == "noop" {
                vec![(a.to_string(), b)]
            } else {
                vec![(a.to_string(), 0), (a.to_string(), b)]
            }
        })
        .collect_vec();
    instructions
}

fn p1(lines: Vec<String>) -> Option<i32> {
    let instructions = gather_instructions(lines);

    let mut cycle_sum: i32 = 1;
    let mut res_sum: i32 = 0;
    for (i, cycle) in instructions.iter().enumerate() {
        let i = (i + 1) as i32;
        if (i + 20) % 40 == 0 {
            res_sum += i * cycle_sum;
        }
        cycle_sum += cycle.1;
    }

    Some(res_sum)
}

fn p2(lines: Vec<String>) -> Option<usize> {
    let instructions = gather_instructions(lines);
    let img_max_j = 40;
    let img_max_i = instructions.len() / img_max_j;

    let mut img = vec![vec!['.'; img_max_j]; img_max_i];

    let mut x: i32 = 1;
    for (i, cycle) in instructions.iter().enumerate() {
        // let i = (i + 1) as i32;
        let n_cycle = i / img_max_j;
        let n_position = i % img_max_j;

        let paint = if ((n_position as i32) - x).abs() < 2 { '#' } else { '.' };
        img[n_cycle][n_position] = paint;

        x += cycle.1;
    }

    println!("{:?}", vec!['.'; 3]);

    print_matrix(&img);
    // EZFPRAKL
    // # # # # . # # # # . # # # # . # # # . . # # # . . . # # . . # . . # . # . . . .
    // # . . . . . . . # . # . . . . # . . # . # . . # . # . . # . # . # . . # . . . .
    // # # # . . . . # . . # # # . . # . . # . # . . # . # . . # . # # . . . # . . . .
    // # . . . . . # . . . # . . . . # # # . . # # # . . # # # # . # . # . . # . . . .
    // # . . . . # . . . . # . . . . # . . . . # . # . . # . . # . # . # . . # . . . .
    // # # # # . # # # # . # . . . . # . . . . # . . # . # . . # . # . . # . # # # # .
    None
}

fn main() {
    let lines = init(2022, 10);
    let timer = Instant::now();
    println!("{:?} | time: {:?}", p1(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p2(lines.clone()), timer.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let expected = Some(13140);
        let actual = p1(include_str!("test.txt").lines().map(String::from).collect());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_p2() {
        let expected = None;
        let actual = p2(include_str!("test.txt").lines().map(String::from).collect());
        assert_eq!(actual, expected);
    }
}
