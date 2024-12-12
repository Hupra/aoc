use aoc_lib::init;

fn p1(lines: Vec<String>) -> usize {
    let mut sum: usize = 0;

    let data: Vec<&str> = lines[0].split(",").collect();

    for code in data {
        let mut cur: usize = 0;
        for c in code.chars() {
            cur += c as usize;
            cur *= 17;
            cur %= 256;
        }
        sum += cur;
    }
    sum
}

fn p2(lines: Vec<String>) -> usize {
    let mut sum: usize = 0;

    let data: Vec<&str> = lines[0].split(",").collect();

    let mut d: Vec<Vec<String>> = vec![Vec::<String>::new(); 256];

    for code in data {
        let mut sid: usize = 0;

        for i in 0..code.len() {
            let c = code.chars().nth(i).unwrap();
            if c == '-' || c == '=' {
                sid = i;
            }
        }

        let mut boxx: usize = 0;
        let lab = &code[..sid];

        for c in lab.chars() {
            boxx += c as usize;
            boxx *= 17;
            boxx %= 256;
        }

        if code.chars().nth(sid).unwrap() == '-' {
            d[boxx] = d[boxx]
                .iter()
                .filter(|s| !s.starts_with(lab))
                .map(|s| s.clone())
                .collect();
        } else {
            let mut present: Option<usize> = None;
            for j in 0..d[boxx].len() {
                if d[boxx][j].starts_with(lab) {
                    present = Some(j);
                }
            }
            if let Some(present) = present {
                d[boxx][present] = code.to_string();
            } else {
                d[boxx].push(code.to_string());
            }
        }
    }
    for i in 0..256 as usize {
        for j in 0..d[i].len() {
            let boxx: usize = i + 1;
            let slot: usize = j + 1;
            let focl: usize = d[i][j].split('=').nth(1).unwrap().parse().unwrap();
            sum += boxx * slot * focl;
        }
    }
    return sum;
}

fn main() {
    let lines = init(2023, 15);
    // stuff here
    println!("{:?}", p1(lines.clone()));
    println!("{:?}", p2(lines.clone()));
}
