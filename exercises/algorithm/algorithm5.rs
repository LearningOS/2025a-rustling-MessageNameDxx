/*
    bfs
    This problem requires you to implement a basic BFS algorithm
*/


use std::collections::VecDeque;

// Define a graph
struct Graph {
    adj: Vec<Vec<usize>>,
}

impl Graph {
    // Create a new graph with n vertices
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // Add an edge to the graph
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest);
        self.adj[dest].push(src);
    }

    // Perform a breadth-first search on the graph, return the order of visited nodes
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        //TODO

        // 1. 初始化访问标记：记录每个节点是否已访问
        // 长度等于节点数，初始全为false
        let mut visited = vec![false; self.adj.len()];

        // 2. 初始化队列：存储待访问的节点（BFS的核心数据结构）
        let mut queue = VecDeque::new();

        // 3. 初始化访问顺序列表
        let mut visit_order = vec![];

        // 4. 启动BFS：处理起始节点
        visited[start] = true; // 标记起始节点为已访问
        queue.push_back(start); // 将起始节点加入队列

        // 5. BFS主循环：队列非空时持续处理
        // 从队列前端取出节点（pop_front），符合"先进先出"特性
        while let Some(node) = queue.pop_front() {
            // 将当前节点加入访问顺序
            visit_order.push(node);

            // 遍历当前节点的所有邻居
            for &neighbor in &self.adj[node] {
                // 如果邻居未被访问
                if !visited[neighbor] {
                    visited[neighbor] = true; // 标记为已访问
                    queue.push_back(neighbor); // 加入队列等待访问
                }
            }
        }

        visit_order // 返回访问顺序
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}
