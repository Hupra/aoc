use aoc_lib::init;
use cached::proc_macro::cached;
use itertools::Itertools;
use rustlind_lib::*;
use std::{collections::HashMap, sync::OnceLock, time::Instant, usize};

static NUM_HELPER: OnceLock<HashMap<(char, char), Vec<String>>> = OnceLock::new();

fn init_pads() -> (HashMap<char, (i32, i32)>, HashMap<char, (i32, i32)>) {
    #[rustfmt::skip]
    let num_pad: HashMap<char, (i32, i32)> = HashMap::from([
        ('7', (0, 0)), ('8', (0, 1)), ('9', (0, 2)),
        ('4', (1, 0)), ('5', (1, 1)), ('6', (1, 2)),
        ('1', (2, 0)), ('2', (2, 1)), ('3', (2, 2)),
                       ('0', (3, 1)), ('A', (3, 2)),
        ]);

    #[rustfmt::skip]
    let dir_pad: HashMap<char, (i32, i32)> = HashMap::from([
                       ('^', (0, 1)), ('A', (0, 2)),
        ('<', (1, 0)), ('v', (1, 1)), ('>', (1, 2)),
        ]);

    (num_pad, dir_pad)
}

// notice that any sequence from char to char always starts with an A
// see below example where we can see that <^ will have it initialposition on A:
// <>^ =>  >< | <^
//     => <<<A|>^A
// so if we want to calculate these parts separately we must
// include the A|> part in one of the two parts,
// and we must chose the "right" part as it doesn't need any
// information from the "left" part

fn get_num_helper() -> &'static HashMap<(char, char), Vec<String>> {
    NUM_HELPER.get_or_init(|| {
        let (_num_pad, dir_pad) = init_pads();

        let inputs = "A^v<>";
        let perms = [
            inputs
                .chars()
                .permutations(2)
                .into_iter()
                .collect::<Vec<Vec<char>>>(),
            inputs
                .chars()
                .map(|c| vec![c, c])
                .collect::<Vec<Vec<char>>>(),
        ]
        .concat();

        perms
            .into_iter()
            .map(|perm| {
                let (a, b) = (perm[0], perm[1]);
                ((a, b), find_paths(&format!("{}{}", a, b), &dir_pad, (0, 0)))
            })
            .collect()
    })
}

fn find_paths(command: &str, pad: &HashMap<char, (i32, i32)>, skip: (i32, i32)) -> Vec<String> {
    if command.len() < 2 {
        return vec!["".to_string()];
    }

    let cur_pos = *pad.get(&command.chars().nth(0).unwrap()).unwrap();
    let nex_pos = *pad.get(&command.chars().nth(1).unwrap()).unwrap();
    let (ci, cj) = cur_pos;
    let (ni, nj) = nex_pos;

    let mut keys: Vec<char> = Vec::new();

    if cj < nj {
        keys.extend(vec!['>'; (nj - cj) as usize]);
    }
    if cj > nj {
        keys.extend(vec!['<'; (cj - nj) as usize]);
    }
    if ci < ni {
        keys.extend(vec!['v'; (ni - ci) as usize]);
    }
    if ci > ni {
        keys.extend(vec!['^'; (ci - ni) as usize]);
    }

    let mut paths: Vec<String> = Vec::new();

    'perm: for perm in keys.iter().permutations(keys.len()) {
        let mut here = cur_pos;
        for c in &perm {
            let next = match c {
                '^' => tadd!(here, (-1, 0)),
                'v' => tadd!(here, (1, 0)),
                '<' => tadd!(here, (0, -1)),
                '>' => tadd!(here, (0, 1)),
                _ => (i32::MAX, i32::MAX),
            };
            if next == skip {
                continue 'perm;
            }
            here = next;
        }
        paths.push(perm.into_iter().collect());
    }

    let child_paths = find_paths(&command[1..], pad, skip);

    let mut all_paths: Vec<String> = Vec::new();
    for path in paths {
        for child_path in &child_paths {
            all_paths.push(format!("{}A{}", path, child_path));
        }
    }
    all_paths
}

#[cached(
    key = "(String, usize)",
    convert = r#"{ (seq.to_string(), iterations) }"#
)]
fn dp(seq: &str, iterations: usize) -> usize {
    if iterations == 0 {
        return seq.len();
    }

    ('A'.to_string() + &seq)
        .chars()
        .tuple_windows()
        .map(|(a, b)| {
            get_num_helper()
                .get(&(a, b))
                .unwrap()
                .iter()
                .map(|path| dp(path, iterations - 1))
                .min()
                .unwrap()
        })
        .sum()
}

fn solve(lines: Vec<String>) -> (usize, usize) {
    let mut sum_p1: usize = 0;
    let mut sum_p2: usize = 0;
    let (num_pad, _dir_pad) = init_pads();

    for line in lines {
        let numeric_code_part = line
            .chars()
            .filter(|c: &char| c.is_digit(10))
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        let paths = find_paths(&("A".to_string() + &line), &num_pad, (3, 0));

        let min_path_p1 = paths
            .iter()
            .map(|path| dp(path, 2))
            .min()
            .unwrap_or_default();

        let min_path_p2 = paths
            .iter()
            .map(|path| dp(path, 25))
            .min()
            .unwrap_or_default();

        println!(
            "NUM:{} | P1: {} | P2: {}",
            numeric_code_part, min_path_p1, min_path_p2
        );

        sum_p1 += min_path_p1 * numeric_code_part;
        sum_p2 += min_path_p2 * numeric_code_part;
    }

    (sum_p1, sum_p2)
}

fn main() {
    let lines = init(2024, 21);
    let timer = Instant::now();
    println!("{:?} | time: {:?}", solve(lines.clone()), timer.elapsed());
}
