use aoc_lib::init;

fn p1(lines: Vec<String>) -> i32 {
    let mut res: i32 = 0;

    for line in &lines {
        let mut first = None;
        let mut last = None;

        for c in line.chars() {
            if c.is_digit(10) {
                if first.is_none() {
                    first = Some(c); // First digit
                }
                last = Some(c); // Last digit (will update until the last digit is found)
            }
        }

        let result = match (first, last) {
            (Some(first), Some(last)) => format!("{}{}", first, last),
            _ => String::new(), // Return an empty string if no digits were found
        };

        res += result.parse::<i32>().unwrap_or(10000);
    }
    res
}

fn p2(lines: Vec<String>) -> u32 {
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut sum = 0;

    for line in &lines {
        let chars: Vec<char> = line.chars().collect();

        let mut f: Option<u32> = None;
        let mut l: Option<u32> = None;

        for i in 0..line.len() {
            if chars[i].is_digit(10) {
                if f.is_none() {
                    f = chars[i].to_digit(10); // First digit
                }
                l = chars[i].to_digit(10); // Last digit (will update until the last digit is found)
            }

            for (j, snum) in numbers.iter().enumerate() {
                if snum.len() + i <= line.len() {
                    let ss = line[i..i + snum.len()].to_string();
                    if snum == &ss {
                        if f.is_none() {
                            f = Some(1 + j as u32);
                        }
                        l = Some(1 + j as u32);
                    }
                }
            }
        }

        if let (Some(first), Some(last)) = (&f, &l) {
            sum += (first.to_string() + &last.to_string())
                .parse::<u32>()
                .unwrap_or_default();
        }
    }

    return sum;
}

fn main() {
    let lines = init(2023, 1);

    // returns empty line
    // let lines: Vec<&str> = data.split('\n').collect();
    // .iter().map(|x| x.to_string()).collect()

    println!("{}", p1(lines.clone()));
    println!("{}", p2(lines.clone()));

    return ();
}
