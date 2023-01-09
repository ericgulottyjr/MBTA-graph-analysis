use std::f32;

pub fn depth_first_search(adjacency_matrix: &Vec<Vec<f32>>, start: usize, end: usize) -> Vec<usize> {
    let mut path = Vec::new();
    let mut visited = vec![false; adjacency_matrix.len()];

    fn dfs(adjacency_matrix: &Vec<Vec<f32>>, start: usize, end: usize, visited: &mut Vec<bool>, path: &mut Vec<usize>) -> bool {
        visited[start] = true;
        path.push(start);

        if start == end {
            return true;
        }

        for i in 0..adjacency_matrix.len() {
            if !visited[i] && adjacency_matrix[start][i] != 0.0 {
                if dfs(adjacency_matrix, i, end, visited, path) {
                    return true;
                }
            }
        }
        path.pop();
        false
    }
    dfs(adjacency_matrix, start, end, &mut visited, &mut path);
    path
}
