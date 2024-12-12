use std::{cmp, collections::HashMap, usize};

use aoc_lib::init;

fn drec(s: String, g: String, memo: &mut HashMap<(String, String), usize>) -> usize {
    let key: (String, String) = (s.clone(), g.clone());

    // Check if the result is already computed and stored
    if let Some(&res) = memo.get(&key) {
        return res;
    }

    // Base case: if `s` is empty
    if s.is_empty() {
        let res = if g.is_empty() { 1 } else { 0 };
        memo.insert(key, res);
        return res;
    }

    match s.chars().next() {
        Some('.') => {
            let res = drec(s[1..].to_string(), g, memo);
            memo.insert(key, res);
            return res;
        }
        Some('#') => {
            if g.len() == 0 {
                memo.insert(key, 0);
                return 0;
            }

            let bc = g.find(','); // Finds the first occurrence of ',' in the string g
            let bidx = bc.unwrap_or(g.len());
            let bombs: usize = g[..bidx].parse().unwrap_or(0);

            if bombs > s.len()
                || s[..bombs].contains('.')
                || (bombs < s.len() && s.chars().nth(bombs).unwrap() == '#')
            {
                memo.insert(key, 0);
                return 0;
            }
            match bc {
                None => {
                    let res = drec(s[bombs..].to_string(), "".to_string(), memo);
                    memo.insert(key, res);
                    return res;
                }
                _ => {
                    let res = drec(
                        s[cmp::min(bombs + 1, s.len())..].to_string(),
                        g[bc.unwrap() + 1..].to_string(),
                        memo,
                    );
                    memo.insert(key, res);
                    return res;
                }
            }
        }
        Some('?') => {
            // set .
            let dot = drec(s[1..].to_string(), g.clone(), memo);
            if g.len() == 0 {
                memo.insert(key, dot);
                return dot;
            }

            // set #
            let bc = g.find(','); // Finds the first occurrence of ',' in the string g
            let bidx = bc.unwrap_or(g.len());
            let bombs: usize = g[..bidx].parse().unwrap();

            // check if we can't place #s on the next "bombs" chars
            if bombs > s.len()
                || s[..bombs].contains('.')
                || (bombs < s.len() && s.chars().nth(bombs).unwrap() == '#')
            {
                memo.insert(key, dot);
                return dot;
            }
            match bc {
                None => {
                    let res = drec(s[bombs..].to_string(), "".to_string(), memo);
                    memo.insert(key, dot + res);
                    return dot + res;
                }
                _ => {
                    let res = drec(
                        s[cmp::min(bombs + 1, s.len())..].to_string(),
                        g[bc.unwrap() + 1..].to_string(),
                        memo,
                    );
                    memo.insert(key, dot + res);
                    return dot + res;
                }
            }
        }
        _ => {}
    }
    memo.insert(key, 0);
    return 0;
}

fn p1(lines: Vec<String>) -> usize {
    let mut sum = 0;

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let data = parts[0];
        let guide = parts[1];

        let mut memo: HashMap<(String, String), usize> = HashMap::new();
        let part = drec(data.to_string(), guide.to_string(), &mut memo);

        sum += part;
    }

    return sum;
}

fn p2(lines: Vec<String>) -> usize {
    let mut sum: usize = 0;

    for line in lines {
        // Split the line into parts
        let parts: Vec<&str> = line.split_whitespace().collect();

        // Repeat the first part 5 times and join with '?'
        let data = vec![parts[0]; 5].join("?");

        // Repeat the second part 5 times and join with ','
        let guide = vec![parts[1]; 5].join(",");

        let mut memo: HashMap<(String, String), usize> = HashMap::new();
        let part = drec(data.to_string(), guide.to_string(), &mut memo);

        sum += part;
    }

    return sum;
}

fn main() {
    let lines = init(2023, 12);
    // stuff here
    println!("{:?}", p1(lines.clone()));
    println!("{:?}", p2(lines.clone()));
}
