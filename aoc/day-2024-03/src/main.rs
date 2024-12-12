use aoc_lib::init;
use regex::Regex;
use rustlind_lib::*;
use std::time::Instant;

fn p1(lines: Vec<String>) -> usize {
    // Regex to match specific pattern:
    // 1. "mul(number,number)"
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum: usize = 0;

    for line in lines {
        for caps in re.captures_iter(&line) {
            let first_number = &caps[1].parse::<i32>().unwrap(); // First capturing group
            let second_number = &caps[2].parse::<i32>().unwrap(); // Second capturing group
            sum += (first_number * second_number) as usize
        }
    }
    sum
}

fn p1v2(lines: Vec<String>) -> usize {
    let mut sum: usize = 0;
    for line in lines {
        for caps in Regex::new(r"mul\(\d+,\d+\)").unwrap().captures_iter(&line) {
            sum += re_nums(&caps[0]).iter().product::<i32>() as usize
        }
    }
    sum
}

fn p2(lines: Vec<String>) -> usize {
    // Regex to match specific patterns:
    // 1. "mul(number,number)"
    // 2. "do()"
    // 3. "don't()"
    let re = Regex::new(r"mul\((\d+),(\d+)\)|(?:do|don't)\(\)").unwrap();
    let mut sum: usize = 0;
    let mut enabled = true;

    for line in lines {
        for caps in re.captures_iter(&line) {
            let catch = &caps[0];
            match catch {
                "do()" => {
                    enabled = true;
                }
                "don't()" => {
                    enabled = false;
                }
                _ => {
                    if enabled {
                        let first_number = &caps[1].parse::<i32>().unwrap_or_default(); // First capturing group
                        let second_number = &caps[2].parse::<i32>().unwrap_or_default();
                        sum += (first_number * second_number) as usize
                    }
                }
            }
        }
    }
    sum
}

fn main() {
    let lines = init(2024, 3);
    let timer = Instant::now();
    println!("{:?} | time: {:?}", p1(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p1v2(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p2(lines.clone()), timer.elapsed());
}
