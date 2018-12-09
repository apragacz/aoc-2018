use std::io;
use std::io::BufRead;
use std::collections::BTreeSet;
use std::collections::HashMap;

#[derive(Debug)]
struct GraphNode {
    id: char,
    edges_to: BTreeSet<char>,
    num_edges_from: usize,
}

fn create_graph_node(id: char) -> GraphNode {
    return GraphNode {id, edges_to: BTreeSet::new(), num_edges_from: 0}
}

fn main() {
    let mut dependencies: Vec<(char, char)> = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        let segments: Vec<&str> = l.trim().split(' ').collect();
        match *segments.as_slice() {
            [_, a_str, _, _, _, _, _, b_str, _, _] => {
                let a = a_str.chars().next().unwrap();
                let b = b_str.chars().next().unwrap();
                dependencies.push((a, b));
            },
            _ => panic!("invalid line")
        }
    }

    let mut graph: HashMap<char, GraphNode> = HashMap::new();

    for (a, b) in &dependencies {
        graph.entry(*a).or_insert(create_graph_node(*a));
        graph.entry(*b).or_insert(create_graph_node(*b));
    }

    for (a, b) in &dependencies {
        {
            let a_node: &mut GraphNode = graph.get_mut(a).unwrap();
            a_node.edges_to.insert(*b);
        }
        {
            let b_node: &mut GraphNode = graph.get_mut(b).unwrap();
            b_node.num_edges_from += 1;
        }
    }

    let mut start_node_ids: BTreeSet<char> = graph.values().filter(|n| n.num_edges_from == 0).map(|n| n.id).collect();
    let mut order = String::new();

    while !start_node_ids.is_empty() {
        let node_id: char = start_node_ids.iter().next().cloned().unwrap();
        start_node_ids.remove(&node_id);


        let node_edges: Vec<_> = graph.get(&node_id).unwrap().edges_to.iter().cloned().collect();

        for next_node_id in &node_edges {
            {
                let node: &mut GraphNode = graph.get_mut(&node_id).unwrap();
                node.edges_to.remove(next_node_id);
            }
            {
                let next_node: &mut GraphNode = graph.get_mut(&next_node_id).unwrap();
                next_node.num_edges_from -= 1;
                if next_node.num_edges_from == 0 {
                    start_node_ids.insert(*next_node_id);
                }
            }
        }

        order.push(node_id);
    }
    println!("{}", order);
}
