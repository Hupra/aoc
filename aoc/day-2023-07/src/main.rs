use std::collections::HashMap;

use aoc_lib::init;

fn help(hand: &str, part: usize) -> String {
    // for part 2 J is the worst value. ie. z
    let j: char = if part == 2 { 'z' } else { 'd' };
    let fix: HashMap<char, char> = HashMap::from([
        ('A', 'a'),
        ('K', 'b'),
        ('Q', 'c'),
        ('J', j),
        ('T', 'e'),
        ('9', 'f'),
        ('8', 'g'),
        ('7', 'h'),
        ('6', 'i'),
        ('5', 'j'),
        ('4', 'k'),
        ('3', 'l'),
        ('2', 'm'),
    ]);

    let x: String = hand.chars().map(|c| *fix.get(&c).unwrap()).collect();
    let mut d: HashMap<char, usize> = HashMap::new();
    for c in hand.chars() {
        *d.entry(c).or_insert(0) += 1;
    }

    if part == 2 {
        if let Some(&j_count) = d.get(&'J') {
            match j_count {
                4.. => {
                    return format!("1{}", x); // 5 of a kind
                }
                3 => {
                    if d.len() == 2 {
                        return format!("1{}", x); // 5 of a kind
                    } else {
                        return format!("2{}", x); // 4 of a kind
                    }
                }
                2 => {
                    if let Some(&max_val) = d.values().max() {
                        if max_val == 3 {
                            return format!("1{}", x); // 5 of a kind
                        }
                    }
                    if d.len() == 3 {
                        return format!("2{}", x); // 4 of a kind
                    }
                    return format!("4{}", x);
                }
                1 => {
                    if let Some(&max_val) = d.values().max() {
                        if max_val == 4 {
                            return format!("1{}", x); // 5 of a kind
                        }
                        if max_val == 3 {
                            return format!("2{}", x); // 4 of a kind
                        }
                    }
                    match d.len() {
                        3 => return format!("3{}", x), // full house
                        4 => return format!("4{}", x), // 3 of a kind
                        _ => return format!("6{}", x), // 1 pair
                    }
                }
                _ => {}
            }
        }
    }
    if d.len() == 1 {
        // 5 of a kind
        return '1'.to_string() + &x;
    }
    if d.values().max() == Some(&4) {
        // 4 of a kind
        return '2'.to_string() + &x;
    }
    if d.len() == 2 {
        // Full house
        return '3'.to_string() + &x;
    }
    if d.values().max() == Some(&3) {
        // 3 of a kind
        return '4'.to_string() + &x;
    }
    if d.len() == 3 {
        // 2 pair
        return '5'.to_string() + &x;
    }
    if d.len() == 4 {
        // 1 pair
        return '6'.to_string() + &x;
    }
    // No match
    return '7'.to_string() + &x;
}

fn start(lines: Vec<String>, part: usize) -> usize {
    let mut data: Vec<(&str, usize)> = Vec::new();

    for line in &lines {
        let tmp: Vec<&str> = line.split_whitespace().collect();
        data.push((tmp[0], tmp[1].parse().unwrap()));
    }

    let mut hands: Vec<(String, usize)> = data
        .into_iter()
        .map(|x: (&str, usize)| (help(x.0, part), x.1))
        .collect();

    hands.sort();

    let mut sum = 0;

    for i in 0..hands.len() {
        sum += hands[i].1 * (hands.len() - i)
    }

    return sum;
}

fn p1(lines: Vec<String>) -> usize {
    start(lines, 1)
}

fn p2(lines: Vec<String>) -> usize {
    start(lines, 2)
}

fn main() {
    let lines = init(2023, 7);
    // stuff here
    println!("{:?}", p1(lines.clone()));
    println!("{:?}", p2(lines.clone()));
}
