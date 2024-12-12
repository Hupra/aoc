use aoc_lib::init;

fn p1(lines: Vec<String>) -> i32 {
    let time = lines[0].split(": ").collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<usize>>();

    let dist = lines[1].split(": ").collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<usize>>();

    let mut sum = 1;

    for i in 0..time.len() {
        let mut wins = 0;

        for j in 0..time[i] {
            if j * (time[i] - j) > dist[i] {
                wins += 1;
            }
        }

        sum *= wins;
    }

    return sum;
}

fn p2(lines: Vec<String>) -> usize {
    let time = lines[0].split(": ").collect::<Vec<&str>>()[1]
        .replace(" ", "")
        .parse::<usize>()
        .unwrap();

    let dist = lines[1].split(": ").collect::<Vec<&str>>()[1]
        .replace(" ", "")
        .parse::<usize>()
        .unwrap();

    let mut wins = 0;

    for j in 0..time {
        if j * (time - j) > dist {
            wins += 1;
        }
    }

    return wins;
}

fn p3(lines: Vec<String>) -> i64 {
    let time = lines[0].split(": ").collect::<Vec<&str>>()[1]
        .replace(" ", "")
        .parse::<usize>()
        .unwrap();

    let dist = lines[1].split(": ").collect::<Vec<&str>>()[1]
        .replace(" ", "")
        .parse::<usize>()
        .unwrap();

    let a: f64 = 1.0;
    let b: f64 = -(time as f64);
    let c: f64 = dist as f64;
    let d: f64 = b.powi(2) - 4.0 * a * c;

    let x1: f64 = (-b - d.sqrt()) / (2.0 * a);
    let x2: f64 = (-b + d.sqrt()) / (2.0 * a);

    return (x2 as i64) - (x1.ceil() as i64) + 1;
}

fn main() {
    let lines = init(2023, 6);
    // stuff here
    println!("{:?}", p1(lines.clone()));
    println!("{:?}", p2(lines.clone()));
    println!("{:?}", p3(lines.clone()));
}
