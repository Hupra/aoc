use aoc_lib::init;

fn p1(lines: Vec<String>) -> i64 {
    let mut res: Vec<i64> = Vec::new();

    for line in lines {
        let nums: Vec<i64> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let tmp = next(nums);
        res.push(tmp);
    }

    res.iter().sum()
}

fn p2(lines: Vec<String>) -> i64 {
    let mut res: Vec<i64> = Vec::new();

    for line in lines {
        let nums: Vec<i64> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .rev()
            .collect();
        let tmp = next(nums);
        res.push(tmp);
    }

    res.iter().sum()
}

fn next(nums: Vec<i64>) -> i64 {
    if nums.iter().filter(|&p| *p == 0).count() == nums.len() {
        return 0;
    }

    let mut nlist: Vec<i64> = Vec::new();
    for i in 1..nums.len() {
        nlist.push(nums[i] - nums[i - 1]);
    }

    let nxt = next(nlist) + nums[nums.len() - 1];
    return nxt;
}

fn main() {
    let lines = init(2023, 9);
    // stuff here
    println!("{:?}", p1(lines.clone()));
    println!("{:?}", p2(lines.clone()));
}
