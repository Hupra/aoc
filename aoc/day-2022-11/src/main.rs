use aoc_lib::init;
use cached::proc_macro::cached;
use itertools::Itertools;
use rustlind_lib::*;
use std::time::Instant;

struct Monkey<F, T> where F: Fn(usize) -> usize, T: Fn(usize) -> usize {
    id: usize,
    items: Vec<usize>,
    div: usize,
    operation: F,
    test: T,
}

impl<F, T> std::fmt::Debug for Monkey<F, T> where F: Fn(usize) -> usize, T: Fn(usize) -> usize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey").field("id", &self.id).field("items", &self.items).finish()
    }
}

fn get_monkies(
    lines: Vec<String>
) -> Vec<Monkey<impl Fn(usize) -> usize, impl Fn(usize) -> usize>> {
    let mut monkies = lines
        .split(|line| line.is_empty())
        .map(|m| {
            let id = *re_nums(&m[0]).first().unwrap() as usize;
            let items = re_nums(&m[1])
                .iter()
                .map(|&i| i as usize)
                .collect_vec();
            let opration_input = &m[2][23..];
            let (sign, to) = opration_input.split_whitespace().collect_tuple().unwrap_or_default();

            let sign_operation = match sign {
                "*" => |x: usize, y: usize| x * y,
                "+" => |x: usize, y: usize| x + y,
                _ => |_x: usize, _y: usize| 0,
            };

            let to = to.to_string();
            let operation = move |old: usize| {
                let x = to.parse::<usize>().unwrap_or(old);
                sign_operation(old, x)
            };

            let div = *re_nums(&m[3]).first().unwrap() as usize;
            let t = *re_nums(&m[4]).first().unwrap() as usize;
            let f = *re_nums(&m[5]).first().unwrap() as usize;
            let test = move |x: usize| if x % div == 0 { t } else { f };
            Monkey { id, items, div, operation, test }
        })
        .collect_vec();
    monkies
}

fn p1(lines: Vec<String>) -> Option<usize> {
    let mut monkies = get_monkies(lines);
    let mut chase = vec![0; monkies.len()];

    for _ in 0..20 {
        for m_index in 0..monkies.len() {
            let moves: Vec<(usize, usize)> = monkies[m_index].items
                .iter()
                .map(|&item| {
                    let new = (monkies[m_index].operation)(item) / 3;
                    let throw_to = (monkies[m_index].test)(new);
                    (throw_to, new)
                })
                .collect();

            chase[m_index] += moves.len();

            monkies[m_index].items.clear();
            for (throw_to, new) in moves {
                monkies[throw_to].items.push(new);
            }
        }
    }

    chase.sort();
    chase.reverse();

    Some(chase[0] * chase[1])
}

fn p2(lines: Vec<String>) -> Option<usize> {
    let mut monkies = get_monkies(lines);
    let mut chase = vec![0; monkies.len()];

    let lcm = monkies
        .iter()
        .map(|m| m.div)
        .reduce(|a, b| a * b)
        .unwrap_or_default();

    for _ in 0..10_000 {
        for m_index in 0..monkies.len() {
            let moves: Vec<(usize, usize)> = monkies[m_index].items
                .iter()
                .map(|&item| {
                    let new = (monkies[m_index].operation)(item) % lcm;
                    let throw_to = (monkies[m_index].test)(new);
                    (throw_to, new)
                })
                .collect();

            chase[m_index] += moves.len();

            monkies[m_index].items.clear();
            for (throw_to, new) in moves {
                monkies[throw_to].items.push(new);
            }
        }
    }

    chase.sort();
    chase.reverse();

    Some(chase[0] * chase[1])
}

fn main() {
    let lines = init(2022, 11);
    let timer = Instant::now();
    println!("{:?} | time: {:?}", p1(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p2(lines.clone()), timer.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let expected = Some(10605);
        let actual = p1(include_str!("test.txt").lines().map(String::from).collect());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_p2() {
        let expected = Some(2713310158);
        let actual = p2(include_str!("test.txt").lines().map(String::from).collect());
        assert_eq!(actual, expected);
    }
}
