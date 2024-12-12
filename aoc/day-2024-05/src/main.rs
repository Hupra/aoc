use aoc_lib::init;
use rustlind_lib::*;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
    time::Instant,
};

fn p1gpt(input: Vec<String>) -> i32 {
    let mut rules = Vec::new();
    let mut updates = Vec::new();

    // Split input into rules and updates
    let mut is_update_section = false;
    for line in input {
        if line.trim().is_empty() {
            is_update_section = true;
            continue;
        }

        if is_update_section {
            updates.push(
                line.split(',')
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<_>>(),
            );
        } else {
            let parts: Vec<_> = line.split('|').collect();
            let x = parts[0].parse::<i32>().unwrap();
            let y = parts[1].parse::<i32>().unwrap();
            rules.push((x, y));
        }
    }

    // Create a graph from rules
    let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();
    for (x, y) in rules {
        graph.entry(x).or_insert_with(HashSet::new).insert(y);
    }

    // Function to check if a given update is in the correct order
    fn is_update_ordered(update: &[i32], graph: &HashMap<i32, HashSet<i32>>) -> bool {
        let mut index_map = HashMap::new();
        for (i, &page) in update.iter().enumerate() {
            index_map.insert(page, i);
        }

        for (x, y_set) in graph {
            if let Some(&x_idx) = index_map.get(x) {
                for &y in y_set {
                    if let Some(&y_idx) = index_map.get(&y) {
                        if x_idx >= y_idx {
                            return false;
                        }
                    }
                }
            }
        }

        true
    }

    // Process each update and calculate the result
    let mut result = 0;
    for update in updates {
        if is_update_ordered(&update, &graph) {
            let middle = update[update.len() / 2];
            result += middle;
        }
    }

    result
}

fn p1(lines: Vec<String>) -> i32 {
    let mut s_lines = lines.split(|l| l.is_empty()).map(|chunk| chunk.to_vec());

    let rules = s_lines
        .next()
        .unwrap()
        .into_iter()
        .collect::<HashSet<String>>();

    let pages = s_lines
        .next()
        .unwrap()
        .into_iter()
        .map(|l| l.split(",").map(|s| s.parse().unwrap()).collect())
        .collect::<Vec<Vec<i32>>>();

    let mut sum = 0;

    for page in pages {
        let mut valid = true;

        for i in 0..page.len() {
            for j in i + 1..page.len() {
                if !rules.contains(&format!("{}|{}", page[i], page[j])) {
                    valid = false;
                }
            }
        }
        if valid {
            sum += page[page.len() / 2]
        }
    }

    sum
}

fn p2(lines: Vec<String>) -> i32 {
    let mut s_lines = lines.split(|l| l.is_empty()).map(|chunk| chunk.to_vec());

    let rules = s_lines
        .next()
        .unwrap()
        .into_iter()
        .collect::<HashSet<String>>();

    let pages = s_lines
        .next()
        .unwrap()
        .into_iter()
        .map(|l| re_nums(&l))
        .collect::<Vec<Vec<i32>>>();

    let mut sum = 0;

    for page in pages {
        let mut valid = true;

        for i in 0..page.len() {
            for j in i + 1..page.len() {
                if !rules.contains(&format!("{}|{}", page[i], page[j])) {
                    valid = false;
                }
            }
        }
        if valid {
            continue;
        }

        let mut cpage = page;
        cpage.sort_by(|a, b| {
            if rules.contains(&format!("{}|{}", a, b)) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        sum += cpage[cpage.len() / 2]
    }

    sum
}

fn main() {
    let lines = init(2024, 5);
    let timer = Instant::now();
    println!("{:?} | time: {:?}", p1(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p1gpt(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p2(lines.clone()), timer.elapsed());
}
