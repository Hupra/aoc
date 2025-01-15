use regex::Regex;
use reqwest::blocking::Client;
use reqwest::header::{ ACCEPT, COOKIE, USER_AGENT };
use std::cmp::Ordering;
use std::collections::{ BinaryHeap, HashMap, HashSet };
use std::error::Error;
use std::fs::{ create_dir_all, File };
use std::io::Write;
use std::path::Path;
use std::sync::OnceLock;
use std::{ i32, usize };

pub mod utils;

pub fn download_file(url: &str, dest: &str, session_cookie: &str) -> Result<(), Box<dyn Error>> {
    let dest_dir = Path::new(dest).parent().unwrap();
    create_dir_all(dest_dir)?;

    let client = Client::new();
    let mut response = client
        .get(url)
        .header(USER_AGENT, "my_rust_app")
        .header(ACCEPT, "application/text")
        .header(COOKIE, format!("session={}", session_cookie))
        .send()?;

    if response.status().is_success() {
        let mut file = File::create(dest)?;
        let mut content = vec![];
        response.copy_to(&mut content)?;
        file.write_all(&content)?;

        println!("File downloaded successfully!");
    } else {
        eprintln!(
            "Failed to download file: {}. Response: {:?}",
            response.status(),
            response.text()?
        );
    }

    Ok(())
}

pub fn valid_positions<T, D>(ij: (usize, usize), m: &Vec<Vec<T>>, d: D) -> Vec<(usize, usize)>
    where D: IntoIterator<Item = (i32, i32)>
{
    valid_directions(ij, m, d)
        .into_iter()
        .map(|(di, dj)| (((ij.0 as i32) + di) as usize, ((ij.1 as i32) + dj) as usize))
        .collect()
}

pub fn valid_directions<T, D>(ij: (usize, usize), m: &Vec<Vec<T>>, d: D) -> Vec<(i32, i32)>
    where D: IntoIterator<Item = (i32, i32)>
{
    d.into_iter()
        .filter(|&(di, dj)| {
            let new_i = (ij.0 as i32) + di;
            let new_j = (ij.1 as i32) + dj;

            return (
                new_i >= 0 &&
                new_j >= 0 &&
                (new_i as usize) < m.len() &&
                (new_j as usize) < m[0].len()
            );
        })
        .collect()
}

#[derive(Eq, PartialEq)]
struct State<T> {
    cost: i32,
    node: T,
}

impl<T: Ord> Ord for State<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl<T: Ord> PartialOrd for State<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
pub struct Edge<T, C> {
    pub a: T,
    pub b: T,
    pub c: C,
}

#[derive(Clone)]
pub struct Graph<T, C> {
    pub adj: HashMap<T, Vec<Edge<T, C>>>,
}

