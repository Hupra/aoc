use aoc_lib::init;

fn expand_row(m: &Vec<Vec<usize>>, i: usize) -> bool {
    m[i].iter().all(|&x| x == 0)
}
fn expand_col(m: &Vec<Vec<usize>>, j: usize) -> bool {
    m.iter().map(|row| row[j]).sum::<usize>() == 0
}

fn p1(lines: Vec<String>) -> usize {
    let mut m: Vec<Vec<usize>> = vec![vec![0; lines[0].len()]; lines.len()];
    let mut galaxies: Vec<(usize, usize)> = Vec::from([(0, 0)]);
    let mut id: usize = 0;

    for i in 0..lines.len() {
        for j in 0..lines[0].len() {
            if lines[i].chars().nth(j).unwrap() == '#' {
                id += 1;
                m[i][j] = id;
                galaxies.push((i, j));
            }
        }
    }

    // add 1 to galaxy for each row that must be expanded
    for i in (0..m.len()).rev() {
        if expand_row(&m, i) {
            for g in 1..galaxies.len() {
                let gx = galaxies[g];
                if gx.0 > i {
                    galaxies[g] = (gx.0 + 1, gx.1)
                }
            }
        }
    }

    // add 1 to galaxy for each col that must be expanded
    for j in (0..m[0].len()).rev() {
        if expand_col(&m, j) {
            for g in 1..galaxies.len() {
                let gx = galaxies[g];
                if gx.1 > j {
                    galaxies[g] = (gx.0, gx.1 + 1)
                }
            }
        }
    }

    let mut sum: usize = 0;

    for i in 1..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            let h = (galaxies[j].0 as i64 - galaxies[i].0 as i64).abs() as usize;
            let y = (galaxies[j].1 as i64 - galaxies[i].1 as i64).abs() as usize;
            sum += h + y;
        }
    }

    sum
}

fn p2(lines: Vec<String>) -> usize {
    let mut m: Vec<Vec<usize>> = vec![vec![0; lines[0].len()]; lines.len()];
    let mut galaxies: Vec<(usize, usize)> = Vec::from([(0, 0)]);
    let mut id: usize = 0;

    for i in 0..lines.len() {
        for j in 0..lines[0].len() {
            if lines[i].chars().nth(j).unwrap() == '#' {
                id += 1;
                m[i][j] = id;
                galaxies.push((i, j));
            }
        }
    }

    for i in (0..m.len()).rev() {
        if expand_row(&m, i) {
            for g in 1..galaxies.len() {
                let gx = galaxies[g];
                if gx.0 > i {
                    galaxies[g] = (gx.0 + 999_999, gx.1)
                }
            }
        }
    }

    for j in (0..m[0].len()).rev() {
        if expand_col(&m, j) {
            for g in 1..galaxies.len() {
                let gx = galaxies[g];
                if gx.1 > j {
                    galaxies[g] = (gx.0, gx.1 + 999_999)
                }
            }
        }
    }

    let mut sum: usize = 0;

    for i in 1..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            let h = (galaxies[j].0 as i64 - galaxies[i].0 as i64).abs() as usize;
            let y = (galaxies[j].1 as i64 - galaxies[i].1 as i64).abs() as usize;
            sum += h + y;
        }
    }

    sum
}

fn main() {
    let lines = init(2023, 11);
    // stuff here
    println!("{:?}", p1(lines.clone()));
    println!("{:?}", p2(lines.clone()));
}
