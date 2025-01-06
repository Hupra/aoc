use aoc_lib::init;
use cached::proc_macro::cached;
use itertools::{ min, sorted, Itertools };
use petgraph::operator;
use rustlind_lib::{ tadd, utils::simple_graph::Graph };
use std::{
    collections::HashMap,
    fs::File,
    hash::Hash,
    io::Write,
    process::{ Command, Stdio },
    time::Instant,
};

fn p1(lines: Vec<String>) -> Option<usize> {
    let blocks: Vec<_> = lines
        .split(|line| line.is_empty())
        .map(|s| s.to_vec())
        .collect();

    let mut g: Graph<&str> = Graph::new();
    let mut instructions: HashMap<&str, (&str, &str, &str)> = HashMap::new();
    let mut results: HashMap<&str, usize> = HashMap::new();

    for line in &blocks[0] {
        // example
        // x02: 1
        let parts: Vec<&str> = line.split(": ").collect();
        results.insert(parts[0], parts[1].parse().unwrap_or_default());
    }

    for line in &blocks[1] {
        // example
        // ntg XOR fgs -> mjb
        let parts: Vec<&str> = line.split_whitespace().collect();
        let (x, operator, y, target) = (parts[0], parts[1], parts[2], parts[4]);
        g.add_edge(x, target);
        g.add_edge(y, target);
        instructions.insert(target, (x, y, operator));
    }

    for target in g.topological_sort().unwrap() {
        if let Some((x, y, operator)) = instructions.get(&target) {
            let x = results.get(x).unwrap();
            let y = results.get(y).unwrap();

            let target_result = match *operator {
                "AND" => (x + y == 2) as usize,
                "XOR" => (x != y) as usize,
                "OR" => (x + y > 0) as usize,
                _ => 0,
            };
            results.insert(target, target_result);
        }
    }

    let bin_string = results
        .iter()
        .filter(|r| r.0.starts_with('z'))
        .sorted_by(|a, b| b.cmp(a))
        .map(|p| p.1.to_string())
        .collect::<String>();

    Some(usize::from_str_radix(&bin_string, 2).unwrap())
}

fn rename_wires(wires: Vec<Vec<String>>) -> (Vec<Vec<String>>, bool) {
    let mut renames: HashMap<String, String> = HashMap::new();
    renames.insert(String::from("AND00"), String::from("CARRY00"));

    wires.iter().for_each(|wire| {
        let (x, operator, y, target) = (&wire[0], &wire[1], &wire[2], &wire[3]);
        let (x, y) = (x.min(y), x.max(y));
        let xn = x[x.len() - 2..].parse::<usize>();
        let yn = y[y.len() - 2..].parse::<usize>();

        // dbg!(&x, &y, &xn, &yn);

        // "x(N-1)", "XOR", "y(N)" -> "XOR(N)"
        if
            x.starts_with('x') &&
            y.starts_with('y') &&
            operator == "XOR" &&
            !target.starts_with("XOR")
        {
            renames.insert(target.clone(), format!("{}{}", operator, &x[1..]));
        }

        // "x(N-1)", "AND", "y(N)" -> "AND(N)"
        if
            x.starts_with('x') &&
            y.starts_with('y') &&
            operator == "AND" &&
            !target.starts_with("AND") &&
            target != "CARRY00"
        {
            renames.insert(target.clone(), format!("{}{}", operator, &x[1..]));
        }

        // "CARRY(N-1)", "AND", "XOR(N)" -> "INTERMEDIATE_CARRY(N)"
        if let (Ok(xn), Ok(yn)) = (&xn, &yn) {
            if
                x.starts_with("CARRY") &&
                operator == "AND" &&
                y.starts_with("XOR") &&
                xn + 1 == yn.clone() &&
                !target.starts_with("INTERMEDIATE_CARRY")
            {
                renames.insert(target.clone(), format!("INTERMEDIATE_CARRY{:02}", yn));
            }
        }

        // "AND(N)", "OR", "INTERMEDIATE_CARRY(N)" -> "CARRY(N)"
        if let (Ok(xn), Ok(yn)) = (&xn, &yn) {
            if
                x.starts_with("AND") &&
                operator == "OR" &&
                y.starts_with("INTERMEDIATE_CARRY") &&
                xn == yn &&
                !target.starts_with("CARRY")
            {
                renames.insert(target.clone(), format!("CARRY{:02}", xn));
            }
        }
    });

    let mut modified = false;
    let new_wires = wires
        .iter()
        .map(|wire| {
            let (mut x, operator, mut y, mut target) = (
                wire[0].clone(),
                wire[1].clone(),
                wire[2].clone(),
                wire[3].clone(),
            );
            if let Some(new_name) = renames.get(&x) {
                x = new_name.clone();
                modified = true;
            }
            if let Some(new_name) = renames.get(&y) {
                y = new_name.clone();
                modified = true;
            }
            if let Some(new_name) = renames.get(&target) {
                target = new_name.clone();
                modified = true;
            }
            vec![x, operator, y, target]
        })
        .collect();

    // dbg!(renames);

    (new_wires, modified)
}

