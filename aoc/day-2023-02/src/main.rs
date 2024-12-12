use aoc_lib::init;
use std::cmp;

fn p1 (lines: Vec<String>) -> i32 {
    let mut sum: i32 = 0;

    for game in lines{
        let game_data: Vec<&str> = game.split(": ").collect();
        let game_num: i32 = game_data[0].split(" ").nth(1).unwrap_or_default().parse().unwrap_or_default();

        let mut valid: bool = true;

        for set in game_data[1].split("; ") {
            for draw in set.split(", "){
                
                let num_and_color: Vec<&str> = draw.split(" ").collect();
                let num: i32 = num_and_color[0].parse().unwrap_or_default();
                let color: &str = num_and_color[1];

                match color {
                    "red"   if num > 12 => valid = false,
                    "green" if num > 13 => valid = false,
                    "blue"  if num > 14 => valid = false,
                    _ => {}
                }
            }
        }
        if valid {
            sum += game_num;
        }
    }
    return sum;
}

fn p2 (lines: Vec<String>) -> i32 {
    let mut sum: i32 = 0;

    for game in lines{
        let game_data: Vec<&str> = game.split(": ").collect();

        let mut r = 0;
        let mut b = 0;
        let mut g = 0;

        for set in game_data[1].split("; ") {
            for draw in set.split(", "){
                
                let num_and_color: Vec<&str> = draw.split(" ").collect();
                let num: i32 = num_and_color[0].parse().unwrap_or_default();
                let color: &str = num_and_color[1];

                match color {
                    "red"   => r = cmp::max(r, num),
                    "green" => g = cmp::max(g, num),
                    "blue"  => b = cmp::max(b, num),
                    _ => {}
                }
            }
        }
        sum += r * g * b;
    }
    return sum;
}


fn main() {

    let lines = init(2023, 2);
    // stuff here
    println!("{:?}", p1(lines.clone()));
    println!("{:?}", p2(lines.clone()));

    return ()
}
