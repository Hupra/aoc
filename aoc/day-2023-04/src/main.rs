use aoc_lib::init;

fn p1 (lines: Vec<String>) -> i32 {
    let mut sum = 0;
    
    for game in lines{
        let game_data: Vec<&str> = game
            .split(": ")
            .collect();

        let data: Vec<&str> = game_data[1]
            .split(" | ")
            .collect();
        
        let my_cards: Vec<i32> = data[0]
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let winning_cards: Vec<i32> = data[1]
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        let mut num_good_cards = 0;

        for card in my_cards{
            if winning_cards.contains(&card) {
                num_good_cards+=1;
            }
        }

        if num_good_cards > 1 {
            num_good_cards = 1 << (num_good_cards-1);
        }

        sum+=num_good_cards;
    }
    sum
}

fn p2 (lines: Vec<String>) -> i32 {

    let mut cards = vec![0; lines.len()+1];
    
    for game in lines{
        let game_data: Vec<&str> = game
            .split(": ")
            .collect();

        let game_num: usize = game_data[0]
            .split_whitespace()
            .nth(1)
            .and_then(|num| num.parse().ok())
            .unwrap();

        let data: Vec<&str> = game_data[1]
            .split(" | ")
            .collect();
        
        let my_cards: Vec<i32> = data[0]
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let winning_cards: Vec<i32> = data[1]
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        let mut num_good_cards = 0;

        for card in my_cards{
            if winning_cards.contains(&card) {
                num_good_cards+=1;
            }
        }

        // Add the card itself
        cards[game_num]+=1;

        // for each good card, add one card to the
        // next num_good_cards cards.
        for i in 1..=num_good_cards{
            cards[game_num+i]+=cards[game_num];
        }
    }
    return cards.iter().sum();
}

fn main() {

    let lines = init(2023, 4);
    // stuff here
    println!("{:?}", p1(lines.clone()));
    println!("{:?}", p2(lines.clone()));
    
}
