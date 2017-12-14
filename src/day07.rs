use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

pub fn run() {
    let test_graph = create_graph(vec![
                 "pbga (66)".to_string(),
                 "xhth (57)".to_string(),
                 "ebii (61)".to_string(),
                 "havc (66)".to_string(),
                 "ktlj (57)".to_string(),
                 "fwft (72) -> ktlj, cntj, xhth".to_string(),
                 "qoyq (66)".to_string(),
                 "padx (45) -> pbga, havc, qoyq".to_string(),
                 "tknk (41) -> ugml, padx, fwft".to_string(),
                 "jptl (61)".to_string(),
                 "ugml (68) -> gyxo, ebii, jptl".to_string(),
                 "gyxo (61)".to_string(),
                 "cntj (57)".to_string()
    ]);
    assert!(solve1(&test_graph) == "tknk".to_string());

    // Get the input
    let stdin = io::stdin();
    let input: Vec<String> = stdin.lock().lines().filter_map(|l| l.ok()).collect();

    let graph = create_graph(input);
    println!("Day 7-1: {}", solve1(&graph));
}

fn solve1(graph: &Graph) -> String {
    let mut has_parent: HashSet<NodeIndex> = HashSet::new();
    for node in 0..graph.nodes.len() {
        for child in graph.successors(node) {
            has_parent.insert(child);
        }
    }
    let all_indices = HashSet::from_iter(0..graph.nodes.len());
    let roots = all_indices.difference(&has_parent);
    let roots: Vec<&NodeIndex> = roots.collect();

    graph.nodes[*roots[0]].name.clone()
}

fn create_graph(lines: Vec<String>) -> Graph {
    let mut graph = Graph{ nodes: vec![], edges: vec![] };
    let mut names: HashMap<&str, NodeIndex> = HashMap::new();

    for line in lines.iter() {
        let mut split = line.split("->");
        let node = split.next();
        let children = split.next();

        let mut split = node.unwrap().split_whitespace();
        let name = split.next().unwrap();
        let weight: u32 = split.next().unwrap().chars()
            .filter(|c| c.is_digit(10)).collect::<String>().parse().unwrap();

        let children: Vec<&str> = match children {
            Some(x) => x.split_whitespace()
                .map(|name| name.trim_matches(','))
                .collect(),
            None => vec![]
        };

        let parent_index: NodeIndex;
        if !names.contains_key(&name) {
            parent_index = graph.add_node(name.to_string(), Some(weight));
            names.insert(name.clone(), parent_index);
        } else {
            parent_index = names.get(&name).unwrap().clone();
            match graph.nodes[parent_index].weight {
                Some(_) => (),
                None => graph.nodes[parent_index].weight = Some(weight)
            };
        }

        for child in children {
            let child_index: NodeIndex;
            if !names.contains_key(&child) {
                child_index = graph.add_node(child.to_string(), None);
                names.insert(child.clone(), child_index);
            } else {
                child_index = names.get(&child).unwrap().clone();
            }
            graph.add_edge(parent_index, child_index);
        }
    }
    graph
}

struct Graph {
    nodes: Vec<NodeData>,
    edges: Vec<EdgeData>
}

type NodeIndex = usize;

struct NodeData {
    first_outgoing_edge: Option<EdgeIndex>,
    name: String,
    weight: Option<u32>
}

type EdgeIndex = usize;

struct EdgeData {
    target: NodeIndex,
    next_outgoing_edge: Option<EdgeIndex>
}

impl Graph {
    fn add_node(&mut self, name: String, weight: Option<u32>) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(NodeData {
            first_outgoing_edge: None,
            name: name,
            weight: weight
       });
        index
    }
}

impl Graph {
    fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) {
        let edge_index = self.edges.len();
        let node_data = &mut self.nodes[source];
        self.edges.push(EdgeData {
            target: target,
            next_outgoing_edge: node_data.first_outgoing_edge
        });
        node_data.first_outgoing_edge = Some(edge_index);
    }
}

impl Graph {
    fn successors(&self, source: NodeIndex) -> Successors {
        let first_outgoing_edge = self.nodes[source].first_outgoing_edge;
        Successors { graph: self, current_edge_index: first_outgoing_edge }
    }
}

struct Successors<'graph> {
    graph: &'graph Graph,
    current_edge_index: Option<EdgeIndex>
}

impl<'graph> Iterator for Successors<'graph> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<NodeIndex> {
        match self.current_edge_index {
            None => None,
            Some(edge_num) => {
                let edge = &self.graph.edges[edge_num];
                self.current_edge_index = edge.next_outgoing_edge;
                Some(edge.target)
            }
        }
    }
}

