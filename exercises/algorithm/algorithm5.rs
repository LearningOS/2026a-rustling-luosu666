// 📖 讲解：algorithm5 - bfs（广度优先搜索）
// 【题目要求】对无向图（邻接表 Vec<Vec<usize>>）从 start 出发做 BFS，返回访问顺序向量。
// 【考察知识点】BFS 的队列实现（std::collections::VecDeque）；
//             visited 数组防止重复访问（尤其有环时防死循环）；按入队顺序记录访问序。
// 【对应教材】数据结构与算法——图的遍历（BFS/DFS）章节。
// 【解法思路】标准模板：先把起点标记 visited 并入队；
//             每次出队一个节点记入 visit_order，再把"未访问过的邻居"逐个标记并入队。
//             关键细节：必须在【入队时】就标记 visited，而不是出队时，
//             否则同一节点可能被多个邻居重复入队（结果出现重复）。
//             时间 O(V+E)，空间 O(V)。
//             易错点：1) 期望顺序 [0,1,4,2,3] 取决于"按 add_edge 的插入顺序访问邻居"，
//                    Vec 邻接表天然保序，无需排序；
//                    2) 空图/单节点（无边）也要返回 [start]。

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

        let mut visit_order = vec![];

        // 💡 visited 数组：防止环导致重复访问/死循环
        let mut visited = vec![false; self.adj.len()];
        // 💡 BFS 用队列（VecDeque），先进先出保证"一层一层"扩展
        let mut queue = VecDeque::new();

        // 💡 起点在【入队时】就标记为已访问（出队才标记会导致重复入队）
        visited[start] = true;
        queue.push_back(start);

        while let Some(v) = queue.pop_front() {
            // 💡 出队即记录访问顺序
            visit_order.push(v);
            // 💡 邻接表按 add_edge 的插入顺序保存邻居，逐个检查（天然满足测试期望的顺序）
            for &neighbor in &self.adj[v] {
                if !visited[neighbor] {
                    visited[neighbor] = true; // 💡 标记 + 入队
                    queue.push_back(neighbor);
                }
            }
        }

        visit_order
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
