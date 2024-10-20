/*
	dfs
	This problem requires you to implement a basic DFS traversal
*/



use std::collections::HashSet;  
  
struct Graph {  
    adj: Vec<Vec<usize>>,  
}  
  
impl Graph {  
    fn new(n: usize) -> Self {  
        Graph {  
            adj: vec![vec![]; n],  
        }  
    }  
  
    fn add_edge(&mut self, src: usize, dest: usize) {  
        self.adj[src].push(dest);  
        self.adj[dest].push(src);  
    }  
  
    fn dfs_util(&self, v: usize, visited: &mut HashSet<usize>, visit_order: &mut Vec<usize>) {  
        visited.insert(v);  
        visit_order.push(v);  
  
        for &neighbor in &self.adj[v] {  
            if !visited.contains(&neighbor) {  
                self.dfs_util(neighbor, visited, visit_order);  
            }  
        }  
    }  
  
    // Perform a depth-first search on the graph, return the order of visited nodes  
    fn dfs(&self, start: usize) -> Vec<usize> {  
        let mut visited = HashSet::new();  
        let mut visit_order = Vec::new();  
        self.dfs_util(start, &mut visited, &mut visit_order);  
        visit_order  
    }  
}  
  
#[cfg(test)]  
mod tests {  
    use super::*;  
  
    #[test]  
    fn test_dfs_simple() {  
        let mut graph = Graph::new(3);  
        graph.add_edge(0, 1);  
        graph.add_edge(1, 2);  
  
        let visit_order = graph.dfs(0);  
        // Note: DFS visit order can vary due to the non-deterministic nature of HashSet iteration  
        // However, for this simple graph, one possible correct order is [0, 1, 2]  
        // Another possible order is [0, 2, 1] if the adjacency list for 0 is iterated in reverse  
        // We will assert for one of these possible orders  
        let expected_orders = vec![vec![0, 1, 2], vec![0, 2, 1]];  
        assert!(expected_orders.contains(&visit_order));  
    }  
  
    #[test]  
    fn test_dfs_with_cycle() {  
        let mut graph = Graph::new(4);  
        graph.add_edge(0, 1);  
        graph.add_edge(0, 2);  
        graph.add_edge(1, 2);  
        graph.add_edge(2, 3);  
        graph.add_edge(3, 3);  
  
        let visit_order = graph.dfs(0);  
        // Similar to the simple graph test, DFS visit order can vary  
        // One possible correct order is [0, 1, 2, 3]  
        // Another possible order could include revisiting 3 after visiting it initially due to the cycle  
        // However, since we use a HashSet to track visited nodes, revisits are ignored  
        let expected_orders = vec![vec![0, 1, 2, 3], vec![0, 2, 1, 3]];  
        assert!(expected_orders.contains(&visit_order));  
    }  
  
    #[test]  
    fn test_dfs_disconnected_graph() {  
        let mut graph = Graph::new(5);  
        graph.add_edge(0, 1);  
        graph.add_edge(0, 2);  
        graph.add_edge(3, 4);  
  
        let visit_order = graph.dfs(0);  
        assert_eq!(visit_order, vec![0, 1, 2]);  
        let visit_order_disconnected = graph.dfs(3);  
        assert_eq!(visit_order_disconnected, vec![3, 4]);  
    }  
}
