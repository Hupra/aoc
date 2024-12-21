use aoc_lib::init;
use itertools::Itertools;
use rustlind_lib::*;
use std::{time::Instant, usize};

fn run_program(mut a: usize, mut b: usize, mut c: usize, program: &Vec<usize>) -> Vec<usize> {
    let mut i = 0;
    let mut output: Vec<usize> = Vec::new();

    while i < program.len() {
        let opcode = program[i];
        let operand = program[i + 1];

        let combo = match operand {
            4 => a,
            5 => b,
            6 => c,
            _ => operand,
        };

        match opcode {
            0 => a /= 2_usize.pow(combo as u32),
            1 => b ^= operand,
            2 => b = combo % 8,
            3 if a != 0 => {
                i = operand;
                continue;
            }
            4 => b ^= c,
            5 => output.push(combo % 8),
            6 => b = a / 2_usize.pow(combo as u32),
            7 => c = a / 2_usize.pow(combo as u32),
            _ => {}
        }

        i += 2;
    }

    output
}

fn p1(lines: Vec<String>) -> String {
    let a: usize = re_nums(&lines[0])[0] as usize;
    let b: usize = re_nums(&lines[1])[0] as usize;
    let c: usize = re_nums(&lines[2])[0] as usize;
    let program: Vec<usize> = re_nums(&lines[4]).into_iter().map(|i| i as usize).collect();

    run_program(a, b, c, &program).iter().join(",")
}

fn p2(lines: Vec<String>) -> Option<usize> {
    let program: Vec<usize> = re_nums(&lines[4]).into_iter().map(|i| i as usize).collect();

    fn rec(a: usize, i: usize, program: &Vec<usize>) -> Option<usize> {
        let output = run_program(a, 0, 0, program);
        if program == &output {
            return Some(a);
        }
        if program[i..] == output || i == program.len() {
            return (0..8).filter_map(|n| rec(8 * a + n, i - 1, program)).min();
        }
        None
    }
    for n in 0..8 {
        println!("{:?}", run_program(3 * 8 + n, 0, 0, &program));
    }

    rec(0, program.len(), &program)
}

fn main() {
    let lines = init(2024, 17);
    let timer = Instant::now();
    println!("{:?} | time: {:?}", p1(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p2(lines.clone()), timer.elapsed());
}
