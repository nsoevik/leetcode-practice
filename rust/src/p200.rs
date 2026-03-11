use std::collections::{HashSet};

const NEXT_STEPS: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let mut visited = HashSet::new();
    let mut count = 0;

    fn is_valid(grid: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
        return grid.get(i).is_some_and(|row| row.get(j).is_some());
    }

    fn dfs_update(
        visited: &mut HashSet<(usize, usize)>,
        grid: &Vec<Vec<char>>,
        pair: (usize, usize),
    ) {
        visited.insert(pair);

        for step in NEXT_STEPS {
            let added_i = (pair.0 as i32 + step.0) as usize;
            let added_j = (pair.1 as i32 + step.1) as usize;
            if !is_valid(grid, added_i, added_j) || grid[added_i][added_j] == '0' {
                continue;
            }

            let new_pair = (added_i, added_j);
            if visited.contains(&new_pair) {
                continue
            }
            dfs_update(visited, grid, new_pair)
        }
    }

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let pair = (i, j);
            if !visited.contains(&pair) && grid[pair.0][pair.1] == '1' {
                count += 1;
                dfs_update(&mut visited, &grid, (i, j));
            }
        }
    }

    count
}
