use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = mat.clone();
        let mut dq = VecDeque::new();

        for i in 0..mat.len() {
            for j in 0..mat[i].len() {
                if mat[i][j] == 0 {
                    dq.push_back((i as i32, j as i32, 0));
                }
            }
        }

        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let directions: Vec<(i32, i32)> = vec![(0, -1),(0, 1),(-1, 0), (1, 0)];
        while !dq.is_empty() {
            let (i, j, d) = dq.pop_front().unwrap();
            if visited.contains(&(i, j)) {
                continue;
            }

            if let Some(row) = ans.get(i as usize) {
                if let Some(_) = row.get(j as usize) {
                    ans[i as usize][j as usize] = d;
                    visited.insert((i, j));
                    for dir in directions.iter() {
                        dq.push_back((i + dir.0, j + dir.1, d + 1));
                    }
                }
            }
        }

        ans
    }
}

struct Solution;