fn p2(lines: Vec<String>) -> Option<String> {
    let blocks: Vec<_> = lines
        .split(|line| line.is_empty())
        .map(|s| s.to_vec())
        .collect();

    let mut results: HashMap<&str, usize> = HashMap::new();

    for line in &blocks[0] {
        // example
        // x02: 1
        let parts: Vec<&str> = line.split(": ").collect();
        results.insert(parts[0], parts[1].parse().unwrap_or_default());
    }

    let xs = usize
        ::from_str_radix(
            &results
                .iter()
                .filter(|r| r.0.starts_with('x'))
                .sorted_by(|a, b| b.cmp(a))
                .map(|p| p.1.to_string())
                .collect::<String>(),
            2
        )
        .unwrap();
    let ys = usize
        ::from_str_radix(
            &results
                .iter()
                .filter(|r| r.0.starts_with('y'))
                .sorted_by(|a, b| b.cmp(a))
                .map(|p| p.1.to_string())
                .collect::<String>(),
            2
        )
        .unwrap();

    let xys = xs + ys;
    println!("{} {} {} ", xs, xs, xys);

    let mut logic: Vec<(&str, &str, &str)> = Vec::new();
    let mut targets: Vec<&str> = Vec::new();

    let mut real_wires: Vec<Vec<String>> = blocks[1]
        .iter()
        .map(|line|
            line
                .replace(&"-> ".to_string(), "")
                .split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        )
        .collect();

    let u = 46;
    let v = 59;
    let tmp = real_wires[u][3].clone();
    real_wires[u][3] = real_wires[v][3].clone();
    real_wires[v][3] = tmp;

    let u = 7;
    let v = 100;
    let tmp = real_wires[u][3].clone();
    real_wires[u][3] = real_wires[v][3].clone();
    real_wires[v][3] = tmp;

    let u = 57;
    let v = 90;
    let tmp = real_wires[u][3].clone();
    real_wires[u][3] = real_wires[v][3].clone();
    real_wires[v][3] = tmp;

    let u = 71;
    let v = 195;
    let tmp = real_wires[u][3].clone();
    real_wires[u][3] = real_wires[v][3].clone();
    real_wires[v][3] = tmp;

    let mut res = vec![
        &real_wires[46][3],
        &real_wires[59][3],
        &real_wires[7][3],
        &real_wires[100][3],
        &real_wires[57][3],
        &real_wires[90][3],
        &real_wires[71][3],
        &real_wires[195][3]
    ];

    res.sort();

    let res: String = res.iter().join(",");
    return Some(res);

    let mut max_carry: usize = 0;
    let mut best_swap: (usize, usize) = (0, 0);

    for u in 0..blocks[1].len() {
        continue;

        for v in u + 1..blocks[1].len() {
            let mut wires: Vec<Vec<String>> = real_wires.clone();

            let tmp = wires[u][3].clone();
            wires[u][3] = wires[v][3].clone();
            wires[v][3] = tmp;

            let mut modified = true;
            while modified {
                (wires, modified) = rename_wires(wires);
            }

            let new_max_carry = wires
                .iter()
                .map(|wire| {
                    let target = wire[3].clone();
                    if !target.starts_with("CARRY") {
                        0usize
                    } else {
                        target[target.len() - 2..].parse::<usize>().unwrap_or_default()
                    }
                })
                .max()
                .unwrap_or_default();

            if new_max_carry > max_carry {
                max_carry = new_max_carry;
                best_swap = (u, v);
                dbg!(max_carry, best_swap, &blocks[1][best_swap.0], &blocks[1][best_swap.1]);
            }
        }
    }

    ///
    /// ///
    ///
    /// //
    ///
    /// /
    /// //
    ///
    /// /
    /// //
    ///

    let mut wires = real_wires.clone();
    let mut modified = true;
    while modified {
        (wires, modified) = rename_wires(wires);
    }

    // dbg!(&wires);

    for wire in &wires {
        // example
        // ntg XOR fgs -> mjb
        let (x, operator, y, target) = (&wire[0], &wire[1], &wire[2], &wire[3]);
        logic.push((x, y, operator));
        targets.push(&target);
    }

    let mut g: Graph<&str> = Graph::new();
    let mut instructions: HashMap<&str, (&str, &str, &str)> = HashMap::new();

    println!("{:60b}", xys);

    let g_targets = targets.clone();

    // Visualize graph
    // let mut vis_g: Graph<String> = Graph::new();
    // for i in 0..logic.len() {
    //     let logic = logic[i];
    //     let target = targets[i];

    //     // Now pass references to `op` since it is owned and lives long enough
    //     vis_g.add_edge(logic.0.to_string(), target.to_string());
    //     vis_g.add_edge(logic.1.to_string(), target.to_string());
    //     instructions.insert(target, logic);
    // }
    // let _ = vis_g.to_png("ignore/graph.png");

    let mut vis_g: Graph<String> = Graph::new();
    for i in 0..logic.len() {
        let logic = logic[i];
        let target = targets[i];
        let op = match logic.2 {
            "AND" => "& ".to_string() + &i.to_string(),
            "XOR" => "^ ".to_string() + &i.to_string(),
            "OR" => "| ".to_string() + &i.to_string(),
            _ => "".to_string() + &i.to_string(),
        };

        // Now pass references to `op` since it is owned and lives long enough
        vis_g.add_edge(logic.0.to_string(), op.to_string());
        vis_g.add_edge(logic.1.to_string(), op.to_string());
        vis_g.add_edge(op.to_string(), target.to_string());
        instructions.insert(target, logic);
    }
    let _ = vis_g.to_png("ignore/graph.png");

    // start iteration here

    g.clear();
    instructions.clear();

    let mut targets = g_targets.clone();

    for i in 0..logic.len() {
        let logic = logic[i];
        let target = targets[i];
        g.add_edge(logic.0, target);
        g.add_edge(logic.1, target);
        instructions.insert(target, logic);
    }

    let order = g.topological_sort();
    if order.is_err() {
        return None;
    }

    let order = order.unwrap();

    // order.ge

    // for i in 0..order.len() {
    //     if order[i] == "z00" {
    //         println!("--------------{} {:?}", i, order[i]);
    //     }
    //     println!("{} {:?}", i, order[i]);
    // }
    for target in order {
        if let Some((x, y, operator)) = instructions.get(&target) {
            let x = results.get(x).unwrap();
            let y = results.get(y).unwrap();

            let target_result = match *operator {
                "AND" => x & y,
                "XOR" => x ^ y,
                "OR" => x | y,
                _ => 0,
            };
            results.insert(target, target_result);
        }
    }
    let zs = usize
        ::from_str_radix(
            &results
                .iter()
                .filter(|r| r.0.starts_with('z'))
                .sorted_by(|a, b| b.cmp(a))
                .map(|p| p.1.to_string())
                .collect::<String>(),
            2
        )
        .unwrap();

    // 1100001110001111100010001100100000011011100110
    let b = 0b100010001100100000011011100110;
    let a = 0b111111111111111111111111111111;

    let b = 0b100010001100100000011011100110;
    let a = 0b111111111111111111111111111111;

    println!("{:60b}", zs);

    None
}

fn main() {
    let lines = init(2024, 24);
    let timer = Instant::now();
    println!("{:?} | time: {:?}", p1(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p2(lines.clone()), timer.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test_lines() -> Vec<String> {
        include_str!("test.txt").lines().map(String::from).collect()
    }

    #[test]
    fn test_p1() {
        let expected = Some(2024);
        let actual = p1(test_lines());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_p2() {
        let expected = Some("z00,z01,z02,z05".to_string());
        let actual = p2(include_str!("test2.txt").lines().map(String::from).collect());
        assert_eq!(actual, expected);
    }
}
