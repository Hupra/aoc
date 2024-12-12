// use ahash::AHasher;
use csv::ReaderBuilder;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let args: Vec<String> = env::args().collect();
    let input: usize = match args[1].parse::<usize>() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: Please provide a valid integer.");
            std::process::exit(1);
        }
    };

    fn string_to_bin(s: &str) -> Option<u32> {
        let mut bitmask = 0;
        for c in s.chars() {
            if ('a'..='z').contains(&c) {
                bitmask |= 1 << (c as u32 - 'a' as u32);
            } else {
                return None;
            }
        }
        Some(bitmask)
    }

    let letter_frequency: HashMap<char, u64> = HashMap::from([
        ('e', 349588141984),
        ('t', 247577342738),
        ('a', 243662684512),
        ('o', 228025627088),
        ('i', 223353030415),
        ('n', 207910712159),
        ('s', 207080253606),
        ('r', 201896673641),
        ('l', 130649920346),
        ('c', 113913698859),
        ('d', 107605388542),
        ('h', 106367962556),
        ('u', 86950627146),
        ('m', 84155576549),
        ('p', 77553040250),
        ('g', 63045208347),
        ('f', 61328927423),
        ('y', 52941043438),
        ('b', 49798922187),
        ('w', 44294405401),
        ('v', 34402346309),
        ('k', 24380950863),
        ('x', 9151143994),
        ('j', 7637833834),
        ('q', 4218467887),
        ('z', 4192477980),
    ]);

    let mut sorted_letter_frequency: Vec<(char, u64)> = letter_frequency.into_iter().collect();
    sorted_letter_frequency.sort_by(|a: &(char, u64), b: &(char, u64)| b.1.cmp(&a.1));

    let word_length: usize = input;
    // let word_length: usize = 6;

    let desired_letters: String = sorted_letter_frequency
        .into_iter()
        .take(word_length * 3)
        .map(|p| p.0)
        .collect::<Vec<char>>()
        .iter()
        .collect();

    let dl_bin: u32 = string_to_bin(&desired_letters).unwrap_or(0);

    println!("{} {}", desired_letters, dl_bin);

    let file_path: &str = "/workspace/aoc/henrik/unigram_freq.csv";
    let mut rdr: csv::Reader<std::fs::File> = ReaderBuilder::new()
        .has_headers(true)
        .from_path(file_path)
        .unwrap();

    let old_word_collection: Vec<String> = rdr
        .records()
        .filter_map(|result| {
            result
                .ok()
                .and_then(|record| record.get(0).map(|word| word.to_string()))
        })
        .filter(|word| word.len() == word_length)
        .filter(|word| word.chars().collect::<HashSet<char>>().len() == word_length)
        .collect();

    println!("word collection length: {}", old_word_collection.len());

    let mut bin_lookup: HashMap<u32, Vec<String>> = HashMap::new();
    let mut word_collection: HashSet<u32> = HashSet::new();

    for s in old_word_collection {
        match string_to_bin(&s) {
            Some(s_bin) if (s_bin & dl_bin) == s_bin => {
                bin_lookup.entry(s_bin).or_default().push(s.clone());
                word_collection.insert(s_bin);
            }
            _ => {}
        }
    }

    // println!("reduced word collection length: {}", word_collection.len());
    // let wc: Vec<u32> = word_collection.into_iter().collect();
    // let mut result: Vec<(u32, u32, u32)> = Vec::new();
    // for ai in 0..wc.len() {
    //     for bi in (ai + 1)..wc.len() {
    //         if (wc[ai] & wc[bi]) == 0 {
    //             let ab = wc[ai] | wc[bi];
    //             for ci in (bi + 1)..wc.len() {
    //                 let combined_letters = ab | wc[ci];
    //                 if combined_letters == dl_bin {
    //                     result.push((wc[ai], wc[bi], wc[ci]));
    //                 }
    //             }
    //         }
    //     }
    // }

    let wc: Vec<u32> = word_collection.into_iter().collect();
    let wc_set: HashSet<u32> = wc.iter().cloned().collect();
    let mut seen_triplets: HashSet<(u32, u32, u32)> = HashSet::new();

    let mut result: Vec<(u32, u32, u32)> = Vec::new();
    for ai in 0..wc.len() {
        let a = wc[ai];
        for bi in (ai + 1)..wc.len() {
            if (a & wc[bi]) == 0 {
                let ab = a | wc[bi];
                let c = dl_bin & !ab;
                if wc_set.contains(&c) {
                    let mut triplet = vec![a, wc[bi], c];
                    triplet.sort();
                    let triplet = (triplet[0], triplet[1], triplet[2]);

                    if !seen_triplets.contains(&triplet) {
                        result.push(triplet);
                        seen_triplets.insert(triplet);
                    }
                }
            }
        }
    }

    println!("result: {}", result.len());

    let mut options: Vec<(String, String, String)> = Vec::new();
    for (a, b, c) in &result {
        if let (Some(w1_list), Some(w2_list), Some(w3_list)) =
            (bin_lookup.get(a), bin_lookup.get(b), bin_lookup.get(c))
        {
            for w1 in w1_list {
                for w2 in w2_list {
                    for w3 in w3_list {
                        options.push((w1.clone(), w2.clone(), w3.clone()));
                    }
                }
            }
        }
    }

    println!("options: {}", options.len());
    println!("First 5:");
    for x in options.iter().take(5) {
        println!("{} {} {}", x.0, x.1, x.2);
    }

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}
