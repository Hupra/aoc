use aoc_lib::init;
use std::{ cell::RefCell, collections::{ HashMap, HashSet }, rc::{ Rc, Weak }, time::Instant };

#[derive(Debug)]
struct Dir {
    name: String,
    parent: Option<Weak<RefCell<Dir>>>,
    sub_dirs: HashMap<String, Rc<RefCell<Dir>>>,
    files: HashSet<(String, usize)>,
}

impl Dir {
    fn new(name: String) -> Rc<RefCell<Dir>> {
        Rc::new(
            RefCell::new(Dir {
                name,
                parent: None,
                sub_dirs: HashMap::new(),
                files: HashSet::new(),
            })
        )
    }

    fn add_file(&mut self, filename: String, size: usize) {
        self.files.insert((filename, size));
    }

    fn add_sub_dir(parent: &Rc<RefCell<Dir>>, name: String) -> Rc<RefCell<Dir>> {
        let new_dir = Rc::new(
            RefCell::new(Dir {
                name: name.clone(),
                parent: Some(Rc::downgrade(parent)),
                sub_dirs: HashMap::new(),
                files: HashSet::new(),
            })
        );
        parent.borrow_mut().sub_dirs.insert(name, Rc::clone(&new_dir));
        new_dir
    }

    fn get_parent(&self) -> Option<Rc<RefCell<Dir>>> {
        self.parent.as_ref().and_then(|weak_parent| weak_parent.upgrade())
    }

    fn total_size(&self) -> usize {
        let sum_files = self.files
            .iter()
            .map(|(_, size)| size)
            .sum::<usize>();
        let sum_subdirs = self.sub_dirs
            .values()
            .map(|sub_dir_rc| sub_dir_rc.borrow().total_size())
            .sum::<usize>();

        sum_files + sum_subdirs
    }

    fn collect_all_sizes(&self) -> Vec<(String, usize)> {
        let mut result = Vec::new();

        let this_size = self.total_size();
        result.push((self.name.clone(), this_size));

        for sub_dir_rc in self.sub_dirs.values() {
            let sub_dir = sub_dir_rc.borrow();
            result.extend(sub_dir.collect_all_sizes());
        }

        result
    }
}

fn build_dir(lines: Vec<String>) -> Rc<RefCell<Dir>> {
    let root = Dir::new("/".to_string());
    let mut dir = Rc::clone(&root);

    for line in lines.iter().skip(1) {
        match line {
            _ if line == "$ ls" => {
                continue;
            }
            _ if line.starts_with("dir") => {
                let dir_name = line.split_whitespace().nth(1).unwrap();
                Dir::add_sub_dir(&dir, dir_name.to_string());
            }
            _ if line.starts_with("$ cd ..") => {
                let parent = dir.borrow().get_parent().unwrap();
                dir = Rc::clone(&parent);
            }
            _ if line.starts_with("$ cd") => {
                let dir_name = &line[5..];
                let sub_dirs = dir.borrow().sub_dirs.clone();
                let sub_dir = sub_dirs.values().find(|&d| d.borrow().name == dir_name);
                if let Some(sub_dir) = sub_dir {
                    dir = Rc::clone(sub_dir);
                }
            }
            _ => {
                let mut parts = line.split_whitespace();
                let size = parts.next().unwrap().parse::<usize>().unwrap();
                let filename = parts.next().unwrap();
                dir.borrow_mut().add_file(filename.to_string(), size);
            }
        }
    }
    root
}

fn p1(lines: Vec<String>) -> Option<usize> {
    let root = build_dir(lines);

    let x = root
        .borrow()
        .collect_all_sizes()
        .into_iter()
        .map(|(_dirname, size)| size)
        .filter(|&size| size < 100_000)
        .map(|size| Some(size))
        .sum();
    x
}

fn p2(lines: Vec<String>) -> Option<usize> {
    let root = build_dir(lines);

    let total: usize = 70_000_000;
    let needed_unused: usize = 30_000_000;
    let min_delete: usize = root.borrow().total_size() - (total - needed_unused);

    let x = root
        .borrow()
        .collect_all_sizes()
        .into_iter()
        .map(|(_name, size)| size)
        .filter(|size| *size >= min_delete)
        .min();
    x
}

fn main() {
    let lines = init(2022, 7);
    let timer = Instant::now();
    println!("{:?} | time: {:?}", p1(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p2(lines.clone()), timer.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let expected = Some(95437);
        let actual = p1(include_str!("test.txt").lines().map(String::from).collect());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_p2() {
        let expected = Some(24933642);
        let actual = p2(include_str!("test.txt").lines().map(String::from).collect());
        assert_eq!(actual, expected);
    }
}
