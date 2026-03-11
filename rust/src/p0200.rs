pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut count = 0;

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '1' {
                count += 1;
                dfs(&mut grid, r, c, rows, cols);
            }
        }
    }
    count
}

fn dfs(grid: &mut Vec<Vec<char>>, r: usize, c: usize, rows: usize, cols: usize) {
    if r >= rows || c >= cols || grid[r][c] != '1' {
        return;
    }
    grid[r][c] = '0';
    if r > 0 { dfs(grid, r - 1, c, rows, cols); }
    dfs(grid, r + 1, c, rows, cols);
    if c > 0 { dfs(grid, r, c - 1, rows, cols); }
    dfs(grid, r, c + 1, rows, cols);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_island() {
        let grid = vec![
            vec!['1', '1', '0'],
            vec!['0', '1', '0'],
            vec!['0', '0', '0'],
        ];
        assert_eq!(num_islands(grid), 1);
    }

    #[test]
    fn test_multiple_islands() {
        let grid = vec![
            vec!['1', '0', '1'],
            vec!['0', '0', '0'],
            vec!['1', '0', '1'],
        ];
        assert_eq!(num_islands(grid), 4);
    }
}
