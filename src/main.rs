mod dijkstra;

use dijkstra::{dijkstra, Edge, Graph, Node};

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
