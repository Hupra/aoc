use aoc_lib::init;
use rustlind_lib::*;
use std::{collections::HashMap, time::Instant};

fn rec(num: usize, rounds_left: usize, memo: &mut HashMap<(usize, usize), usize>) -> usize {
    if rounds_left == 0 {
        return 1;
    }
    let key = (num, rounds_left);
    if let Some(res) = memo.get(&key) {
        return *res;
    }
    if num == 0 {
        let res = rec(1, rounds_left - 1, memo);
        memo.insert(key, res);
        return res;
    }

    let num_string = num.to_string();

    if num_string.len() % 2 == 1 {
        let res = rec(num * 2024, rounds_left - 1, memo);
        memo.insert(key, res);
        return res;
    }

    let l = &num_string[..num_string.len() / 2].parse::<usize>().unwrap();
    let r = &num_string[num_string.len() / 2..].parse::<usize>().unwrap();

    let res = rec(*l, rounds_left - 1, memo) + rec(*r, rounds_left - 1, memo);
    memo.insert(key, res);
    res
}

fn p1(lines: Vec<String>) -> usize {
    let nums = re_nums_usize(&lines[0]);

    let mut sum = 0;
    let mut memo: HashMap<(usize, usize), usize> = HashMap::new();

    for num in nums {
        sum += rec(num, 25, &mut memo)
    }
    sum
}

fn p2(lines: Vec<String>) -> usize {
    let nums = re_nums_usize(&lines[0]);

    let mut sum = 0;
    let mut memo: HashMap<(usize, usize), usize> = HashMap::new();

    for num in nums {
        sum += rec(num, 75, &mut memo)
    }
    sum
}

fn main() {
    let lines = init(2024, 11);
    let timer = Instant::now();
    println!("{:?} | time: {:?}", p1(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p2(lines.clone()), timer.elapsed());
}
