use aoc_lib::init;
use cached::proc_macro::cached;
use itertools::Itertools;
use petgraph::{ graph::UnGraph, visit::NodeRef };
use rustlind_lib::*;
use std::collections::HashSet;
use std::{ collections::HashMap, time::Instant };
use petgraph::graph::{ Graph, NodeIndex };
use petgraph::Undirected;

fn p1(lines: Vec<String>) -> Option<usize> {
    let mut graph: rustlind_lib::Graph<String, i32> = rustlind_lib::Graph::new();
    for line in lines {
        let parts: Vec<&str> = line.split('-').collect();
        graph.add_edge(parts[0].to_string(), parts[1].to_string(), 1);
        graph.add_edge(parts[1].to_string(), parts[0].to_string(), 1);
    }
    let triangles = graph.find_triangles();

    let triangle_sum = triangles
        .iter()
        .map(|(a, b, c)| (a.starts_with('t') || b.starts_with('t') || c.starts_with('t')) as usize)
        .sum();

    Some(triangle_sum)
}

fn p2(lines: Vec<String>) -> Option<String> {
    let edges: Vec<(String, String)> = lines
        .into_iter()
        .map(|line| {
            let parts: Vec<&str> = line.split('-').collect();
            (parts[0].to_string(), parts[1].to_string())
        })
        .collect();

    let mut g = UnGraph::<String, ()>::new_undirected();
    let mut indices = HashMap::new();

    for (label_a, label_b) in edges {
        let a = *indices.entry(label_a.clone()).or_insert_with(|| g.add_node(label_a));
        let b = *indices.entry(label_b.clone()).or_insert_with(|| g.add_node(label_b));
        g.add_edge(a, b, ());
    }

    let max_clique = find_max_clique(&g).and_then(|hs| {
        let mut names = hs
            .iter()
            .map(|&node| g[node].clone())
            .collect::<Vec<String>>();
        names.sort();
        Some(names.join(","))
    });

    max_clique
}

//"as,bu,cp,dj,ez,fd,hu,it,kj,nx,pp,xh,yu"
//"as,bu,cp,dj,ez,fd,hu,it,kj,nx,pp,xh,yu"
fn main() {
    let lines = init(2024, 23);
    let timer = Instant::now();
    println!("{:?} | time: {:?}", p1(lines.clone()), timer.elapsed());
    println!("{:?} | time: {:?}", p2(lines.clone()), timer.elapsed());
}

// r -> current clique
// p -> candidate set
// x -> ex clusion sey
fn bron_kerbosch_pivot(
    graph: &Graph<String, (), Undirected>,
    r: &mut HashSet<NodeIndex>,
    p: &mut HashSet<NodeIndex>,
    x: &mut HashSet<NodeIndex>,
    cliques: &mut Vec<HashSet<NodeIndex>>
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r.clone());
        return;
    }

    // Choose a pivot from P ∪ X
    let pivot = p.union(x).next().cloned();
    let pivot = match pivot {
        Some(u) => u,
        None => {
            return;
        }
    };

    // P \ N(pivot)
    let candidates: Vec<_> = p.difference(&graph.neighbors(pivot).collect()).cloned().collect();

    for v in candidates {
        r.insert(v);
        let neighbors_v: HashSet<_> = graph.neighbors(v).collect();

        let mut p_new = p.intersection(&neighbors_v).cloned().collect();
        let mut x_new = x.intersection(&neighbors_v).cloned().collect();

        bron_kerbosch_pivot(graph, r, &mut p_new, &mut x_new, cliques);

        r.remove(&v);
        p.remove(&v);
        x.insert(v);
    }
}

/// Finds the maximum clique in the given undirected graph.
/// Returns `None` if the graph has no cliques (i.e., no nodes).
fn find_max_clique(graph: &Graph<String, (), Undirected>) -> Option<HashSet<NodeIndex>> {
    let mut cliques: Vec<HashSet<NodeIndex>> = Vec::new();
    let mut r: HashSet<NodeIndex> = HashSet::new();
    let mut p: HashSet<NodeIndex> = graph.node_indices().collect();
    let mut x: HashSet<NodeIndex> = HashSet::new();

    bron_kerbosch_pivot(graph, &mut r, &mut p, &mut x, &mut cliques);

    cliques.into_iter().max_by_key(|c| c.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test_lines() -> Vec<String> {
        include_str!("test.txt").lines().map(String::from).collect()
    }

    #[test]
    fn test_p1() {
        let expected = Some(7);
        let actual = p1(test_lines());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_p2() {
        let expected = Some("co,de,ka,ta".to_string());
        let actual = p2(include_str!("test2.txt").lines().map(String::from).collect());
        assert_eq!(actual, expected);
    }
}
