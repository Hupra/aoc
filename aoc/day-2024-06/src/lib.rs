use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn turn_right(self) -> Dir {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }

    fn forward_delta(self) -> (isize, isize) {
        match self {
            Dir::Up => (-1, 0),
            Dir::Right => (0, 1),
            Dir::Down => (1, 0),
            Dir::Left => (0, -1),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct State {
    x: usize,
    y: usize,
    dir: Dir,
}

fn parse_map(lines: &[String]) -> (Vec<Vec<char>>, (usize, usize), Dir) {
    let height = lines.len();
    let width = lines[0].len();
    let mut map = vec![vec!['.'; width]; height];

    let mut start_pos = (0, 0);
    let mut start_dir = Dir::Up;

    for (i, line) in lines.iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            map[i][j] = ch;
            if ch == '^' || ch == 'v' || ch == '<' || ch == '>' {
                start_pos = (i, j);
                start_dir = match ch {
                    '^' => Dir::Up,
                    '>' => Dir::Right,
                    'v' => Dir::Down,
                    '<' => Dir::Left,
                    _ => unreachable!(),
                };
                // Replace the guard symbol with '.' since it's effectively open floor
                map[i][j] = '.';
            }
        }
    }

    (map, start_pos, start_dir)
}

fn in_bounds(x: isize, y: isize, h: usize, w: usize) -> bool {
    x >= 0 && y >= 0 && (x as usize) < h && (y as usize) < w
}

/// Simulate the guard's movement until leaving the map.
/// Returns a set of visited positions.
fn simulate(
    map: &Vec<Vec<char>>,
    start_pos: (usize, usize),
    start_dir: Dir,
) -> HashSet<(usize, usize)> {
    let h = map.len();
    let w = map[0].len();
    let mut visited = HashSet::new();
    let (mut x, mut y) = start_pos;
    let mut dir = start_dir;

    visited.insert((x, y));

    loop {
        // Check forward
        let (dx, dy) = dir.forward_delta();
        let nx = x as isize + dx;
        let ny = y as isize + dy;
        if !in_bounds(nx, ny, h, w) {
            // leaving map
            break;
        }
        let nxu = nx as usize;
        let nyu = ny as usize;

        if map[nxu][nyu] == '#' {
            // turn right
            dir = dir.turn_right();
        } else {
            // move forward
            x = nxu;
            y = nyu;
            visited.insert((x, y));
        }
    }

    visited
}

/// Simulate with loop detection. If a loop occurs, returns true, else false.
/// Uses the given map (with the possible added obstacle).
fn simulate_with_loop_check(
    map: &Vec<Vec<char>>,
    start_pos: (usize, usize),
    start_dir: Dir,
) -> bool {
    let h = map.len();
    let w = map[0].len();
    let mut x = start_pos.0;
    let mut y = start_pos.1;
    let mut dir = start_dir;

    let mut seen_states = HashSet::new();
    seen_states.insert(State { x, y, dir });

    // Run simulation until out of bounds or detect a loop
    loop {
        let (dx, dy) = dir.forward_delta();
        let nx = x as isize + dx;
        let ny = y as isize + dy;

        if !in_bounds(nx, ny, h, w) {
            // leaves the map, no loop
            return false;
        }

        let nxu = nx as usize;
        let nyu = ny as usize;

        if map[nxu][nyu] == '#' {
            // turn right
            dir = dir.turn_right();
        } else {
            x = nxu;
            y = nyu;
        }

        let st = State { x, y, dir };
        if seen_states.contains(&st) {
            // loop detected
            return true;
        } else {
            seen_states.insert(st);
        }
    }
}

/// Part 1: Number of distinct positions visited before leaving the map.
pub fn p1gpt(lines: Vec<String>) -> usize {
    let (map, start_pos, start_dir) = parse_map(&lines);
    let visited = simulate(&map, start_pos, start_dir);
    visited.len()
}

/// Part 2: Number of positions where adding an obstacle creates a loop.
pub fn p2gpt(lines: Vec<String>) -> usize {
    let (mut map, start_pos, start_dir) = parse_map(&lines);
    let h = map.len();
    let w = map[0].len();

    // The guard's starting position cannot be chosen.
    let start_x = start_pos.0;
    let start_y = start_pos.1;

    let mut count = 0;

    // Try placing an obstacle in every '.' position except the start
    for i in 0..h {
        for j in 0..w {
            if i == start_x && j == start_y {
                continue;
            }
            if map[i][j] == '.' {
                // Place obstacle
                map[i][j] = '#';
                if simulate_with_loop_check(&map, start_pos, start_dir) {
                    count += 1;
                }
                // Remove obstacle
                map[i][j] = '.';
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let lines = vec![
            "....#.....".to_string(),
            ".........#".to_string(),
            "..........".to_string(),
            "..#.......".to_string(),
            ".......#..".to_string(),
            "..........".to_string(),
            ".#..^.....".to_string(),
            "........#.".to_string(),
            "#.........".to_string(),
            "......#...".to_string(),
        ];

        let part1 = p1gpt(lines.clone());
        assert_eq!(part1, 41);

        let part2 = p2gpt(lines);
        // According to the description, there are 6 possible positions.
        assert_eq!(part2, 6);
    }
}
