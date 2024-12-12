use aoc_lib::init;
use num::integer::lcm;
use std::collections::HashMap;

fn p1(lines: Vec<String>) -> usize {
    let directions: Vec<char> = lines[0].chars().collect();

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();

    for i in 2..lines.len() {
        map.insert(&lines[i][0..3], (&lines[i][7..10], &lines[i][12..15]));
    }

    let mut j: usize = 0;
    let mut pos = "AAA";

    loop {
        if pos == "ZZZ" {
            return j;
        }
        if directions[j % directions.len()] == 'L' {
            pos = map.get(pos).unwrap().0
        } else {
            pos = map.get(pos).unwrap().1
        }
        j += 1;
    }
}

fn p2(lines: Vec<String>) -> usize {
    let directions: Vec<char> = lines[0].chars().collect();

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut poss: Vec<&str> = Vec::new();

    for i in 2..lines.len() {
        map.insert(&lines[i][0..3], (&lines[i][7..10], &lines[i][12..15]));
        if lines[i].chars().nth(2).unwrap() == 'A' {
            poss.push(&lines[i][0..3]);
        }
    }

    let mut js: Vec<usize> = Vec::new();

    for posx in poss {
        let mut j: usize = 0;
        let mut pos = posx;

        loop {
            if pos.chars().nth(2).unwrap() == 'Z' {
                js.push(j);
                break;
            }
            if directions[j % directions.len()] == 'L' {
                pos = map.get(pos).unwrap().0
            } else {
                pos = map.get(pos).unwrap().1
            }
            j += 1;
        }
    }

    // println!("js: {:?}", js);

    let mut res: usize = 1;
    for j in js {
        // res = res * j / gcd(res, j);
        res = lcm(res, j);
    }

    return res;
}

fn main() {
    let lines = init(2023, 8);
    // stuff here
    println!("{:?}", p1(lines.clone()));
    println!("{:?}", p2(lines.clone()));
}
