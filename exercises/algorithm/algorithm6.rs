// 📖 讲解：algorithm6 - dfs（深度优先搜索）
// 【题目要求】补全 dfs_util：对无向图从起点做递归 DFS，把访问顺序记录进 visit_order（HashSet 记录已访问）。
// 【考察知识点】DFS 递归模板（标记→记录→递归邻居）；HashSet 判重防环；
//             邻接表按插入顺序遍历（测试期望 [0,1,2,3] 这类"先走第一个邻居"的顺序）。
// 【对应教材】数据结构与算法——图的遍历（DFS/BFS）章节。
// 【解法思路】进入 dfs_util 先用 visited.insert(v) 的返回值判断：false 说明已访问过，直接返回（防环）；
//             否则记录 v 到 visit_order，再按邻接表顺序对每个未访问邻居递归。
//             insert 返回 bool 是 Rust 里很地道的"插入并查重"用法，省一次 contains。
//             时间 O(V+E)，空间 O(V)（递归栈 + visited）。
//             易错点：1) 自环边（3-3）和回边不会造成死循环，全靠 insert 判重；
//                    2) 非连通图只访问起点所在连通分量（测试正是这么断言的）。

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
        // 💡 insert 返回 false 表示 v 已在集合中（已访问过）——防环 / 防自环的关键
        if !visited.insert(v) {
            return;
        }
        // 💡 首次到达：记录访问顺序
        visit_order.push(v);
        // 💡 按邻接表（即 add_edge 的插入）顺序递归访问未访问的邻居 → 深度优先
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
        assert_eq!(visit_order, vec![0, 1, 2]);
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
        assert_eq!(visit_order, vec![0, 1, 2, 3]);
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
