use aoc_lib::init;
use itertools::Itertools;
use std::{ collections::HashSet, time::Instant, vec };

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Pos {
    i: i32,
    j: i32,
}

impl Pos {
    fn new(i: i32, j: i32) -> Self {
        Self { i, j }
    }
}

fn dir(c: char) -> Pos {
    match c {
        'D' => Pos::new(-1, 0),
        'U' => Pos::new(1, 0),
        'R' => Pos::new(0, 1),
        'L' => Pos::new(0, -1),
        _ => panic!(),
    }
}

fn p1(lines: Vec<String>) -> Option<usize> {
    let mut head = Pos::new(0, 0);
    let mut tail = head;
    let mut head_old;
    let mut visited: HashSet<Pos> = HashSet::new();
    visited.insert(tail);

    for line in lines {
        let (c, n) = line.split_whitespace().next_tuple().unwrap();
        let c = c.chars().next().unwrap();
        let n = n.parse::<i32>().unwrap();

        for _ in 0..n {
            // dbg!(head, tail);
            // println!("---");
            let d = dir(c);
            head_old = head;
            head = Pos::new(head.i + d.i, head.j + d.j);

            let diff_i = (head.i - tail.i).abs();
            let diff_j = (head.j - tail.j).abs();
            let diff = diff_i + diff_j;
            if diff >= 3 || diff_i >= 2 || diff_j >= 2 {
                tail = head_old;
                visited.insert(tail);
            }
        }
    }

    Some(visited.len())
}

fn p2(lines: Vec<String>) -> Option<usize> {
    let snake_length = 10;
    let mut snake = vec![Pos::new(0,0); snake_length];
    let mut visited: HashSet<Pos> = HashSet::from([Pos::new(0, 0)]);

    for line in lines {
        let (c, n) = line.split_whitespace().next_tuple().unwrap();
        let c = c.chars().next().unwrap();
        let n = n.parse::<i32>().unwrap();

        let d = dir(c);

        for _ in 0..n {
            snake[0] = Pos::new(snake[0].i + d.i, snake[0].j + d.j);

            for i in 0..snake.len() - 1 {
                let head = snake[i];
                let tail = snake[i + 1];

                let diff_i = head.i - tail.i;
                let diff_j = head.j - tail.j;

                if diff_i.abs() >= 2 || diff_j.abs() >= 2 {
                    let di = diff_i.signum();
                    let dj = diff_j.signum();
                    snake[i + 1] = Pos::new(tail.i + di, tail.j + dj);
                }
            }
            visited.insert(snake.last().unwrap().clone());
        }
    }

    Some(visited.len())
}

fn main() {
    let lines: Vec<String> = init(2022, 9);
    let timer: Instant = Instant::now();
    println!("{:?} | time: {:?}", p1(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p2(lines.clone()), timer.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let expected = Some(13);
        let actual = p1(include_str!("test.txt").lines().map(String::from).collect());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_p2() {
        let expected = Some(36);
        let actual = p2(include_str!("test2.txt").lines().map(String::from).collect());
        assert_eq!(actual, expected);
    }
}
