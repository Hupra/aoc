use aoc_lib::init;
use std::time::Instant;

fn valid_vline(vline: &Vec<i32>) -> bool {
    let inc = vline[1] > vline[0];
    vline.windows(2).all(|pair| {
        let diff = pair[1] - pair[0];
        let diff_abs = diff.abs();
        (pair[1] > pair[0]) == inc && (1..=3).contains(&diff_abs)
    })
}

fn p1(lines: Vec<String>) -> Option<i32> {
    let mut sum = 0;

    for line in lines {
        let vline: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        if valid_vline(&vline) {
            sum += 1;
        }
    }

    Some(sum)
}

fn p2(lines: Vec<String>) -> Option<i32> {
    let mut sum = 0;

    for line in lines {
        let vline: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let mut cline: Vec<i32>;
        for x in 0..vline.len() {
            // let s1 = &vline[..x];
            // let s2 = &vline[x + 1..];
            // let s3 = s1.iter().chain(s2.iter()).copied().collect::<Vec<i32>>();
            // let s3 = [s1, s2].concat();
            cline = vline.clone();
            cline.remove(x);
            if valid_vline(&cline) {
                sum += 1;
                break;
            }
        }
    }

    Some(sum)
}

fn main() {
    let lines = init(2024, 2);
    let timer = Instant::now();
    println!("{:?} | time: {:?}", p1(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p2(lines.clone()), timer.elapsed());
}
