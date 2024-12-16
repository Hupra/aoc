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

fn find_seq(s: &Vec<usize>, f: impl Fn(usize) -> bool) -> Vec<(usize, usize)> {
    let mut list: Vec<(usize, usize)> = Vec::new();
    let mut start: usize = 0;

    for i in 1..s.len() {
        let pre = s[i - 1];
        let cur = s[i];
        if f(pre) && pre != cur {
            list.push((start, i - 1));
        }
        if pre != cur {
            start = i;
        }
        if f(cur) && i == s.len() - 1 {
            list.push((start, i));
        }
    }

    list
}

// skiv part 2 helt om til at bruge et vec usize, og stop med regex, lav et rigtigt loop :(
// loop tilføj til den ene / anden side
// break din if på cur_int != last_int
fn p2(lines: Vec<String>) -> usize {
    let mut sum: usize = 0;
    for line in lines {
        let mut s: Vec<usize> = line
            .chars()
            .chunks(2)
            .into_iter()
            .enumerate()
            .map(|(i, chunk)| {
                let ab: Vec<char> = chunk.collect();
                let a = ab[0].to_digit(10).unwrap() as usize;
                //insert the numbers
                let mut r = vec![i; a];
                //insert dots (the if is for the case of the last element that doesnt have a dots numbers)
                if ab.len() == 2 {
                    let b = ab[1].to_digit(10).unwrap() as usize;
                    r.extend(vec![usize::MAX; b as usize]);
                }
                r
            })
            .flat_map(|vc| vc)
            .collect::<Vec<usize>>();
        // find dots + nums ranges
        let mut dots = find_seq(&s, |x| x == usize::MAX);
        let mut nums = find_seq(&s, |x| x != usize::MAX);
        nums.reverse();

        // move through nums in reverse order as we try to move the last one first
        for num in nums {
            let num_len = 1 + num.1 - num.0;
            //find a valid spot for the number to be moved to
            for (j, dot) in dots.iter().enumerate() {
                // makre sure dots is left of num
                if dot.0 > num.0 {
                    break;
                }
                let dot_len = 1 + dot.1 - dot.0;
                // check if num fits in dot slot
                if num_len <= dot_len {
                    for x in dot.0..dot.0 + num_len {
                        s[x] = s[num.0];
                    }
                    for x in num.0..=num.1 {
                        s[x] = usize::MAX;
                    }

                    // update how how many dots are left, or remove the dot range as it's used
                    if num_len != dot_len {
                        dots[j] = (dot.0 + num_len, dot.1);
                    } else {
                        dots.remove(j);
                    }

                    break;
                }
            }
        }

        for (i, num) in s.iter().enumerate() {
            if *num != usize::MAX {
                sum += i * num;
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
