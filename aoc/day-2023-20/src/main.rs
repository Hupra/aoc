use aoc_lib::init;
use num::integer::lcm;
use std::{
    collections::{HashMap, VecDeque},
    time::Instant,
};

fn solve(lines: Vec<String>) {
    let mut total_low: usize = 0;
    let mut total_high: usize = 0;
    let mut q: VecDeque<(String, String, bool)> = VecDeque::new();
    let mut d: HashMap<String, (char, Option<HashMap<String, bool>>, Vec<String>)> = HashMap::new();
    let mut broad: Vec<String> = Vec::new();

    for line in lines {
        let line: Vec<&str> = line.split(" -> ").collect();
        let module: &str = line[0];
        let connections: Vec<String> = line[1].split(", ").map(|s| s.to_string()).collect();

        if module == "broadcaster" {
            broad = connections.clone();
        }

        let module_symbol: Option<char> = module.chars().next();
        let module_key: String = module[1..].to_string();

        match module_symbol {
            Some('%') => {
                d.insert(module_key, (module_symbol.unwrap(), None, connections));
            }
            Some('&') => {
                d.insert(
                    module_key,
                    (module_symbol.unwrap(), Some(HashMap::new()), connections),
                );
            }
            _ => {}
        }
    }

    for k in d.clone().keys() {
        if let Some(v) = d.get_mut(k) {
            for con in v.2.clone() {
                if let Some(vc) = d.get_mut(&con) {
                    if vc.0 == '&' {
                        if let Some(hm) = &mut vc.1 {
                            hm.insert(k.clone(), false); // Insert into the inner HashMap
                        }
                    }
                } else {
                    d.insert(con, ('X', None, Vec::new()));
                }
            }
        }
    }

    let mut nums: HashMap<String, usize> = HashMap::new();

    let dclone = &d.clone();
    for (k, v) in dclone {
        if v.0 == '&' {
            if v.2.contains(&"cl".to_string()) {
                nums.insert(k.clone(), 0);
            }
            println!("{} {:?}", k, v);
        }
    }

    println!("{:?}", broad);

    for i in 0..100_000 {
        if i == 1_000 {
            println!("low: {}", total_low);
            println!("hig: {}", total_high);
            println!("p1r: {}", total_low * total_high);
        }

        total_low += 1;

        for x in &broad {
            q.push_back(("".to_string(), x.to_string(), false));
        }

        while let Some(item) = q.pop_front() {
            let a = item.0;
            let b = item.1;
            let pulse = item.2;

            if pulse {
                total_high += 1;
            } else {
                total_low += 1;
            }

            //p2 here

            if b == "cl" {
                for key in nums.clone().keys() {
                    let y = *d
                        .get(&b)
                        .unwrap()
                        .1
                        .clone()
                        .unwrap()
                        .get(key)
                        .clone()
                        .unwrap();
                    if y {
                        if let Some(n) = nums.get(key) {
                            if *n == 0 {
                                nums.insert(key.to_string(), i + 1);
                                // println!("{}", i + 1)
                            }
                        }
                    }
                }

                if nums.values().all(|&n| n > 0) {
                    let p2res = nums
                        .values()
                        .into_iter()
                        .map(|n| *n)
                        .reduce(|prev, cur| lcm(prev, cur))
                        .unwrap();
                    if i >= 1_000 {
                        println!("{i} {:?}", nums.values());
                        println!("p2r: {}", p2res);
                        return;
                    }
                }
            }

            let symbol = d.get(&b).unwrap().0;
            let connections = d.get(&b).unwrap().2.clone();

            if symbol == '%' {
                let x = d.get_mut(&b).unwrap();
                if pulse == false {
                    // flip on/off switch
                    let switch = if x.1.is_some() {
                        None
                    } else {
                        Some(HashMap::<String, bool>::new())
                    };
                    x.1 = switch.clone();
                    for con in connections {
                        q.push_back((b.to_string(), con.to_string(), switch.is_some()));
                    }
                }
            } else if symbol == '&' {
                let mut all_high_pulse = false;
                if let Some(vc) = d.get_mut(&b) {
                    if let Some(hm) = &mut vc.1 {
                        hm.insert(a, pulse);
                        all_high_pulse = hm.values().all(|&x| x == true);
                    }
                }

                for con in connections {
                    q.push_back((b.to_string(), con.to_string(), !all_high_pulse));
                }
            }
        }
    }
}

fn main() {
    // The code for this solution is...
    // very bad... but it works :)
    let lines = init(2023, 20);
    let timer = Instant::now();
    solve(lines.clone());
    println!("time: {:?}", timer.elapsed());
}
