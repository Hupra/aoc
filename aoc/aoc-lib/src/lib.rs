use rustlind_lib::download_file;
use std::env;
use std::fs::metadata;
use std::fs::File;
use std::io::Read;

pub fn init(year: i32, day: i32) -> Vec<String> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let dest = format!("/workspace/aoc/inputs/day-{year}-{day:02}");
    let session_cookie =
        env::var_os("AOC_SESSION_COOKIE").and_then(|cookie| cookie.to_str().map(|s| s.to_string()));

    session_cookie.clone().expect("AOC_SESSION_COOKIE is None");

    if metadata(&dest).is_err() && session_cookie.is_some() {
        let _ = download_file(&url, &dest, &session_cookie.unwrap());
    }

    let mut file = File::open(dest).expect("Failed to open file");
    let mut data = String::new();
    let _ = file.read_to_string(&mut data);

    data.lines().map(|line| line.to_string()).collect()
}

pub fn safe_get<T, I>(m: &[Vec<T>], i: I, j: I) -> Option<&T>
where
    I: TryInto<usize>,
{
    let i: usize = i.try_into().ok()?;
    let j: usize = j.try_into().ok()?;

    if i >= m.len() || j >= m[i].len() {
        return None;
    }

    Some(&m[i][j])
}

pub fn print_matrix<T: std::fmt::Display>(matrix: &[Vec<T>]) {
    for row in matrix {
        let row_str = row
            .iter()
            .map(|item| format!("{}", item))
            .collect::<Vec<_>>()
            .join(" ");
        println!("{}", row_str);
    }
}
