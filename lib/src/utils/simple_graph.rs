use std::{
    collections::{ HashMap, VecDeque },
    fmt::{ self, Display, Write as FmtWrite },
    fs::File,
    hash::Hash,
    io::Write,
    process::{ Command, Stdio },
};
// L = Label
#[derive(Debug)]
pub struct Graph<L> {
    pub adj: Vec<Vec<usize>>,
    pub labels: HashMap<L, usize>,
    pub indices: Vec<L>,
}

impl<L> FromIterator<(L, L)> for Graph<L> where L: Hash + Eq + Clone {
    fn from_iter<I: IntoIterator<Item = (L, L)>>(iter: I) -> Self {
        let mut graph = Graph::new();
        for (a, b) in iter {
            graph.add_edge(a, b);
        }
        graph
    }
}

impl<L> Graph<L> where L: Hash + Eq + Clone {
    pub fn new() -> Self {
        Graph {
            adj: Vec::new(),
            labels: HashMap::new(),
            indices: Vec::new(),
        }
    }
    pub fn clear(&mut self) {
        self.adj.clear();
        self.indices.clear();
        self.labels.clear();
    }
    pub fn get_node_id(&mut self, label: L) -> usize {
        match self.labels.get(&label) {
            Some(&id) => id,
            None => {
                let id = self.indices.len();
                self.adj.push(Vec::new());
                self.labels.insert(label.clone(), id);
                self.indices.push(label);
                id
            }
        }
    }
    pub fn add_edge(&mut self, label_a: L, label_b: L) -> (usize, usize) {
        let a = self.get_node_id(label_a);
        let b = self.get_node_id(label_b);
        self.adj[a].push(b);
        (a, b)
    }

    pub fn indices_to_labels(&mut self, list: &Vec<usize>) -> Vec<L> {
        list.into_iter()
            .map(|&id| self.indices[id].clone())
            .collect()
    }

    pub fn topological_sort(&mut self) -> Result<Vec<L>, String> {
        topological_sort(&self.adj).and_then(|list| Ok(self.indices_to_labels(&list)))
    }
}

fn topological_sort(adj: &Vec<Vec<usize>>) -> Result<Vec<usize>, String> {
    let num_vertices = adj.len();
    let mut in_degree = vec![0; num_vertices];

    // Compute in-degree of each vertex
    for u in 0..num_vertices {
        for &v in &adj[u] {
            if v >= num_vertices {
                return Err(format!("Invalid vertex index: {}", v));
            }
            in_degree[v] += 1;
        }
    }

    // Initialize queue with all vertices having in-degree 0
    let mut queue = VecDeque::new();
    for u in 0..num_vertices {
        if in_degree[u] == 0 {
            queue.push_back(u);
        }
    }

    let mut sorted_order = Vec::with_capacity(num_vertices);

    while let Some(u) = queue.pop_front() {
        sorted_order.push(u);

        for &v in &adj[u] {
            in_degree[v] -= 1;
            if in_degree[v] == 0 {
                queue.push_back(v);
            }
        }
    }

    if sorted_order.len() == num_vertices {
        Ok(sorted_order)
    } else {
        // There's a cycle in the graph
        Err("Graph contains a cycle; topological sort not possible.".to_string())
    }
}

impl<L> Graph<L> where L: Hash + Eq + Clone + Display {
    /// Generates a DOT representation of the graph.
    pub fn to_dot(&self) -> Result<String, fmt::Error> {
        let mut dot = String::new();
        writeln!(dot, "digraph G {{")?;
        // Optional: Define graph attributes (e.g., rank direction)
        writeln!(dot, "    rankdir=LR;")?;
        // Define nodes
        for (id, label) in self.indices.iter().enumerate() {
            writeln!(dot, "    {} [label=\"{}\"];", id, label)?;
        }
        // Define edges
        for (from_id, neighbors) in self.adj.iter().enumerate() {
            let from_starts_with_xy = self.indices[from_id]
                .to_string()
                .chars()
                .next()
                .map(|c| matches!(c.to_ascii_lowercase(), 'x' | 'y'))
                .unwrap_or(false);

            let from_xor = self.indices[from_id].to_string().contains("XOR");

            for &to_id in neighbors {
                let to_starts_with_z = self.indices[to_id]
                    .to_string()
                    .chars()
                    .next()
                    .map(|c| c.to_ascii_lowercase() == 'z')
                    .unwrap_or(false);

                if to_starts_with_z && from_xor == false {
                    writeln!(dot, "    {} -> {} [color=red, penwidth=3.0];", from_id, to_id)?;
                } else if from_starts_with_xy && to_starts_with_z {
                    writeln!(dot, "    {} -> {} [color=blue, penwidth=2.0];", from_id, to_id)?;
                } else {
                    // Default edge style
                    writeln!(dot, "    {} -> {};", from_id, to_id)?;
                }
            }
        }
        writeln!(dot, "}}")?;
        Ok(dot)
    }
}

// impl<L> Graph<L> where L: Hash + Eq + Clone + Display {
//     /// Generates a DOT representation of the graph.
//     pub fn to_dot(&self) -> Result<String, fmt::Error> {
//         let mut dot = String::new();
//         writeln!(dot, "digraph G {{")?;
//         // Optional: Define graph attributes (e.g., rank direction)
//         writeln!(dot, "    rankdir=LR;")?;
//         // Define nodes
//         for (id, label) in self.indices.iter().enumerate() {
//             writeln!(dot, "    {} [label=\"{}\"];", id, label)?;
//         }
//         // Define edges
//         for (from_id, neighbors) in self.adj.iter().enumerate() {
//             for &to_id in neighbors {

//                 writeln!(dot, "    {} -> {};", from_id, to_id)?;
//             }
//         }
//         writeln!(dot, "}}")?;
//         Ok(dot)
//     }
// }

impl<L> Graph<L> where L: Hash + Eq + Clone + Display {
    /// Generates a DOT representation of the graph.
    pub fn to_png(&self, path: &str) -> Result<String, fmt::Error> {
        let dot = self.to_dot()?;

        // Spawn the `dot` process
        let mut child = Command::new("dot")
            .args(&["-Tpng"]) // Specify PNG output
            .stdin(Stdio::piped()) // Enable writing to stdin
            .stdout(Stdio::piped()) // Enable reading from stdout
            .spawn()
            .unwrap();

        // Write DOT content to the `dot` process's stdin
        {
            let stdin = child.stdin.as_mut().ok_or("Failed to open stdin").unwrap();
            stdin.write_all(dot.as_bytes()).unwrap();
        }

        // Capture the output from the `dot` process
        let output = child.wait_with_output().unwrap();

        // Check if the command was successful
        if output.status.success() {
            // Save the PNG data to a file
            if let Ok(mut file) = File::create(path) {
                let _ = file.write_all(&output.stdout);
                println!("PNG image saved as output.png");
            } else {
                let error = String::from_utf8_lossy(&output.stderr);
                eprintln!("Error generating PNG: {}", error);
            }
        } else {
            // Print the error message from `dot`
            let error = String::from_utf8_lossy(&output.stderr);
            eprintln!("Error generating PNG: {}", error);
        }

        Ok("yes".to_string())
    }
}
