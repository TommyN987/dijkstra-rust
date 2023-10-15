use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::fmt::Display;

#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
}

#[derive(Debug)]
pub struct Edge {
    pub cost: u32,
    pub target_index: usize,
}

#[derive(Debug)]
pub struct Graph<T> {
    pub nodes: Vec<Node<T>>, // List of nodes: the position of each node in the vector is its unique identifier
    pub edges: Vec<Vec<Edge>>, // Adjacency list: for each node, a list of edges
}

// `Current` represents the current node and the cost to reach it.
pub struct Current<T> {
    pub cost: u32,
    pub node: T,
}

// Implement PartialEq, comparing only the `cost`.
impl<T> PartialEq for Current<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

// Implement Eq since PartialEq is implemented. We can leave it empty because
// Eq is a marker trait that doesn't require any methods.
impl<T> Eq for Current<T> {}

// Implement Ord, considering only the `cost`.
// We reverse the order of costs to turn the BinaryHeap into a min-heap.
impl<T> Ord for Current<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

// Implement PartialOrd. It must be consistent with Ord.
impl<T> PartialOrd for Current<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn dijkstra<T>(graph: &Graph<T>, start: usize, end: usize) -> Option<(VecDeque<usize>, u32)>
where
    T: Display,
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
