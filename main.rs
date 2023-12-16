mod graph;
mod algos;

use graph::Graph;
use std::path::Path;
use rand::{seq::SliceRandom, thread_rng};


fn main() -> std::io::Result<()> {
    // Paths for different country network data files
    let file_paths = vec![
        "C:\\Users\\psoon\\DS210final\\deezer_croatia.txt",
        "C:\\Users\\psoon\\DS210final\\deezer_hungary.txt",
        "C:\\Users\\psoon\\DS210final\\musae_git_edges.txt",
    ];

    for path in file_paths {
        let graph = Graph::from_file(Path::new(&path))?;

        let mut rng = thread_rng();

        // how many starting nodes I want to use, these are then randomly selected 
        let num_starting_nodes = 200;  
        let all_nodes: Vec<u32> = graph.edges.keys().cloned().collect();

        // Randomly select starting nodes
        let starting_nodes: Vec<u32> = all_nodes.choose_multiple(&mut rng, num_starting_nodes).cloned().collect();

        let mut within_six_degrees = 0;
        let mut total_pairs = 0;

        for &start_node in &starting_nodes {
            let degrees = algos::bfs_degrees_of_separation(&graph, start_node);

            for &degree in degrees.values() {
                if degree <= 6 {
                    within_six_degrees += 1;
                }
                total_pairs += 1;
            }
        }

        let percentage_within_six_degrees = (within_six_degrees as f64 / total_pairs as f64) * 100.0;
        println!("Percentage of node pairs within six degrees of separation in {}: {:.2}%", path, percentage_within_six_degrees);

        let (node, followers) = algos::find_node_with_most_followers(&graph);
        println!("Most popular User: {} ({} followers)", node, followers);

        let total_unique_users = algos::calculate_total_unique_users(&graph);
        println!("Total unique users : {}", total_unique_users);

    }

    Ok(())
}

#[cfg(test)]

mod tests {
    #[allow(unused_imports)]
    use super::algos::{find_node_with_most_followers, bfs_degrees_of_separation};
    use crate::graph::Graph;


    #[test]
    fn test_bfs_degrees_of_separation() {
        // Creating a test graph
        let mut test_graph = Graph::new();
        test_graph.add_edge(1, 2);
        test_graph.add_edge(1, 3);
        test_graph.add_edge(2, 4);
        test_graph.add_edge(4, 5);

        let degrees = bfs_degrees_of_separation(&test_graph, 1);

        // Assert that the degrees of separation are correct
        assert_eq!(degrees.get(&1), Some(&0));
        assert_eq!(degrees.get(&2), Some(&1));
        assert_eq!(degrees.get(&3), Some(&1));
        assert_eq!(degrees.get(&4), Some(&2));
        assert_eq!(degrees.get(&5), Some(&3));
    }

    #[test]
    fn test_find_node_with_most_followers() {
        // Create a test graph with known nodes and edges
        let mut test_graph = Graph::new();
        test_graph.add_edge(1, 2);
        test_graph.add_edge(1, 3);
        test_graph.add_edge(2, 3);
        test_graph.add_edge(4, 5);

        // Call the function to find the node with the most followers
        let (max_degree_node, max_degree) = find_node_with_most_followers(&test_graph);

        assert_eq!(max_degree_node, 1); // Node 1 has the most followers
        assert_eq!(max_degree, 2);     // Node 1 has 2 followers
    }
}


