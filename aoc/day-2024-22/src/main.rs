use aoc_lib::init;
use cached::proc_macro::cached;
use itertools::Itertools;
use rustlind_lib::*;
use std::{ cmp::max, collections::{ btree_map::Keys, HashMap }, time::Instant };

macro_rules! prune {
    ($secret:expr) => {
        $secret % 16777216
    };
}

macro_rules! mix {
    ($secret:expr, $value:expr) => {
        $value ^ $secret
    };
}

fn process(mut x: usize) -> usize {
    x = mix!(x, x * 64);
    x = prune!(x);
    x = mix!(x, x / 32);
    x = prune!(x);
    x = mix!(x, x * 2048);
    x = prune!(x);
    x
}

fn p1(lines: Vec<String>) -> Option<usize> {
    let mut sum: usize = 0;

    for line in lines {
        let mut secret: usize = line.parse().unwrap_or_default();

        for _ in 0..2000 {
            secret = process(secret);
        }

        sum += secret;
    }

    Some(sum)
}

fn p2(lines: Vec<String>) -> Option<usize> {
    let mut global_hm: HashMap<(i32, i32, i32, i32), usize> = HashMap::new();

    for line in lines {
        let mut secret: usize = line.parse().unwrap_or_default();
        let mut prices: Vec<usize> = Vec::new();
        let mut price_changes: Vec<i32> = Vec::new();
        let mut local_hm: HashMap<(i32, i32, i32, i32), usize> = HashMap::new();

        for _ in 0..2000 {
            let new_secret = process(secret);
            let old_price = secret % 10;
            let new_price = new_secret % 10;

            prices.push(new_price);
            price_changes.push((new_price as i32) - (old_price as i32));

            secret = new_secret;
        }

        for i in 3..2000 as usize {
            let key = (
                price_changes[i - 3],
                price_changes[i - 2],
                price_changes[i - 1],
                price_changes[i],
            );
            let val = prices[i];

            local_hm.entry(key.clone()).or_insert(val);
        }

        for (key, val) in local_hm {
            *global_hm.entry(key).or_default() += val;
        }
    }

    global_hm
        .iter()
        .max_by_key(|&(_key, &value)| value)
        .map(|(_key, val)| {
            println!("{:?}", _key);
            *val
        })
}

fn main() {
    let lines = init(2024, 22);
    let timer = Instant::now();

    // let lines = vec!["123".to_string()];
    // 2237 too high
    println!("{:?} | time: {:?}", p1(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p2(lines.clone()), timer.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let actual = p1(include_str!("test.txt").lines().map(String::from).collect());
        let expected = Some(37327623);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_p2() {
        let actual = p2(include_str!("testp2.txt").lines().map(String::from).collect());
        let expected = Some(23);
        assert_eq!(actual, expected);
    }
}
