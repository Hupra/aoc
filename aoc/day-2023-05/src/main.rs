use aoc_lib::init;
use std::cmp;

fn p1(lines: Vec<String>) -> i64 {
    let seeds: Vec<i64> = lines[0]
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut transfomers: Vec<Vec<Vec<i64>>> = Vec::new();

    let mut i: usize = 1;

    while i < lines.len() {
        if lines[i].is_empty() {
            transfomers.push(Vec::new());
            i += 1;
        } else {
            // From, To, Range
            let data: Vec<i64> = lines[i].split(" ").map(|s| s.parse().unwrap()).collect();
            let trip: Vec<i64> = vec![data[1], data[1] + data[2] - 1, data[0] - data[1]];
            let idx = transfomers.len() - 1;
            transfomers[idx].push(trip);
        }
        i += 1;
    }

    let mut locations = Vec::<i64>::new();

    for seed in seeds {
        let mut loc = seed;

        for paths in &transfomers {
            for path in paths {
                if loc >= path[0] && loc <= path[1] {
                    loc += path[2];
                    break;
                }
            }
        }
        locations.push(loc);
    }

    locations.into_iter().min().unwrap()
}

fn p2(lines: Vec<String>) -> i64 {
    let nums: Vec<i64> = lines[0]
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut ranges: Vec<(i64, i64)> = Vec::new();

    for x in (0..nums.len()).step_by(2) {
        ranges.push((nums[x], nums[x] + nums[x + 1] - 1));
    }

    let mut transfomers: Vec<Vec<Vec<i64>>> = Vec::new();

    let mut i: usize = 1;
    while i < lines.len() {
        if lines[i].is_empty() {
            transfomers.push(Vec::new());
            i += 1;
        } else {
            // From, To, Range
            let data: Vec<i64> = lines[i].split(" ").map(|s| s.parse().unwrap()).collect();
            let trip: Vec<i64> = vec![data[1], data[1] + data[2] - 1, data[0] - data[1]];
            let idx = transfomers.len() - 1;
            transfomers[idx].push(trip);
        }
        i += 1;
    }

    return ranges
        .into_iter()
        .map(|range| rec(range, 0, &transfomers))
        .min()
        .unwrap();
}

fn rec(range: (i64, i64), t: usize, transformers: &Vec<Vec<Vec<i64>>>) -> i64 {
    // range start/end
    let rs = range.0;
    let re = range.1;

    // stop condition
    if t >= transformers.len() {
        return rs;
    }

    for path in &transformers[t] {
        let l = path[0];
        let u = path[1];
        let d = path[2];

        // intersection
        let trs = cmp::max(rs, l);
        let tre = cmp::min(re, u);

        // valid intersection
        if trs <= tre {
            let mut res: Vec<i64> = Vec::new();

            // transform intersection
            res.push(rec((trs + d, tre + d), t + 1, transformers));

            // get left-leftover range
            if rs < trs {
                res.push(rec((rs, trs - 1), t, transformers));
            }

            // get right-leftover range
            if tre < re {
                res.push(rec((tre + 1, re), t, transformers));
            }

            return res.into_iter().min().unwrap();
        }
    }

    return rec(range, t + 1, transformers);
}

fn main() {
    let lines = init(2023, 5);
    // stuff here
    println!("{:?}", p1(lines.clone()));
    println!("{:?}", p2(lines.clone()));
}
