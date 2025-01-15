use aoc_lib::init;
use cached::proc_macro::cached;
use itertools::Itertools;
use rustlind_lib::*;
use std::{ iter, time::Instant };

fn setup(lines: Vec<String>) -> (Vec<Vec<char>>, Vec<(i32, i32, i32)>) {
    let (stacks_input, moves_input) = lines
        .split(|line| line.is_empty())
        .collect_tuple()
        .unwrap();

    let moves: Vec<(i32, i32, i32)> = moves_input
        .iter()
        .map(|line| re_nums(&line.clone()).into_iter().collect_tuple().unwrap_or_default())
        .collect();

    let mut stacks: Vec<Vec<char>> = Vec::new();

    stacks_input
        .into_iter()
        .take(stacks_input.len() - 1)
        .for_each(|line| {
            line.chars()
                .into_iter()
                .enumerate()
                .for_each(|(i, c)| {
                    let stack_i = i / 4;
                    while stacks.len() <= stack_i {
                        stacks.push(Vec::new());
                    }
                    if i % 4 == 1 && c != ' ' {
                        stacks[stack_i].insert(0, c);
                    }
                });
        });

    (stacks, moves)
}

fn p1(lines: Vec<String>) -> Option<String> {
    let (mut stacks, moves) = setup(lines);

    for instruction in moves {
        let (amount, from, to) = instruction;
        let from = (from as usize) - 1;
        let to = (to as usize) - 1;

        for _ in 0..amount {
            if let Some(c) = stacks[from].pop() {
                stacks[to].push(c);
            }
        }
    }

    let msg: String = stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect();

    Some(msg)
}

fn p2(lines: Vec<String>) -> Option<String> {
    let (mut stacks, moves) = setup(lines);

    for instruction in moves {
        let (amount, from, to) = instruction;
        let from = (from as usize) - 1;
        let to = (to as usize) - 1;

        let start = stacks[from].len() - (amount as usize);
        let mut elements: Vec<char> = stacks[from].drain(start..).collect();
        stacks[to].append(&mut elements);
    }

    let msg: String = stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect();

    Some(msg)
}

fn main() {
    let lines = init(2022, 5);
    let timer = Instant::now();
    println!("{:?} | time: {:?}", p1(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p2(lines.clone()), timer.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let expected = Some(String::from("CMZ"));
        let actual = p1(include_str!("test.txt").lines().map(String::from).collect());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_p2() {
        let expected = Some(String::from("MCD"));
        let actual = p2(include_str!("test.txt").lines().map(String::from).collect());
        assert_eq!(actual, expected);
    }
}
