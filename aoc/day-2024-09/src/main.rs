use aoc_lib::init;
use itertools::Itertools;
use regex::Regex;
use rustlind_lib::*;
use std::{time::Instant, u32}; // Required for chunks

fn p1(lines: Vec<String>) -> usize {
    let mut sum: usize = 0;
    for line in lines {
        let mut s: Vec<char> = line
            .chars()
            .chunks(2)
            .into_iter()
            .enumerate()
            .map(|(i, chunk)| {
                let ab: Vec<char> = chunk.collect();
                let a = ab[0].to_digit(10).unwrap() as usize;
                let mut r = vec![char::from_u32(i as u32).unwrap(); a];
                if ab.len() == 2 {
                    let b = ab[1].to_digit(10).unwrap() as usize;
                    r.extend(vec![char::MAX; b as usize]);
                }
                r
            })
            .flat_map(|vc| vc)
            .collect::<Vec<char>>();

        let mut i: usize = 0;
        let mut ri: usize = s.len() - 1;

        while i <= ri {
            if s[i] != char::MAX {
                i += 1;
                continue;
            }
            if s[ri] == char::MAX {
                ri -= 1;
                continue;
            }
            s[i] = s[ri];
            s[ri] = char::MAX;
        }

        for (i, c) in s.iter().enumerate() {
            if *c != char::MAX {
                sum += i * *c as u32 as usize;
            }
        }
    }

    sum
}

fn p2(lines: Vec<String>) -> usize {
    let mut sum: usize = 0;
    for line in lines {
        let mut s: Vec<char> = line
            .chars()
            .chunks(2)
            .into_iter()
            .enumerate()
            .map(|(i, chunk)| {
                let ab: Vec<char> = chunk.collect();
                let a = ab[0].to_digit(10).unwrap() as usize;
                let mut r = vec![char::from_u32(i as u32).unwrap(); a];
                if ab.len() == 2 {
                    let b = ab[1].to_digit(10).unwrap() as usize;
                    r.extend(vec!['.'; b as usize]);
                }
                r
            })
            .flat_map(|vc| vc)
            .collect::<Vec<char>>();

        let ts = s.iter().collect::<String>();

        let re = Regex::new(r"\u{10ffff}+").unwrap();
        let mut dots: Vec<(usize, usize)> =
            re.find_iter(&ts).map(|m| (m.start(), m.end())).collect();

        let re = Regex::new(r"[^\u{10FFFF}]+").unwrap();
        let mut not_dots: Vec<(usize, usize)> =
            re.find_iter(&ts).map(|m| (m.start(), m.end())).collect();

        not_dots.reverse();

        for (i, n_dot) in not_dots.iter().enumerate() {
            let n_dot_len = n_dot.1 - n_dot.0;

            for (j, dot) in dots.iter().enumerate() {
                let dot_len = dot.1 - dot.0;
                // let char = ts.chars().nth(i.0).unwrap();
                if n_dot_len <= dot_len {
                    // for x in i.0..i.1 {
                    //     s[x] = char::MAX;
                    // }
                    // for x in j.0..j.0 + i_len {
                    //     s[x] = 'x';
                    // }

                    println!("{:?} {:?}", dot, n_dot);

                    dots.remove(j);

                    break;
                }
            }
        }

        dbg!(s.len());
        dbg!(ts.len());

        for (i, c) in s.iter().enumerate() {
            if *c != char::MAX {
                sum += i * *c as u32 as usize;
            }
        }
    }

    sum
}

fn main() {
    let lines = init(2024, 9);
    let timer = Instant::now();
    println!("{:?} | time: {:?}", p1(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p2(lines.clone()), timer.elapsed());
}
