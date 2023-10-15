use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::fmt::Display;
use std::hash::Hash;

#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Node<T> {
    value: T,
}

#[derive(Debug)]
struct Edge {
    cost: u32,
    target_index: usize,
}

#[derive(Debug)]
struct Graph<T> {
    nodes: Vec<Node<T>>, // List of nodes: the position of each node in the vector is its unique identifier
    edges: Vec<Vec<Edge>>, // Adjacency list: for each node, a list of edges
}

// `Current` represents the current node and the cost to reach it.
#[derive(Debug, Eq, PartialEq)]
struct Current<T> {
    cost: u32,
    node: T,
}

// Implementing `Ord` to make `Current` sortable in the `BinaryHeap`.
// We want a min-heap, so we reverse the order of costs.
impl<T> Ord for Current<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl<T> PartialOrd for Current<T>
where
    T: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra<T>(graph: &Graph<T>, start: usize, end: usize) -> Option<(VecDeque<usize>, u32)>
where
    T: Hash + Eq + Ord + Display,
{
    let mut dist: HashMap<usize, u32> = HashMap::new();
    let mut heap: BinaryHeap<Current<usize>> = BinaryHeap::new();
    let mut prev: HashMap<usize, usize> = HashMap::new();

    dist.insert(start, 0);
    heap.push(Current {
        cost: 0,
        node: start,
    });

    while let Some(Current { cost, node }) = heap.pop() {
        if node == end {
            let path = reconstruct_path(&prev, start, end);
            return Some((path, cost));
        }

        // Proceed with the current node only if we haven't processed a shorter path before.
        if cost > *dist.entry(node).or_insert(u32::MAX) {
            continue;
        }

        // Visit each neighbor of the current node.
        for edge in &graph.edges[node] {
            let next = Current {
                cost: cost + edge.cost,
                node: edge.target_index,
            };

            // If we found a shorter path to the neighbor, remember it.
            if next.cost < *dist.entry(next.node).or_insert(u32::MAX) {
                dist.insert(next.node, next.cost);
                prev.insert(next.node, node);
                heap.push(next);
            }
        }
    }

    None // No path found
}

fn reconstruct_path(prev: &HashMap<usize, usize>, start: usize, end: usize) -> VecDeque<usize> {
    let mut path = VecDeque::new();
    let mut current = end;

    while current != start {
        path.push_front(current);
        current = *prev.get(&current).expect("Path construction error");
    }

    path.push_front(start);
    path
}

fn main() {
    // Create a few nodes
    let node_a = Node { value: "A" };
    let node_b = Node { value: "B" };
    let node_c = Node { value: "C" };
    let node_d = Node { value: "D" };

    // Create a graph
    let graph = Graph {
        nodes: vec![node_a, node_b, node_c, node_d],
        edges: vec![
            // Edges for node A
            vec![
                Edge {
                    cost: 1,
                    target_index: 1,
                },
                Edge {
                    cost: 4,
                    target_index: 2,
                },
            ],
            // Edges for node B
            vec![Edge {
                cost: 3,
                target_index: 3,
            }],
            // Edges for node C
            vec![Edge {
                cost: 2,
                target_index: 3,
            }],
            // Edges for node D (no outgoing edges)
            vec![],
        ],
    };

    // Define start and end nodes
    let start_node = 0; // Node "A"
    let end_node = 3; // Node "D"

    match dijkstra(&graph, start_node, end_node) {
        Some((path, cost)) => {
            // If a path was found, print it and the cost
            println!("Path found:");
            for index in path {
                print!("{} ", graph.nodes[index].value);
            }
            println!("\nTotal cost: {}", cost);
        }
        None => {
            // If no path was found, indicate this
            println!(
                "No path found from {} to {}.",
                graph.nodes[start_node].value, graph.nodes[end_node].value
            );
        }
    }
}
