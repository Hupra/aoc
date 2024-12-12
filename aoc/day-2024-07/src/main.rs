use aoc_lib::init;
use rustlind_lib::*;
use std::time::Instant;

fn rec(nums: &[usize], acc: usize, goal: usize) -> bool {
    if acc == goal && nums.len() == 0 {
        return true;
    }
    if nums.len() == 0 {
        return false;
    }
    if acc > goal {
        return false;
    }

    let cur: usize = nums[0];
    return rec(&nums[1..], acc + cur, goal) || rec(&nums[1..], acc * cur, goal);
}

fn p1(lines: Vec<String>) -> usize {
    let mut sum = 0;
    for line in lines {
        let nums = re_nums_usize(&line);
        let goal: usize = nums[0];
        let nums = &nums[1..];

        if rec(nums, 0, goal) {
            sum += goal;
        }
    }
    sum
}

fn rec2(nums: &[usize], acc: usize, goal: usize) -> bool {
    if acc == goal && nums.len() == 0 {
        return true;
    }
    if nums.len() == 0 {
        return false;
    }
    if acc > goal {
        return false;
    }

    let cur: usize = nums[0];
    let combined = format!("{acc}{cur}").parse::<usize>().unwrap();

    return rec2(&nums[1..], acc + cur, goal)
        || rec2(&nums[1..], acc * cur, goal)
        || rec2(&nums[1..], combined, goal);
}

fn p2(lines: Vec<String>) -> usize {
    let mut sum = 0;
    for line in lines {
        let nums = re_nums_usize(&line);
        let goal: usize = nums[0];
        let nums = &nums[1..];

        if rec2(nums, 0, goal) {
            sum += goal;
        }
    }
    sum
}

fn main() {
    let lines = init(2024, 7);
    let timer = Instant::now();
    println!("{:?} | time: {:?}", p1(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p2(lines.clone()), timer.elapsed());
}
