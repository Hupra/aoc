use aoc_lib::init;
use rustlind_lib::*;
use std::{collections::HashMap, time::Instant};

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

    // char == char -> A

    (num_pad, dir_pad)
}

fn rec(cur_pos: (i32, i32), command: &str, pad: &HashMap<char, (i32, i32)>) -> Vec<char> {
    if command.is_empty() {
        return vec![];
    }

    // println!("{:?}", command);

    let (ci, cj) = cur_pos;
    let (ni, nj) = *pad.get(&command.chars().nth(0).unwrap()).unwrap();

    if ci == ni && cj == nj {
        let mut path = Vec::from(['A']);
        path.extend(rec(cur_pos, &command[1..], pad));
        return path;
    }
    if cj < nj {
        let mut path = Vec::from(['>']);
        path.extend(rec(tadd!(cur_pos, (0, 1)), command, pad));
        return path;
    }
    if cj > nj {
        let mut path = Vec::from(['<']);
        path.extend(rec(tadd!(cur_pos, (0, -1)), command, pad));
        return path;
    }
    if ci < ni {
        let mut path = Vec::from(['v']);
        path.extend(rec(tadd!(cur_pos, (1, 0)), command, pad));
        return path;
    }
    if ci > ni {
        let mut path = Vec::from(['^']);
        path.extend(rec(tadd!(cur_pos, (-1, 0)), command, pad));
        return path;
    }

    return vec![];
}

fn p1(lines: Vec<String>) -> usize {
    #[rustfmt::skip]
    let num_helper: HashMap<(char, char), Vec<char>> = HashMap::from([
        (('A', 'A'), vec!['A']),
        (('<', '<'), vec!['A']),
        (('>', '>'), vec!['A']),
        (('^', '^'), vec!['A']),
        (('v', 'v'), vec!['A']),

        (('A', '^'), vec!['<', 'A']),
        (('A', '>'), vec!['v', 'A']),
        (('A', 'v'), vec!['v', '<', 'A']),
        (('A', '<'), vec!['v', '<', '<', 'A']),

        (('^', 'A'), vec!['>', 'A']),
        (('^', 'v'), vec!['v', 'A']),
        (('^', '<'), vec!['v', '<', 'A']),
        (('^', '>'), vec!['v', '>', 'A']),

        (('v', 'A'), vec!['^', '>', 'A']),
        (('v', '^'), vec!['^', 'A']),
        (('v', '<'), vec!['<', 'A']),
        (('v', '>'), vec!['>', 'A']),

        (('<', 'v'), vec!['>', 'A']),
        (('<', '>'), vec!['>', '>', 'A']),
        (('<', 'A'), vec!['>', '>', '^', 'A']),
        (('<', '^'), vec!['>', '^', 'A']),

        (('>', 'v'), vec!['<', 'A']),
        (('>', '<'), vec!['<', '<', 'A']),
        (('>', 'A'), vec!['^', 'A']),
        (('>', '^'), vec!['^', '<', 'A']),
    ]);

    #[rustfmt::skip]
    let hand_drawn_paths = [
        "340A",
        "586A",
        "839A",
        "413A",
        "968A",
    ];
    // ALWAYS USE < THEN V THEN ^/>
    let hand_drawn_paths = [
        "^A<<^A>vvA>A",
        "<^^A^Av>AvvA",
        "<^^^Avv>A^^AvvvA",
        "^^<<AvA>>AvA",
        "^^^AvA<^Avvv>A",
    ];

    // #[rustfmt:<<<<:skip]
    // let hand_drawn_paths = [
    //     "029A",
    //     "980A",
    //     "179A",
    //     "456A",
    //     "379A",
    // ];
    // #[rustfmt::skip]
    // let hand_drawn_paths = [
    //     "<^A",
    //     "^<A",
    //     "",
    //     "",
    //     "",
    // ];

    let mut sum: usize = 0;
    for (i, line) in lines.iter().enumerate() {
        let numeric_code_part = line
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        let mut path: Vec<char> = hand_drawn_paths[i].chars().collect();

        for _ in 0..25 {
            path.insert(0, 'A');
            path = path
                .windows(2)
                .map(|w: &[char]| num_helper.get(&(w[0], w[1])).unwrap())
                .flat_map(|vec| vec.clone())
                .collect();
            // println!("{} {:?}", path.len(), numeric_code_part);
        }

        sum += path.len() * numeric_code_part;
    }
    sum
}

fn p2(lines: Vec<String>) -> usize {
    0
}

fn main() {
    //230750
    let lines = init(2024, 21);
    let timer = Instant::now();
    println!("{:?} | time: {:?}", p1(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p2(lines.clone()), timer.elapsed());
}
