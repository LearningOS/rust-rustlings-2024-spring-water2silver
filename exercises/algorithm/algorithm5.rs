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
        let mut visit_order = vec![];
        let mut vis = vec![0;self.adj.len()];
        visit_order.push(start);
        vis[start] = 1;
        let mut queue: VecDeque<usize> = VecDeque::new();
        queue.push_back(start);
        while queue.len()!=0
        {
            
            let point = queue.pop_front().unwrap();

            //self.adj is Vec<Vec<usize>>, 
            // for i in &self.adj[point]
            // {
            //     if vis[*i]==0
            //     {
            //         vis[*i] = 1;
            //         queue.push_back(i.clone());
            //         visit_order.push(i.clone());
            //     }
            // }

            //GPT的解法

            // 假设 vis 是一个 Vec<usize>
            // 假设 queue 和 visit_order 是 Vec<usize> 类型的队列和访问顺序列表
            // 遍历 self.adj[point] 中的每个元素
            for &i in &self.adj[point] {
            // 检查是否已经访问过
                if vis[i] == 0 {
                    // 标记为已访问
                    vis[i] = 1;
                    
                    // 将未访问的节点添加到队列和访问顺序列表中
                    queue.push_back(i);
                    visit_order.push(i);
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

