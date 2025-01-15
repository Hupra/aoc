use aoc_lib::init;
use cached::proc_macro::cached;
use itertools::Itertools;
use rustlind_lib::*;
use std::{ iter::zip, time::Instant };

fn transpose_chunk(chunk: &[String], default: char) -> Vec<String> {
    // Determine the maximum length of the strings in the chunk
    let max_length = chunk
        .iter()
        .map(|s| s.len())
        .max()
        .unwrap_or(0);

    // Initialize a vector to hold the transposed lines
    let mut transposed = Vec::with_capacity(max_length);

    // Iterate over each character position
    for i in 0..max_length {
        let mut transposed_line = String::new();
        for line in chunk {
            // Push the character if it exists, otherwise push a space or another placeholder
            let ch = line.chars().nth(i).unwrap_or(default);
            transposed_line.push(ch);
        }
        transposed.push(transposed_line);
    }

    transposed
}

fn solve(lines: Vec<String>) -> Option<usize> {
    let chunks: Vec<Vec<String>> = lines
        .split(|line| line.is_empty())
        .map(|chunk| chunk.to_vec())
        .collect();

    let transposed_chunks: Vec<Vec<String>> = chunks
        .into_iter()
        .map(|chunk| transpose_chunk(&chunk, '.'))
        .collect();

    let (locks, keys): (Vec<Vec<String>>, Vec<Vec<String>>) = transposed_chunks
        .into_iter()
        .partition(|chunk| chunk.first().map_or(false, |line| line.starts_with('#')));

    let lock_counts: Vec<Vec<usize>> = locks
        .iter()
        .map(|lock|
            lock
                .iter()
                .map(|line|
                    line
                        .chars()
                        .filter(|&c| c == '#')
                        .count()
                )
                .collect()
        )
        .collect();

    let key_counts: Vec<Vec<usize>> = keys
        .iter()
        .map(|lock|
            lock
                .iter()
                .map(|line|
                    line
                        .chars()
                        .filter(|&c| c == '#')
                        .count()
                )
                .collect()
        )
        .collect();

    let combinations = lock_counts
        .into_iter()
        .cartesian_product(key_counts.into_iter())
        .filter(|(lock, key)| zip(lock, key).all(|(&l, &k)| l + k <= 7))
        .count();

    Some(combinations)
}

fn main() {
    let lines = init(2024, 25);
    let timer = Instant::now();
    println!("{:?} | time: {:?}", solve(lines.clone()), timer.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test_lines() -> Vec<String> {
        include_str!("test.txt").lines().map(String::from).collect()
    }

    #[test]
    fn test_solve() {
        let expected = Some(3);
        let actual = solve(test_lines());
        assert_eq!(actual, expected);
    }
}
