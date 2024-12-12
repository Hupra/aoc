use aoc_lib::init;
use std::{collections::HashMap, time::Instant};

fn p1(lines: Vec<String>) -> Option<i32> {
    let mut f = lines.split(|s| s.is_empty()).map(|grp| grp.to_vec());
    let workflows = f.next().unwrap();
    let parts = f.next().unwrap();

    let mut d: HashMap<String, (Vec<String>, String)> = HashMap::new();

    for flow in workflows {
        let flow = &flow[..flow.len() - 1];
        let brac_s = flow.find('{').unwrap();
        let mut arr: Vec<String> = flow[brac_s + 1..]
            .split(',')
            .map(|s| s.to_string())
            .collect();
        let key = flow[..brac_s].to_string();
        let end = arr.pop()?;

        d.insert(key, (arr, end));
    }

    let mut new_parts: Vec<HashMap<char, i32>> = Vec::new();
    for i in 0..parts.len() {
        let part = parts[i][1..parts[i].len() - 1].to_string();
        let part: Vec<&str> = part.split(',').collect();
        let mut partd: HashMap<char, i32> = HashMap::new();
        for sub in part {
            let k = sub.chars().next()?;
            let v = sub[2..].parse::<i32>().unwrap();
            partd.insert(k, v);
        }
        new_parts.push(partd);
    }

    fn rec(
        key: String,
        part: &HashMap<char, i32>,
        d: &HashMap<String, (Vec<String>, String)>,
    ) -> String {
        if key == "A" || key == "R" {
            return key;
        }

        let flow = d.get(&key).unwrap();
        let rules = &flow.0;

        for rule in rules {
            let rulechar = rule.chars().next().unwrap();

            if part.contains_key(&rulechar) {
                let mut exq = rule.split(':');
                if let (Some(con), Some(key)) = (exq.next(), exq.next()) {
                    let sign = con.chars().nth(1).unwrap();
                    let char_num = *part.get(&rulechar).unwrap();
                    let num = con[2..].parse::<i32>().unwrap();
                    let res: bool = if sign == '<' {
                        char_num < num
                    } else {
                        char_num > num
                    };

                    if res {
                        return rec(key.to_string(), part, d);
                    }
                }
            }
        }

        return rec(flow.1.clone(), part, d);
    }

    let mut sum = 0;
    for part in new_parts {
        let letter = rec("in".to_string(), &part, &d);
        if letter == "A".to_string() {
            sum += part.values().sum::<i32>()
        }
    }

    Some(sum)
}

fn p2(lines: Vec<String>) -> Option<usize> {
    let mut f = lines.split(|s| s.is_empty()).map(|grp| grp.to_vec());
    let workflows = f.next().unwrap();
    let mut d: HashMap<String, (Vec<String>, String)> = HashMap::new();

    for flow in workflows {
        let flow = &flow[..flow.len() - 1];
        let brac_s = flow.find('{').unwrap();
        let mut arr: Vec<String> = flow[brac_s + 1..]
            .split(',')
            .map(|s| s.to_string())
            .collect();
        let key = flow[..brac_s].to_string();
        let end = arr.pop()?;

        d.insert(key, (arr, end));
    }

    let mut part: HashMap<char, (usize, usize)> = HashMap::from([
        ('x', (1, 4000)),
        ('m', (1, 4000)),
        ('a', (1, 4000)),
        ('s', (1, 4000)),
    ]);

    return Some(rec(format!("in"), &mut part, &d));

    fn rec(
        key: String,
        part: &mut HashMap<char, (usize, usize)>,
        d: &HashMap<String, (Vec<String>, String)>,
    ) -> usize {
        if key == "R" {
            return 0;
        }
        if key == "A" {
            return part.values().map(|ran| ran.1 - ran.0 + 1).product();
        }

        let mut points = 0;
        let flow = d.get(&key).unwrap();
        let rules = &flow.0;

        for rule in rules {
            let rulechar = rule.chars().next().unwrap();

            if part.contains_key(&rulechar) {
                // println!("{}", rule);
                let mut exq = rule.split(':');
                let con = exq.next().unwrap();
                let conres = exq.next().unwrap();
                let currange = part.get(&rulechar).unwrap();
                let number = con[2..].parse::<usize>().unwrap();
                let sign = con.chars().nth(1).unwrap();

                match sign {
                    '<' => {
                        if currange.1 < number {
                            // recurse the full range
                            return rec(conres.to_string(), part, d) + points;
                        }
                        if currange.0 < number {
                            // recurse some of the range
                            // maxrange must be => number     newpart   part
                            // s<1351 px      1200 1500 -> 1200,1350 1351,1500
                            //                1200 1351 -> 1200,1350 1351,1351
                            //                1350 1351 -> 1350,1350 1351,1351
                            let mut newpart = part.clone();
                            newpart.insert(rulechar, (currange.0, number - 1));
                            part.insert(rulechar, (number, currange.1));
                            points += rec(conres.to_string(), &mut newpart, d)
                        }
                    }
                    '>' => {
                        if currange.0 > number {
                            // recurse the full range
                            return rec(conres.to_string(), part, d) + points;
                        }
                        if currange.1 > number {
                            // recure some of the range, there must a subrange less than number,
                            // else first case would have taken all
                            let mut newpart = part.clone();
                            newpart.insert(rulechar, (number + 1, currange.1));
                            part.insert(rulechar, (currange.0, number));
                            points += rec(conres.to_string(), &mut newpart, d)
                        }
                    }
                    _ => {}
                };
            }
        }
        return rec(flow.1.to_string(), part, d) + points;
    }
}

// r 134370637448305
fn main() {
    let lines = init(2023, 19);
    let timer = Instant::now();
    println!("{:?} | time: {:?}", p1(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p2(lines.clone()), timer.elapsed());
}
