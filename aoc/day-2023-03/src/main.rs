use aoc_lib::init;
use std::collections::HashMap;

fn safe_get<T, I>(m: &[Vec<T>], i: I, j: I) -> Option<&T>
where
    I: TryInto<usize>,
{
    let i: usize = i.try_into().ok()?;
    let j: usize = j.try_into().ok()?;

    if i >= m.len() || j >= m[i].len() {
        return None;
    }

    Some(&m[i][j])
}

fn p1(lines: Vec<String>) -> i32 {
    let mut sum = 0;

    let m: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();

    for i in 0..lines.len() as i32 {
        let mut j: i32 = 0;
        while j < lines.len() as i32 {
            let word_start = j;
            let mut k = 0;

            while safe_get(&m, i, j).map_or(false, |c| c.is_digit(10)) {
                j += 1;
                k += 1;
            }

            if k > 0 {
                j -= 1;
                let mut valid = false;

                for ii in i - 1..=i + 1 {
                    for jj in word_start - 1..=j + 1 {
                        if let Some(c) = safe_get(&m, ii, jj) {
                            if *c != '.' && !c.is_digit(10) {
                                valid = true;
                            }
                        }
                    }
                }
                if valid {
                    let h = &m[i as usize][word_start as usize..=j as usize]
                        .iter()
                        .collect::<String>()
                        .parse::<i32>()
                        .unwrap_or(0);
                    sum += h;
                }
            }
            j += 1;
        }
    }

    return sum;
}

fn p2(lines: Vec<String>) -> i32 {
    let mut sum = 0;

    let m: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();

    let mut dic: HashMap<(i32, i32), Vec<i32>> = HashMap::new();

    for i in 0..lines.len() as i32 {
        let mut j: i32 = 0;
        while j < lines.len() as i32 {
            let word_start = j;
            let mut k = 0;

            while safe_get(&m, i, j).map_or(false, |c| c.is_digit(10)) {
                j += 1;
                k += 1;
            }

            if k > 0 {
                j -= 1;

                for ii in i - 1..=i + 1 {
                    for jj in word_start - 1..=j + 1 {
                        if let Some(c) = safe_get(&m, ii, jj) {
                            if *c == '*' {
                                let pair = (ii, jj);
                                let val = lines[i as usize][word_start as usize..=j as usize]
                                    .parse::<i32>()
                                    .unwrap_or(0);
                                dic.entry(pair).or_default().push(val);
                            }
                        }
                    }
                }
            }
            j += 1;
        }
    }

    for cluster in dic.values().into_iter() {
        if cluster.len() == 2 {
            sum += cluster[0] * cluster[1];
        }
    }

    return sum;
}

fn main() {
    let lines = init(2023, 3);
    // stuff here
    println!("{:?}", p1(lines.clone()));
    println!("{:?}", p2(lines.clone()));

    return ();
}