impl<T> Graph<T, i32> where T: Eq + std::hash::Hash + Clone + Ord + std::fmt::Debug {
    pub fn new() -> Self {
        Graph {
            adj: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, a: T, b: T, c: i32) {
        let edge = Edge { a: a.clone(), b, c };
        self.adj.entry(a).or_default().push(edge);
    }

    pub fn add_edge_obj(&mut self, edge: Edge<T, i32>) {
        self.adj.entry(edge.a.clone()).or_default().push(edge);
    }

    pub fn nodes(&self) -> Vec<T> {
        self.adj
            .iter()
            .flat_map(|a_bs| {
                a_bs.1
                    .iter()
                    .flat_map(|e| [e.a.clone(), e.b.clone()])
                    .collect::<Vec<T>>()
            })
            .collect::<HashSet<T>>()
            .into_iter()
            .collect::<Vec<T>>()
    }

    pub fn speedify(&self) -> (Graph<usize, i32>, Vec<T>, HashMap<T, usize>) {
        let mut ng: Graph<usize, i32> = Graph::new();
        let mut node_t_to_i: HashMap<T, usize> = HashMap::new();
        let node_i_to_t: Vec<T> = self.nodes();

        for (i, node) in node_i_to_t.iter().enumerate() {
            node_t_to_i.insert(node.clone(), i);
        }

        for a_bs in &self.adj {
            for edge in a_bs.1 {
                let a = node_t_to_i.get(&edge.a).unwrap();
                let b = node_t_to_i.get(&edge.b).unwrap();
                ng.add_edge(*a, *b, edge.c);
            }
        }

        (ng, node_i_to_t, node_t_to_i)
    }

    fn shrink_edge(&self, old_edge: &Edge<T, i32>) -> Edge<T, i32> {
        let real_a = old_edge.a.clone();
        let mut old_a = old_edge.a.clone();
        let mut new_a = old_edge.b.clone();
        let mut new_c = old_edge.c;
        loop {
            if let Some(bs) = self.adj.get(&new_a) {
                let bsf = bs
                    .iter()
                    .filter(|&e| e.b != old_a)
                    .collect::<Vec<&Edge<T, i32>>>();
                if bsf.len() == 1 {
                    // use the next edge.
                    old_a = new_a;
                    new_a = bsf[0].b.clone();
                    new_c += bsf[0].c;
                } else {
                    // println!("{:?} {:?} {:?}", &real_a, &new_a, &new_c);
                    let e = Edge {
                        a: real_a,
                        b: new_a,
                        c: new_c,
                    };
                    return e;
                }
            }
        }
    }

    pub fn shrink(&self, s: T) -> Graph<T, i32> {
        let mut ng: Graph<T, i32> = Graph::new();

        // should shrink all edges that are "ourskirts"
        ng.add_edge_obj(self.shrink_edge(&self.adj.get(&s).unwrap()[0]));

        for a_bs in &self.adj {
            let bs = a_bs.1;
            if bs.len() < 3 {
                continue;
            }
            for b in bs {
                ng.add_edge_obj(self.shrink_edge(b));
            }
        }
        ng
    }

    pub fn dijkstra(&self, start: T) -> (HashMap<T, i32>, HashMap<T, Option<T>>) {
        let mut dist: HashMap<T, i32> = HashMap::new();
        let mut prev: HashMap<T, Option<T>> = HashMap::new();
        let mut heap: BinaryHeap<State<T>> = BinaryHeap::new();

        // tilføjer kun keys, skal loop vlaues i stedet <<<
        // Initialize the distance map with "infinity" and start with the source node
        // for node in self.adj.keys() {
        //     dist.insert(node.clone(), i32::MAX);
        //     prev.insert(node.clone(), None);
        // }

        dist.insert(start.clone(), 0);
        heap.push(State {
            cost: 0,
            node: start.clone(),
        });

        while let Some(State { cost, node }) = heap.pop() {
            if cost > *dist.entry(node.clone()).or_insert(i32::MAX) {
                continue;
            }
            if let Some(edges) = self.adj.get(&node) {
                for edge in edges {
                    let new_cost = cost + edge.c;
                    if new_cost < *dist.entry(edge.b.clone()).or_insert(i32::MAX) {
                        dist.insert(edge.b.clone(), new_cost);
                        prev.insert(edge.b.clone(), Some(edge.a.clone()));
                        heap.push(State {
                            cost: new_cost,
                            node: edge.b.clone(),
                        });
                    }
                }
            }
        }

        (dist, prev)
    }

    pub fn dijkstra_all(&self, start: T) -> (HashMap<T, i32>, HashMap<T, HashSet<T>>) {
        let mut dist: HashMap<T, i32> = HashMap::new();
        let mut prev: HashMap<T, HashSet<T>> = HashMap::new();
        let mut heap: BinaryHeap<State<T>> = BinaryHeap::new();

        dist.insert(start.clone(), 0);
        heap.push(State {
            cost: 0,
            node: start.clone(),
        });

        while let Some(State { cost, node }) = heap.pop() {
            if cost > *dist.entry(node.clone()).or_insert(i32::MAX) {
                continue;
            }
            if let Some(edges) = self.adj.get(&node) {
                for edge in edges {
                    let new_cost = cost + edge.c;
                    let cur_cost = *dist.get(&edge.b).unwrap_or(&i32::MAX);
                    if new_cost < cur_cost {
                        dist.insert(edge.b.clone(), new_cost);
                        prev.insert(edge.b.clone(), HashSet::from([edge.a.clone()]));
                        heap.push(State {
                            cost: new_cost,
                            node: edge.b.clone(),
                        });
                    }
                    if new_cost == cur_cost {
                        if let Some(hs) = prev.get_mut(&edge.b) {
                            hs.insert(edge.a.clone());
                        }
                    }
                }
            }
        }

        (dist, prev)
    }

    pub fn shortest_paths(&self, start: T, end: T) -> Option<(i32, Vec<Vec<T>>)> {
        let (dist, prev) = self.dijkstra_all(start.clone());

        // If the end node is not reachable, return None
        if !dist.contains_key(&end) {
            return None;
        }

        let mut all_paths = Vec::new();
        let mut current_path = vec![end.clone()];

        // Recursive helper function to build paths
        fn build_paths<T: Clone + Eq + std::hash::Hash + Ord>(
            prev: &HashMap<T, HashSet<T>>,
            current: &T,
            start: &T,
            current_path: &mut Vec<T>,
            all_paths: &mut Vec<Vec<T>>
        ) {
            if current == start {
                let mut path = current_path.clone();
                path.reverse();
                all_paths.push(path);
                return;
            }

            if let Some(parents) = prev.get(current) {
                for parent in parents {
                    current_path.push(parent.clone());
                    build_paths(prev, parent, start, current_path, all_paths);
                    current_path.pop();
                }
            }
        }

        build_paths(&prev, &end, &start, &mut current_path, &mut all_paths);

        Some((dist[&end], all_paths))
    }

    pub fn shortest_path(&self, start: T, end: T) -> (Option<i32>, Vec<T>) {
        let (dist, prev) = self.dijkstra(start.clone());
        let mut path = vec![];
        let mut current = end.clone();

        while let Some(prev_node) = prev.get(&current) {
            path.push(current.clone());
            if let Some(prev_node) = prev_node {
                current = prev_node.clone();
            } else {
                break;
            }
        }
        path.reverse();
        (dist.get(&end).and_then(|&x| Some(x)), path)
    }

    pub fn longest_path(&self, s: T, t: T) -> (i32, Vec<T>) {
        let speed = self.shrink(s.clone()).speedify();

        let ng = speed.0;
        let i_t = speed.1;
        let t_i = speed.2;

        let ns = *t_i.get(&s).unwrap();
        let nt = *t_i.get(&t).unwrap();

        let res = longest_path_rec(ns, Vec::new(), &ng, nt);

        let path = res.1
            .into_iter()
            .map(|i| i_t[i].clone())
            .collect();
        return (res.0, path);

        fn longest_path_rec(
            a: usize,
            mut v: Vec<usize>,
            g: &Graph<usize, i32>,
            t: usize
        ) -> (i32, Vec<usize>) {
            if a == t {
                v.push(a);
                return (0, v);
            }

            v.push(a);
            let mut best = i32::MIN;
            let mut best_path = vec![];

            if let Some(adj) = g.adj.get(&a) {
                for e in adj {
                    if !v.contains(&e.b) {
                        // Clone the path for the recursive call
                        let (cost, path) = longest_path_rec(e.b, v.clone(), g, t);
                        let total_cost = e.c + cost;
                        if total_cost > best {
                            best = total_cost;
                            best_path = path;
                        }
                    }
                }
            }

            // If no path is found, return an empty path and minimum cost
            if best == i32::MIN {
                return (best, vec![]);
            }

            // Prepend the current node to the best path
            let mut full_path = vec![a];
            full_path.extend(best_path);

            (best, full_path)
        }
    }
}

impl<T, C> Graph<T, C> {
    pub fn map_graph<U, F>(self, mapper: F) -> Graph<U, C>
        where F: Fn(T) -> U, U: std::cmp::Eq + std::hash::Hash
    {
        let mut new_adj = HashMap::new();

        for (key, edges) in self.adj {
            let new_key = mapper(key);
            let new_edges: Vec<Edge<U, C>> = edges
                .into_iter()
                .map(|edge| Edge {
                    a: mapper(edge.a),
                    b: mapper(edge.b),
                    c: edge.c,
                })
                .collect();

            new_adj.insert(new_key, new_edges);
        }

        Graph { adj: new_adj }
    }
}

// for undirected graph
impl<T, C> Graph<T, C> where T: Eq + std::hash::Hash + Clone + Ord + std::fmt::Debug {
    pub fn find_triangles(&self) -> Vec<(T, T, T)> {
        let mut visited: HashSet<T> = HashSet::new();
        let mut trianlges: Vec<(T, T, T)> = Vec::new();

        for (a, a_edges) in &self.adj {
            visited.insert(a.clone());
            for i in 0..a_edges.len() {
                let a_b = &a_edges[i];
                if visited.contains(&a_b.b) {
                    continue;
                }
                for j in i + 1..a_edges.len() {
                    let a_c = &a_edges[j];
                    if visited.contains(&a_c.b) {
                        continue;
                    }
                    if let Some(b_edges) = self.adj.get(&a_b.b) {
                        for b_c in b_edges {
                            if b_c.b == a_c.b {
                                trianlges.push((a.clone(), a_b.b.clone(), a_c.b.clone()));
                            }
                        }
                    }
                }
            }
        }
        trianlges
    }
}

pub fn poly_area<T>(points: Vec<(T, T)>) -> f64 where T: Into<f64> + Copy {
    let mut area = 0.0;
    let n = points.len();

    for i in 0..n {
        let j = (i + 1) % n;

        let x1: f64 = points[i].0.into();
        let x2: f64 = points[j].0.into();
        let y1: f64 = points[i].1.into();
        let y2: f64 = points[j].1.into();

        area += x1 * y2 - x2 * y1;
    }

    area.abs() / 2.0
}

// Parsing stuff
static RE_NUMS: OnceLock<Regex> = OnceLock::new();
pub fn re_nums(input: &str) -> Vec<i32> {
    let re = RE_NUMS.get_or_init(|| Regex::new(r"\d+").unwrap());
    re.find_iter(input)
        .filter_map(|mat| mat.as_str().parse::<i32>().ok())
        .collect()
}
pub fn re_nums_usize(input: &str) -> Vec<usize> {
    let re = Regex::new(r"\d+").unwrap();
    re.find_iter(input)
        .filter_map(|mat| mat.as_str().parse::<usize>().ok())
        .collect()
}

pub fn fnums(list: Vec<String>) -> Vec<i32> {
    list.iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

pub fn fsplit(line: &str, split_on: &str) -> Vec<String> {
    line.split(split_on)
        .map(|s| s.to_string())
        .collect()
}

pub fn re_nums_neg(input: &str) -> Vec<i32> {
    let re = Regex::new(r"-?\d+").unwrap();
    re.find_iter(input)
        .filter_map(|mat| mat.as_str().parse::<i32>().ok())
        .collect()
}

// Matrix magic
pub fn get_or<T: Copy>(ij: (i32, i32), m: &Vec<Vec<T>>, or: T) -> T {
    if ij.0 < 0 || ij.1 < 0 || ij.0 >= (m.len() as i32) || ij.1 >= (m[ij.0 as usize].len() as i32) {
        return or;
    }
    return m[ij.0 as usize][ij.1 as usize];
}

pub fn m_swap(a: (i32, i32), b: (i32, i32), m: &mut Vec<Vec<char>>) {
    let a_i = a.0 as usize;
    let a_j = a.1 as usize;
    let b_i = b.0 as usize;
    let b_j = b.1 as usize;

    let tmp = m[a_i][a_j];
    m[a_i][a_j] = m[b_i][b_j];
    m[b_i][b_j] = tmp;
}
// Tuple helpers
#[macro_export]
macro_rules! tadd {
    ($tuple1:expr, $tuple2:expr) => {
        ($tuple1.0 + $tuple2.0, $tuple1.1 + $tuple2.1)
    };
}

// macro_rules! tmul {
//     ($t:expr, $mul:expr) => {
//         ($t.0 * $mul, $t.1 * $mul)
//     };
// }
