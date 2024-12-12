use aoc_lib::init;
use num::iter;
use rustlind_lib::*;
use std::{
    cmp::{self, max, min},
    collections::HashSet,
    iter::zip,
    time::Instant,
};

fn solve(lines: Vec<String>) {
    let mut bricks: Vec<((usize, usize, usize), (usize, usize, usize))> = Vec::new();
    //
    //      0 1 2
    //    _______
    // 0  | x y z
    // 1  | X Y Z

    for brick in lines {
        let ab = fsplit(&brick, "~")
            .iter()
            .map(|s| re_nums(s))
            .collect::<Vec<Vec<i32>>>();

        let (c, f): (Vec<usize>, Vec<usize>) = zip(&ab[0], &ab[1])
            .map(|(&ai, &bi)| (ai.min(bi) as usize, ai.max(bi) as usize))
            .unzip();

        bricks.push(((c[0], c[1], c[2]), (f[0], f[1], f[2])));
    }

    bricks.sort_by_key(|&b| b.0 .2);

    let (mut x, mut y, mut z) = (0, 0, 0);
    for brick in &bricks {
        x = x.max(brick.1 .0);
        y = y.max(brick.1 .1);
        z = z.max(brick.1 .2);
    }
    let mut m = vec![vec![vec![0 as usize; z + 2]; y + 1]; x + 1];

    for i in 0..bricks.len() {
        // Find brick location
        let brick = &mut bricks[i];
        let mut z: usize = brick.0 .2;

        'scan: while z > 0 {
            z -= 1;
            for x in (brick.0 .0)..=(brick.1 .0) {
                for y in (brick.0 .1)..=(brick.1 .1) {
                    if m[x][y][z] != 0 {
                        break 'scan;
                    }
                }
            }
        }
        z += 1;

        // Update brick
        let nz = z + brick.1 .2 - brick.0 .2;
        brick.0 .2 = z;
        brick.1 .2 = nz;

        // Add brick to m
        for x in (brick.0 .0)..=(brick.1 .0) {
            for y in (brick.0 .1)..=(brick.1 .1) {
                for z in (brick.0 .2)..=(brick.1 .2) {
                    m[x][y][z] = i + 1;
                }
            }
        }
    }

    // find bricks above and below each brick
    let mut zrelations: Vec<(HashSet<usize>, HashSet<usize>)> = Vec::new();

    for i in 0..bricks.len() {
        let brick = &mut bricks[i];
        let mut bricks_above: HashSet<usize> = HashSet::new();
        let mut bricks_below: HashSet<usize> = HashSet::new();
        let zabove = brick.1 .2 + 1;
        let zbelow = brick.0 .2 - 1;
        for x in (brick.0 .0)..=(brick.1 .0) {
            for y in (brick.0 .1)..=(brick.1 .1) {
                if m[x][y][zabove] != 0 {
                    bricks_above.insert(m[x][y][zabove]);
                }
                if m[x][y][zbelow] != 0 {
                    bricks_below.insert(m[x][y][zbelow]);
                }
            }
        }
        zrelations.push((bricks_above, bricks_below));
    }
    // check if brick can be disintegrated
    let mut counter: usize = 0;
    for i in 0..bricks.len() {
        let mut disintegrable = true;
        for above in zrelations[i].0.iter() {
            if zrelations[above - 1].1.len() == 1 {
                disintegrable = false;
            }
        }
        if disintegrable {
            counter += 1;
        }
    }
    println!("{}", counter);

    fn func(
        mut brickset: HashSet<usize>,
        zrelations: &Vec<(HashSet<usize>, HashSet<usize>)>,
    ) -> usize {
        loop {
            let leng = brickset.len();
            let mut brickparents: HashSet<usize> = HashSet::new();
            for i in brickset.iter() {
                brickparents.extend(&zrelations[i - 1].0);
            }
            for i in brickparents {
                if zrelations[i - 1].1.is_subset(&brickset) {
                    brickset.insert(i);
                }
            }
            if brickset.len() == leng {
                return brickset.len() - 1;
            }
        }
    }
    let mut counter: usize = 0;

    for i in 0..bricks.len() {
        counter += func(HashSet::from([i + 1]), &zrelations)
    }

    println!("{}", counter);
}

fn main() {
    let lines = init(2023, 22);
    let timer = Instant::now();
    solve(lines.clone());
    println!("time: {:?}", timer.elapsed());
}
