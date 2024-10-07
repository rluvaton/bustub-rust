use mut_binary_heap_index_keys::BinaryHeapIndexKeys;
use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    cost: usize,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Each node is represented as a `usize`, for a shorter implementation.
struct Edge {
    node: usize,
    cost: usize,
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node.
fn shortest_path(edges: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
    let mut heap: BinaryHeapIndexKeys<usize, Node> = BinaryHeapIndexKeys::new();
    heap.push(
        start,
        Node {
            cost: 0,
            position: start,
        },
    );

    while let Some(Node { cost, position }) = heap.pop() {
        if position == goal {
            return Some(cost);
        }

        for edge in &edges[position] {
            let next_cost = cost + edge.cost;

            // if the edge points to the node that is already in the heap, check
            // if it's cost is greater than the cost via this edge.
            // Note that normally dijkstra would also have a closed list with all
            // nodes that we have already visited. That closed list is also used to
            // keep track of the path we have taken.
            // To simplify this example we ignore that and only calculate the cost
            // to the goal.
            if heap.contains_key(&edge.node) {
                let mut node = heap.get_mut(&edge.node).unwrap();
                assert_eq!(node.position, edge.node);
                if next_cost < node.cost {
                    node.cost = next_cost;
                }
                // by dropping `node` the heap is autmatically updated.
            } else {
                heap.push(
                    edge.node,
                    Node {
                        cost: next_cost,
                        position: edge.node,
                    },
                );
            }
        }
    }
    // If the heap is empty, the goal wasn't found.
    None
}

fn main() {
    // This is the directed graph we're going to use.
    // The node numbers correspond to the different states,
    // and the edge weights symbolize the cost of moving
    // from one node to another.
    // Note that the edges are one-way.
    //
    //                  7
    //          +-----------------+
    //          |                 |
    //          v   1        2    |  2
    //          0 -----> 1 -----> 3 ---> 4
    //          |        ^        ^      ^
    //          |        | 1      |      |
    //          |        |        | 3    | 1
    //          +------> 2 -------+      |
    //           10      |               |
    //                   +---------------+
    //
    // The graph is represented as an adjacency list where each index,
    // corresponding to a node value, has a list of outgoing edges.
    // Chosen for its efficiency.
    let graph = vec![
        // Node 0
        vec![Edge { node: 2, cost: 10 }, Edge { node: 1, cost: 1 }],
        // Node 1
        vec![Edge { node: 3, cost: 2 }],
        // Node 2
        vec![
            Edge { node: 1, cost: 1 },
            Edge { node: 3, cost: 3 },
            Edge { node: 4, cost: 1 },
        ],
        // Node 3
        vec![Edge { node: 0, cost: 7 }, Edge { node: 4, cost: 2 }],
        // Node 4
        vec![],
    ];

    assert_eq!(shortest_path(&graph, 0, 1), Some(1));
    assert_eq!(shortest_path(&graph, 0, 3), Some(3));
    assert_eq!(shortest_path(&graph, 3, 0), Some(7));
    assert_eq!(shortest_path(&graph, 0, 4), Some(5));
    assert_eq!(shortest_path(&graph, 4, 0), None);
}
