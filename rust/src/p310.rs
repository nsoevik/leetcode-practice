use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n == 1 {
            return vec![0];
        }
        let mut degrees = vec![0; n as usize];
        let mut neighbors: HashMap<usize, Vec<usize>> = HashMap::new();
        for edge in edges {
            degrees[edge[0] as usize] += 1;
            degrees[edge[1] as usize] += 1;
            neighbors
                .entry(edge[0] as usize)
                .or_insert(vec![])
                .push(edge[1] as usize);
            neighbors
                .entry(edge[1] as usize)
                .or_insert(vec![])
                .push(edge[0] as usize);
        }

        let mut dq = VecDeque::<usize>::new();
        for (i, d) in degrees.iter().enumerate() {
            if *d == 1 {
                dq.push_back(i);
            }
        }

        let mut ans = vec![];
        while !dq.is_empty() {
            ans.clear();
            for _ in 0..dq.len() {
                let curr_edge = dq.pop_front().unwrap();
                ans.push(curr_edge.clone() as i32);
                for neighbor in neighbors.get(&curr_edge).unwrap() {
                    degrees[*neighbor] -= 1;
                    if degrees[*neighbor] == 1 {
                        dq.push_back(*neighbor);
                    }
                }
            }
        }

        ans
    }
}

struct Solution;
