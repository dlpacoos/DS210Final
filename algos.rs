use std::collections::{VecDeque, HashMap, HashSet};
use super::graph::Graph;

// Function to find the degrees of separation for each node from the start node
pub fn bfs_degrees_of_separation(graph: &Graph, start: u32) -> HashMap<u32, u32> {
    let mut degrees = HashMap::new();
    let mut queue = VecDeque::new();

    degrees.insert(start, 0);
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        let degree = degrees[&node];

        for &neighbor in graph.neighbors(node).iter() {
            if !degrees.contains_key(&neighbor) {
                degrees.insert(neighbor, degree + 1);
                queue.push_back(neighbor);
            }
        }
    }

    degrees
}

pub fn find_node_with_most_followers(graph: &Graph) -> (u32, usize) {
    let mut max_degree = 0;
    let mut max_degree_node = 0;

    for (&node, neighbors) in &graph.edges {
        let degree = neighbors.len(); // Count of followers/connections
        if degree > max_degree {
            max_degree = degree;
            max_degree_node = node;
        }
    }

    (max_degree_node, max_degree)
}

pub fn calculate_total_unique_users(graph: &Graph) -> usize {
    // Create a HashSet to store unique users
    let mut unique_users = HashSet::new();

    // Iterate through the edges (connections) in the graph
    for (&user, neighbors) in &graph.edges {
        // Add the current user to the HashSet
        unique_users.insert(user);

        // Iterate through the user's neighbors and add them to the HashSet
        for &neighbor in neighbors {
            unique_users.insert(neighbor);
        }
    }

    // Return the count of unique users
    unique_users.len()
}
